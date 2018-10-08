use std::{fmt::Display, path::PathBuf};

pub const EDITOR: &'static str = "EDITOR";
//pub const PATH: &'static str = "PATH";

#[derive(Debug)]
pub struct Exports {
    pub editor: PathBuf,
    pub path: Vec<PathBuf>,
}

impl Exports {
    pub fn to_shell(&self, shell: Shell) -> String {
        let editor_line = export_shell(
            shell,
            EDITOR,
            self.editor
                .as_path()
                .to_string_lossy()
                .into_owned()
                .as_str(),
        );
        let lines: &[String] = &[editor_line];
        lines.join("\n")
    }
}

pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl<'a> From<&'a str> for Shell {
    fn from(source: &str) -> Self {
        match source {
            "fish" => Shell::Fish,
            "zsh" => Shell::Zsh,
            _ => Shell::Bash, // bash is the default
        }
    }
}

fn export_bash<S>(key: S, value: S) -> String
where
    S: Into<String> + AsRef<str> + Display,
{
    format!("export {}={}", key, value)
}

fn export_fish<S>(key: S, value: S) -> String
where
    S: Into<String> + AsRef<str> + Display,
{
    format!("set --export {} {}", key, value)
}

fn export_shell<S>(shell: Shell, key: S, value: S) -> String
where
    S: Into<String> + AsRef<str> + Display,
{
    match shell {
        Shell::Bash => export_bash(key, value),
        Shell::Fish => export_fish(key, value),
        Shell::Zsh => export_zsh(key, value),
    }
}

fn export_zsh<S>(key: S, value: S) -> String
where
    S: Into<String> + AsRef<str> + Display,
{
    export_bash(key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bash() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            path: Vec::<PathBuf>::new(),
        };
        let got = exports.to_shell(Shell::Bash);
        let want = "export EDITOR=/usr/bin/vim";
        assert_eq!(got, want);
    }

    #[test]
    fn to_fish() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            path: Vec::<PathBuf>::new(),
        };
        let got = exports.to_shell(Shell::Fish);
        let want = "set --export EDITOR /usr/bin/vim";
        assert_eq!(got, want);
    }

    #[test]
    fn to_zsh() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            path: Vec::<PathBuf>::new(),
        };
        let got = exports.to_shell(Shell::Zsh);
        let want = "export EDITOR=/usr/bin/vim";
        assert_eq!(got, want);
    }
}
