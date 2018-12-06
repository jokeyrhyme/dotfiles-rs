use std::{env::consts::OS, io, path::PathBuf};

use crate::utils::{
    env::home_dir,
    process::{command_output, command_spawn_wait},
};

pub fn bin_dir() -> PathBuf {
    home_dir().join(".cargo").join("bin")
}

pub fn has_rustup() -> bool {
    match command_output(rustup_exe(), &["--version"]) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn rustup<S>(args: &[S]) -> io::Result<()>
where
    S: Into<String> + AsRef<str>,
{
    command_spawn_wait(rustup_exe(), args).map(|_| ())
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn rustup_output<S>(args: &[S]) -> io::Result<String>
where
    S: Into<String> + AsRef<str>,
{
    let output = command_output(rustup_exe(), args)?;
    Ok(format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim(),
    ).to_string())
}

pub fn rustup_version() -> String {
    match command_output(rustup_exe(), &["--version"]) {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

fn rustup_exe() -> PathBuf {
    bin_dir().join(if OS == "windows" {
        "rustup.exe"
    } else {
        "rustup"
    })
}
