use std::{collections::HashMap, fs, str};

use serde_derive::Deserialize;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

const COMMAND_DELIMITERS: &[char] = &[';', '|', '&'];
const ERROR_MSG: &str = "error: git";

pub fn task() -> Task {
    Task {
        name: String::from("git"),
        sync,
        ..Default::default()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct ComplexConfigEntry {
    value: String,
    #[serde(default)]
    when: String,
}
impl<S> From<S> for ComplexConfigEntry
where
    S: AsRef<str>,
{
    fn from(s: S) -> ComplexConfigEntry {
        ComplexConfigEntry {
            value: String::from(s.as_ref()),
            when: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    config: HashMap<String, ConfigEntry>,
}
impl Config {
    fn new() -> Config {
        Config {
            config: HashMap::<String, ConfigEntry>::new(),
        }
    }
}
impl<S> From<S> for Config
where
    S: AsRef<str>,
{
    fn from(s: S) -> Config {
        match toml::from_str(&s.as_ref()) {
            Ok(c) => c,
            Err(error) => {
                println!("warning: git: unable to parse TOML, {}", error);
                Config::new()
            }
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum ConfigEntry {
    Basic(String),
    Complex(ComplexConfigEntry),
}

fn extract_commands<S>(s: S) -> Vec<String>
where
    S: AsRef<str>,
{
    s.as_ref()
        .split(|c: char| COMMAND_DELIMITERS.contains(&c))
        .filter_map(|s| match s.trim().split(' ').next() {
            Some("") => None,
            Some(s) => Some(String::from(s)),
            None => None,
        })
        .collect()
}

fn sync() -> task::Result {
    if !utils::git::has() {
        return Ok(Status::Skipped);
    }

    // synchronise git settings
    let cfg = Config::from(load_config());
    for (key, ce) in cfg.config {
        let cce = match ce {
            ConfigEntry::Basic(s) => ComplexConfigEntry::from(s),
            ConfigEntry::Complex(c) => c,
        };
        match cce.when.as_str() {
            "unset" => {
                let current =
                    utils::process::command_output("git", &["config", "--global", "--get", &key])?;
                if str::from_utf8(&current.stdout)
                    .unwrap_or_default()
                    .trim()
                    .is_empty()
                {
                    utils::process::command_spawn_wait(
                        "git",
                        &["config", "--global", &key, &cce.value],
                    )?;
                }
            }
            "which" => {
                if extract_commands(cce.value.clone())
                    .iter()
                    .all(|v| which::which(v).is_ok())
                {
                    utils::process::command_spawn_wait(
                        "git",
                        &["config", "--global", &key, &cce.value],
                    )?;
                }
            }
            _ => {
                utils::process::command_spawn_wait(
                    "git",
                    &["config", "--global", &key, &cce.value],
                )?;
            }
        };
    }

    if !utils::nodejs::has_npx() {
        return Ok(Status::Skipped);
    }

    // https://www.npmjs.com/package/npm-merge-driver
    utils::process::command_spawn_wait("npx", &["-q", "npm-merge-driver", "install", "--global"])
        .expect(ERROR_MSG);

    if utils::nodejs::has_yarn() {
        // https://www.npmjs.com/package/npm-merge-driver
        utils::process::command_spawn_wait(
            "npx",
            &[
                "-q",
                "npm-merge-driver",
                "install",
                "--global",
                "--driver-name",
                "yarn-merge-driver",
                "--driver",
                "npx npm-merge-driver merge %A %O %B %P -c yarn",
                "--files",
                "yarn.lock",
            ],
        )
        .expect(ERROR_MSG);
    }

    Ok(Status::Done)
}

fn load_config() -> String {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/git.toml");

    match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("git: ignoring config: {}", error);
            String::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_commands_from_strings() {
        assert_eq!(extract_commands("gpg"), vec![String::from("gpg")]);
        assert_eq!(
            extract_commands("diff | less"),
            vec![String::from("diff"), String::from("less")]
        );
        assert_eq!(
            extract_commands("diff foo bar; less"),
            vec![String::from("diff"), String::from("less")]
        );
        assert_eq!(
            extract_commands("diff && less foo"),
            vec![String::from("diff"), String::from("less")]
        );
    }

    #[test]
    fn parse_config_toml() {
        let input = r#"
            [config]
            "color.ui" = "foo.bar"
            "gpg.program" = { value = "gpg", when = "which" }
            "#;
        let mut want = HashMap::<String, ConfigEntry>::new();
        want.insert(
            String::from("color.ui"),
            ConfigEntry::Basic(String::from("foo.bar")),
        );
        want.insert(
            String::from("gpg.program"),
            ConfigEntry::Complex(ComplexConfigEntry {
                value: String::from("gpg"),
                when: String::from("which"),
            }),
        );

        let cfg = Config::from(input);
        assert_eq!(want, cfg.config);
    }
}
