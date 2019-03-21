use std::{
    env::{consts::OS, var},
    fmt::Display,
    path::PathBuf,
};

pub const EDITOR: &str = "EDITOR";
pub const GOPATH: &str = "GOPATH";
pub const GOROOT: &str = "GOROOT";
pub const PATH: &str = "PATH";

#[derive(Debug)]
pub struct Exports {
    pub editor: PathBuf,
    pub gopath: PathBuf,
    pub goroot: PathBuf,
    pub path: Vec<PathBuf>,
}

impl Exports {
    pub fn new() -> Exports {
        Exports {
            editor: PathBuf::new(),
            gopath: PathBuf::new(),
            goroot: PathBuf::new(),
            path: Vec::<PathBuf>::new(),
        }
    }

    pub fn to_shell(&self, shell: Shell) -> String {
        let editor_line = export_shell(
            &shell,
            EDITOR,
            &self.editor.as_path().to_string_lossy().into_owned(),
        );

        let gopath_line = export_shell(
            &shell,
            GOPATH,
            &self.gopath.as_path().to_string_lossy().into_owned(),
        );

        let goroot_line = export_shell(
            &shell,
            GOROOT,
            &self.goroot.as_path().to_string_lossy().into_owned(),
        );

        let path_strings: Vec<String> = self
            .path
            .clone()
            .into_iter()
            .map(|p| p.as_path().to_string_lossy().into_owned())
            .collect();
        let path_line = export_shell(
            &shell,
            PATH,
            &path_strings.join(if OS == "windows" { ";" } else { ":" }),
        );

        let lines: &[String] = &[editor_line, gopath_line, goroot_line, path_line];
        lines.join("\n")
    }
}

impl Default for Exports {
    fn default() -> Exports {
        let mut exports = Self::new();
        exports.path = match var("PATH") {
            Ok(paths) => {
                let path_strings = paths.split(if OS == "windows" { ";" } else { ":" });
                path_strings.map(|p| PathBuf::from(p)).collect()
            }
            Err(_) => exports.path,
        };
        exports
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

fn export_shell<S>(shell: &Shell, key: S, value: S) -> String
where
    S: Into<String> + AsRef<str> + Display,
{
    if value.as_ref().trim().is_empty() {
        return String::new();
    }
    match shell {
        Shell::Bash => export_bash(key, value),
        Shell::Fish => export_fish(key, value),
        Shell::Zsh => export_bash(key, value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bash() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            gopath: PathBuf::new(),
            goroot: PathBuf::new(),
            path: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
        };
        let got = exports.to_shell(Shell::Bash);
        let want = "export EDITOR=/usr/bin/vim\n\n\nexport PATH=/usr/bin:/bin";
        assert_eq!(got, want);
    }

    #[test]
    fn to_fish() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            gopath: PathBuf::new(),
            goroot: PathBuf::new(),
            path: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
        };
        let got = exports.to_shell(Shell::Fish);
        let want = "set --export EDITOR /usr/bin/vim\n\n\nset --export PATH /usr/bin:/bin";
        assert_eq!(got, want);
    }

    #[test]
    fn to_zsh() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            gopath: PathBuf::new(),
            goroot: PathBuf::new(),
            path: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
        };
        let got = exports.to_shell(Shell::Zsh);
        let want = "export EDITOR=/usr/bin/vim\n\n\nexport PATH=/usr/bin:/bin";
        assert_eq!(got, want);
    }
}
