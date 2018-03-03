use std::io::Error;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;
use std::str;

#[cfg(not(windows))]
pub fn command_output<'a, T: AsRef<str>>(cmd: &str, args: &[T]) -> Result<Output, Error> {
    let cmd_args: Vec<&str> = args.into_iter().map(|s| s.as_ref()).collect();
    return Command::new(cmd).args(cmd_args).output();
}

#[cfg(windows)]
pub fn command_output<'a, T: AsRef<str>>(cmd: &str, args: &[T]) -> Result<Output, Error> {
    let mut cmd_args = Vec::<&str>::new();
    cmd_args.push("/c");
    cmd_args.push(cmd);
    cmd_args.extend::<Vec<&str>>(args.into_iter().map(|s| s.as_ref()).collect());
    return Command::new("cmd").args(cmd_args).output();
}

#[cfg(not(windows))]
pub fn command_spawn_wait<'a, T: AsRef<str>>(cmd: &str, args: &[T]) -> Result<ExitStatus, Error> {
    let cmd_args: Vec<&str> = args.into_iter().map(|s| s.as_ref()).collect();
    return Command::new(cmd).args(cmd_args).spawn()?.wait();
}

#[cfg(windows)]
pub fn command_spawn_wait<'a, T: AsRef<str>>(cmd: &str, args: &[T]) -> Result<ExitStatus, Error> {
    let mut cmd_args = Vec::<&str>::new();
    cmd_args.push("/c");
    cmd_args.push(cmd);
    cmd_args.extend::<Vec<&str>>(args.into_iter().map(|s| s.as_ref()).collect());
    return Command::new("cmd").args(cmd_args).spawn()?.wait();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_output_cargo_version () {
        match command_output("cargo", &["version"]) {
            Ok(output) => {
                assert!(output.status.success());

                let stderr = str::from_utf8(&output.stderr).unwrap();
                assert!(stderr.is_empty());

                let stdout = str::from_utf8(&output.stdout).unwrap();
                assert_eq!(&stdout[0..5], "cargo");
            }
            Err(error) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn command_spawn_wait_cargo_version () {
        match command_spawn_wait("cargo", &["version"]) {
            Ok(status) => {
                assert!(status.success());
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }

    fn command_spawn_wait_does_not_exist () {
        match command_spawn_wait("does_not_exist", &["nope"]) {
            Ok(status) => {
                assert!(!status.success());
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }
}