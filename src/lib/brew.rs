use std::{env, io, path::PathBuf, str};

use dirs;
use which::{self, which_in};

use crate::utils::process::{command_output, command_spawn_wait};

const INSTALL_DIRS: &[&str] = &["/usr/local", "/home/linuxbrew/.linuxbrew", "~/.linuxbrew"];

pub fn has_brew() -> bool {
    match brew_exe() {
        Some(exe) => match command_output(exe, &["--version"]) {
            Ok(_) => true,
            Err(_) => false,
        },
        None => false,
    }
}

pub fn brew<S>(args: &[S]) -> io::Result<()>
where
    S: Into<String> + AsRef<str>,
{
    match brew_exe() {
        Some(exe) => command_spawn_wait(exe, args).map(|_| ()),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "brew")),
    }
}

pub fn brew_output<S>(args: &[S]) -> io::Result<String>
where
    S: Into<String> + AsRef<str>,
{
    match brew_exe() {
        Some(exe) => {
            let output = command_output(exe, args)?;
            Ok(format!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout).trim(),
                String::from_utf8_lossy(&output.stderr).trim(),
            )
            .to_string())
        }
        None => Err(io::Error::new(io::ErrorKind::NotFound, "brew")),
    }
}

pub fn brew_prefix() -> Option<PathBuf> {
    let home_dir = dirs::home_dir().expect("brew: no $HOME");
    for prefix in INSTALL_DIRS {
        let dir = PathBuf::from(if prefix.starts_with("~/") {
            prefix.replace("~", &home_dir.to_string_lossy())
        } else {
            prefix.to_string()
        });
        match which_in(
            "brew",
            Some(dir.join("bin")),
            env::current_dir().expect("brew: no $PWD"),
        ) {
            Ok(_) => {
                return Some(dir);
            }
            Err(_) => {}
        }
    }
    None
}

fn brew_exe() -> Option<PathBuf> {
    match brew_prefix() {
        Some(p) => Some(p.join("bin").join("brew")),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brew_exe_does_not_panic() {
        brew_exe();
    }
}
