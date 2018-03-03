use std::io::Error;
use std::process::Command;
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
                println!("command_output_cargo_version: {:?}", error);
                assert!(false);
            }
        }
    }
}