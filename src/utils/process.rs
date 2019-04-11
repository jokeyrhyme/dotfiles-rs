use std::{
    env::consts::OS,
    ffi::OsStr,
    io,
    process::{Command, ExitStatus, Output},
    str,
};

use crate::tasks;

pub fn command_output<O, S>(cmd: O, args: &[S]) -> io::Result<Output>
where
    O: AsRef<OsStr>,
    S: AsRef<str>,
{
    let cmd_args = args.iter().map(|s| String::from(s.as_ref())).collect();
    if OS == "windows" {
        let cmd_os = cmd.as_ref();
        with_env(
            Command::new("cmd").args(
                [
                    vec![String::from("/c"), cmd_os.to_string_lossy().into_owned()],
                    cmd_args,
                ]
                .concat(),
            ),
        )
        .output()
    } else {
        with_env(Command::new(cmd.as_ref()).args(cmd_args)).output()
    }
}

pub fn command_spawn_wait<O, S>(cmd: O, args: &[S]) -> io::Result<ExitStatus>
where
    O: AsRef<OsStr>,
    S: AsRef<str>,
{
    let cmd_args = args.iter().map(|s| String::from(s.as_ref())).collect();
    if OS == "windows" {
        let cmd_os = cmd.as_ref();
        with_env(
            Command::new("cmd").args(
                [
                    vec![String::from("/c"), cmd_os.to_string_lossy().into_owned()],
                    cmd_args,
                ]
                .concat(),
            ),
        )
        .spawn()?
        .wait()
    } else {
        with_env(Command::new(cmd.as_ref()).args(cmd_args))
            .spawn()?
            .wait()
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
        let output = command_output("cargo", &["version"]).expect("must execute");
        assert!(output.status.success());

        let stderr = str::from_utf8(&output.stderr).unwrap();
        assert!(stderr.is_empty());

        let stdout = str::from_utf8(&output.stdout).unwrap();
        assert_eq!(&stdout[0..5], "cargo");
    }

    #[test]
    fn command_spawn_wait_cargo_version() {
        let status = command_spawn_wait("cargo", &["version"]).expect("must execute");
        assert!(status.success());
    }

    #[test]
    fn command_spawn_wait_does_not_exist() {
        match command_spawn_wait("does_not_exist", &["nope"]) {
            Ok(status) => {
                assert!(OS == "windows" && !status.success());
            }
            Err(_error) => assert!(OS != "windows"),
        }
    }
}
