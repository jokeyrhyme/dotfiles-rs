use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;
use std::{
    ffi::{OsStr, OsString},
    io, str,
};

#[cfg(not(windows))]
pub fn command_output<O, S>(cmd: O, args: &[S]) -> io::Result<Output>
where
    O: Into<OsString> + AsRef<OsStr>,
    S: Into<String> + AsRef<str>,
{
    let cmd_args: Vec<&str> = args.into_iter().map(|s| s.as_ref()).collect();
    Command::new(cmd).args(cmd_args).output()
}

#[cfg(windows)]
pub fn command_output<O, S>(cmd: O, args: &[S]) -> io::Result<Output>
where
    O: Into<OsString> + AsRef<OsStr>,
    S: Into<String> + AsRef<str>,
{
    let mut cmd_args = Vec::<&str>::new();
    cmd_args.push("/c");
    cmd_args.push(cmd);
    cmd_args.extend::<Vec<&str>>(args.into_iter().map(|s| s.as_ref()).collect());
    Command::new("cmd").args(cmd_args).output()
}

#[cfg(not(windows))]
pub fn command_spawn_wait<O, S>(cmd: O, args: &[S]) -> io::Result<ExitStatus>
where
    O: Into<OsString> + AsRef<OsStr>,
    S: Into<String> + AsRef<str>,
{
    let cmd_args: Vec<&str> = args.into_iter().map(|s| s.as_ref()).collect();
    Command::new(cmd).args(cmd_args).spawn()?.wait()
}

#[cfg(windows)]
pub fn command_spawn_wait<O, S>(cmd: O, args: &[S]) -> io::Result<ExitStatus>
where
    O: Into<OsString> + AsRef<OsStr>,
    S: Into<String> + AsRef<str>,
{
    let mut cmd_args = Vec::<&str>::new();
    cmd_args.push("/c");
    cmd_args.push(cmd);
    cmd_args.extend::<Vec<&str>>(args.into_iter().map(|s| s.as_ref()).collect());
    Command::new("cmd").args(cmd_args).spawn()?.wait()
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
            #[cfg(not(windows))]
            Ok(_status) => {
                assert!(false);
            }
            #[cfg(windows)]
            Ok(status) => {
                assert!(!status.success());
            }
            Err(_error) => {
                #[cfg(not(windows))]
                assert!(true);

                #[cfg(windows)]
                assert!(false);
            }
        }
    }
}
