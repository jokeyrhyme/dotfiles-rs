use std::{fmt::Display, path::PathBuf};

use textwrap;
use which;

use crate::lib::ssh::{consts, fields::RootField};

fn format<D, S>(key: S, value: &Option<D>) -> String
where
    D: Display,
    S: Into<String> + AsRef<str>,
{
    match value {
        Some(v) => format!("{} {}\n", key.as_ref(), v),
        None => String::from(""),
    }
}

fn format_pathbuf(key: &str, value: &Option<PathBuf>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v.display()),
        None => String::from(""),
    }
}

fn format_strings(key: &str, value: &Option<Vec<String>>) -> String {
    match value {
        Some(v) => format!("{} {}\n", key, v.join(",")),
        None => String::from(""),
    }
}

pub fn has_ssh() -> bool {
    which::which("ssh").is_ok()
}

/*
#[cfg(test)]
mod tests {
    use std::{env::consts::OS, fs, path::Path};

    use super::*;

    #[test]
    fn bitor_merges_configs() {
        let mut c1 = Config::new();
        c1.AddKeysToAgent = Some(String::from("confirm"));
        c1.ConnectTimeout = Some(30);
        let mut c2 = Config::new();
        c2.ConnectTimeout = Some(10);

        let c3 = c1 | c2;

        assert_eq!(c3.AddKeysToAgent, Some(String::from("confirm")));
        assert_eq!(c3.ConnectTimeout, Some(10));
    }

    #[test]
    fn config_from_string() {
        let config_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ssh_config.input.txt");

        let got = fs::read_to_string(&config_path).unwrap();
        let want = example_config();

        let config = Config::from(got.as_str());

        assert_eq!(config.Hosts, want.Hosts);
        assert_eq!(config.Matches, want.Matches);
        assert_eq!(config, want);
    }

    fn example_config() -> Config {
        let mut cfg = Config::new();
        cfg.AddKeysToAgent = Some(String::from("confirm"));
        cfg.ConnectTimeout = Some(10);
        cfg.VisualHostKey = Some(YesNo::Yes);

        let mut h1 = Config::new();
        h1.Ciphers = Some(vec![
            String::from("aes128-ctr"),
            String::from("aes192-ctr"),
            String::from("aes256-ctr"),
        ]);
        h1.ControlMaster = Some(YesNoAskAutoAutoAsk::Auto);
        h1.ControlPersist = Some(YesNoDuration::Duration(Duration::Time(String::from("30s"))));
        h1.Hostname = Some(String::from("foo.example"));
        cfg.Hosts.insert(String::from("foo"), h1);

        let mut h2 = Config::new();
        h2.Hostname = Some(String::from("bar.example"));
        h2.IdentityFile = Some(PathBuf::new().join("~").join(".ssh").join("id_rsa"));
        cfg.Hosts.insert(String::from("bar"), h2);

        let mut m = Config::new();
        m.EscapeChar = Some('%');
        m.StrictHostKeyChecking = Some(YesNoAsk::Ask);
        cfg.Matches.insert(String::from("exec true"), m);

        cfg
    }

    #[test]
    fn string_from_config() {
        let config_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ssh_config.output.txt");

        let want = if OS == "windows" {
            fs::read_to_string(&config_path)
                .unwrap()
                .replace("IdentityFile ~/.ssh/id_rsa", "IdentityFile ~\\.ssh\\id_rsa")
        } else {
            fs::read_to_string(&config_path).unwrap()
        };
        let config = example_config();

        assert_eq!(String::from(&config), want);
    }
}
*/
