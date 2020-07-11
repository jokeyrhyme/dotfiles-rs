use std::{
    env::{join_paths, split_paths, var_os},
    ffi::OsStr,
    fmt::Display,
    path::PathBuf,
};

const DEFAULT_INFO_PATHS: &[&str] = &["/usr/share/info", "/usr/local/share/info"];
const DEFAULT_MAN_PATHS: &[&str] = &["/usr/share/man", "/usr/local/share/man"];
const DEFAULT_PATHS: &[&str] = &["/bin", "/usr/bin", "/usr/local/bin"];

const EDITOR: &str = "EDITOR";
const GOPATH: &str = "GOPATH";
const GOROOT: &str = "GOROOT";
const INFOPATH: &str = "INFOPATH";
const MANPATH: &str = "MANPATH";
const PATH: &str = "PATH";

#[derive(Debug)]
pub struct Exports {
    pub editor: PathBuf,
    pub gopath: Option<PathBuf>,
    pub goroot: Option<PathBuf>,
    pub info_path: Vec<PathBuf>,
    pub man_path: Vec<PathBuf>,
    pub path: Vec<PathBuf>,
}

impl Exports {
    pub fn new() -> Exports {
        Exports {
            editor: PathBuf::new(),
            gopath: None,
            goroot: None,
            info_path: Vec::<PathBuf>::new(),
            man_path: Vec::<PathBuf>::new(),
            path: Vec::<PathBuf>::new(),
        }
    }

    pub fn to_shell(&self, shell: Shell) -> String {
        let editor_line = export_shell(
            &shell,
            EDITOR,
            &self.editor.as_path().to_string_lossy().into_owned(),
        );

        let mut lines = vec![editor_line];

        if let Some(gopath) = &self.gopath {
            lines.push(export_shell(
                &shell,
                GOPATH,
                &gopath.as_path().to_string_lossy().into_owned(),
            ));
        }

        if let Some(goroot) = &self.goroot {
            lines.push(export_shell(
                &shell,
                GOROOT,
                &goroot.as_path().to_string_lossy().into_owned(),
            ));
        }

        if let Ok(paths) = join_paths(&self.info_path) {
            lines.push(export_shell(
                &shell,
                INFOPATH,
                &paths.to_string_lossy().into_owned(),
            ));
        }

        if let Ok(paths) = join_paths(&self.man_path) {
            lines.push(export_shell(
                &shell,
                MANPATH,
                &paths.to_string_lossy().into_owned(),
            ));
        }

        if let Ok(paths) = join_paths(&self.path) {
            lines.push(export_shell(
                &shell,
                PATH,
                &paths.to_string_lossy().into_owned(),
            ));
        }

        lines.join("\n")
    }
}

impl Default for Exports {
    fn default() -> Exports {
        let mut exports = Exports {
            info_path: var_split_paths(INFOPATH),
            man_path: var_split_paths(MANPATH),
            path: var_split_paths(PATH),
            ..Exports::new()
        };

        for path in DEFAULT_INFO_PATHS {
            let pb = PathBuf::from(path);
            if pb.is_dir() && !exports.info_path.contains(&pb) {
                exports.info_path.push(pb); // TODO: de-dupe
            }
        }
        for path in DEFAULT_MAN_PATHS {
            let pb = PathBuf::from(path);
            if pb.is_dir() && !exports.man_path.contains(&pb) {
                exports.man_path.push(pb); // TODO: de-dupe
            }
        }
        for path in DEFAULT_PATHS {
            let pb = PathBuf::from(path);
            if pb.is_dir() && !exports.path.contains(&pb) {
                exports.path.push(pb); // TODO: de-dupe
            }
        }

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
    S: AsRef<str> + Display,
{
    format!("export {}={}", key, value)
}

fn export_fish<S>(key: S, value: S) -> String
where
    S: AsRef<str> + Display,
{
    format!("set --export {} {}", key, value)
}

fn export_shell<S>(shell: &Shell, key: S, value: S) -> String
where
    S: AsRef<str> + Display,
{
    if value.as_ref().trim().is_empty() {
        return String::new();
    }
    match shell {
        Shell::Bash | Shell::Zsh => export_bash(key, value),
        Shell::Fish => export_fish(key, value),
    }
}

fn var_split_paths<O>(name: O) -> Vec<PathBuf>
where
    O: AsRef<OsStr>,
{
    match var_os(name) {
        Some(value) => split_paths(&value).collect(),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use std::env::consts::OS;

    use super::*;

    #[test]
    fn to_bash() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            path: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
            ..Exports::new()
        };
        let got = exports.to_shell(Shell::Bash);
        let want = if OS == "windows" {
            "export EDITOR=/usr/bin/vim\n\n\nexport PATH=/usr/bin;/bin"
        } else {
            "export EDITOR=/usr/bin/vim\n\n\nexport PATH=/usr/bin:/bin"
        };
        assert_eq!(got, want);
    }

    #[test]
    fn to_fish() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            path: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
            ..Exports::new()
        };
        let got = exports.to_shell(Shell::Fish);
        let want = if OS == "windows" {
            "set --export EDITOR /usr/bin/vim\n\n\nset --export PATH /usr/bin;/bin"
        } else {
            "set --export EDITOR /usr/bin/vim\n\n\nset --export PATH /usr/bin:/bin"
        };
        assert_eq!(got, want);
    }

    #[test]
    fn to_zsh() {
        let exports = Exports {
            editor: PathBuf::from("/usr/bin/vim"),
            path: vec![PathBuf::from("/usr/bin"), PathBuf::from("/bin")],
            ..Exports::new()
        };
        let got = exports.to_shell(Shell::Zsh);
        let want = if OS == "windows" {
            "export EDITOR=/usr/bin/vim\n\n\nexport PATH=/usr/bin;/bin"
        } else {
            "export EDITOR=/usr/bin/vim\n\n\nexport PATH=/usr/bin:/bin"
        };
        assert_eq!(got, want);
    }
}
