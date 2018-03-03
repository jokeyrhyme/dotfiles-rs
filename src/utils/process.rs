use std::io::Error;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;
use std::str;

#[cfg(not(windows))]
pub fn command_output<'a>(cmd: &str, args: &[&str]) -> Result<Output, Error> {
    return Command::new(cmd).args(args).output();
}

#[cfg(windows)]
pub fn command_output<'a>(cmd: &'a str, args: &[&str]) -> Result<Output, Error> {
    let mut cmdArgs = Vec::<&str>::new();
    cmdArgs.push("/c");
    cmdArgs.push(cmd);
    cmdArgs.extend(args);
    return Command::new("cmd").args(cmdArgs).output();
}

#[cfg(not(windows))]
pub fn command_spawn_wait<'a>(cmd: &str, args: &[&str]) -> Result<ExitStatus, Error> {
    return Command::new(cmd).args(args).spawn()?.wait();
}

#[cfg(windows)]
pub fn command_spawn_wait<'a>(cmd: &'a str, args: &[&str]) -> Result<ExitStatus, Error> {
    let mut cmdArgs = Vec::<&str>::new();
    cmdArgs.push("/c");
    cmdArgs.push(cmd);
    cmdArgs.extend(args);
    return Command::new("cmd").args(cmdArgs).spawn()?.wait();
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
        match command_spawn_wait("does_not_exist", &[]) {
            Ok(status) => {
                assert!(!status.success());
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }
}