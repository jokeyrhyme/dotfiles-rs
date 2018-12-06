use std::{
    env::consts::OS,
    ffi::{OsStr, OsString},
    io,
    process::{Command, ExitStatus, Output},
    str,
};

use crate::tasks;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn command_output<O, S>(cmd: O, args: &[S]) -> io::Result<Output>
where
    O: Into<OsString> + AsRef<OsStr>,
    S: Into<String> + AsRef<str>,
{
    if OS == "windows" {
        let cmd_os = cmd.into();
        let cmd_lossy = cmd_os.to_string_lossy();
        let mut cmd_args: Vec<&str> = vec!["/c", cmd_lossy.as_ref()];
        cmd_args.extend::<Vec<&str>>(args.into_iter().map(|s| s.as_ref()).collect());
        with_env(Command::new("cmd").args(cmd_args)).output()
    } else {
        let cmd_args: Vec<&str> = args.into_iter().map(|s| s.as_ref()).collect();
        with_env(Command::new(cmd).args(cmd_args)).output()
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn command_spawn_wait<O, S>(cmd: O, args: &[S]) -> io::Result<ExitStatus>
where
    O: Into<OsString> + AsRef<OsStr>,
    S: Into<String> + AsRef<str>,
{
    if OS == "windows" {
        let cmd_os = cmd.into();
        let cmd_lossy = cmd_os.to_string_lossy();
        let mut cmd_args: Vec<&str> = vec!["/c", cmd_lossy.as_ref()];
        cmd_args.extend::<Vec<&str>>(args.into_iter().map(|s| s.as_ref()).collect());
        with_env(Command::new("cmd").args(cmd_args)).spawn()?.wait()
    } else {
        let cmd_args: Vec<&str> = args.into_iter().map(|s| s.as_ref()).collect();
        with_env(Command::new(cmd).args(cmd_args)).spawn()?.wait()
    }
}

fn with_env(c: &mut Command) -> &mut Command {
    let env = tasks::env();
    c.env("EDITOR", env.editor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_output_cargo_version() {
        match command_output("cargo", &["version"]) {
            Ok(output) => {
                assert!(output.status.success());

                let stderr = str::from_utf8(&output.stderr).unwrap();
                assert!(stderr.is_empty());

                let stdout = str::from_utf8(&output.stdout).unwrap();
                assert_eq!(&stdout[0..5], "cargo");
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn command_spawn_wait_cargo_version() {
        match command_spawn_wait("cargo", &["version"]) {
            Ok(status) => {
                assert!(status.success());
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn command_spawn_wait_does_not_exist() {
        match command_spawn_wait("does_not_exist", &["nope"]) {
            Ok(status) => {
                if OS == "windows" {
                    assert!(!status.success());
                } else {
                    assert!(false);
                }
            }
            Err(_error) => {
                if OS == "windows" {
                    assert!(false);
                } else {
                    assert!(true);
                }
            }
        }
    }
}
