use std::{io, path::PathBuf};

use utils::{
    env::home_dir,
    process::{command_output, command_spawn_wait},
};

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

pub fn rustup_version() -> String {
    match command_output(rustup_exe(), &["--version"]) {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

fn rustup_exe() -> PathBuf {
    #[cfg(windows)]
    let pb = home_dir().join(".cargo\\bin\\rustup.exe");
    #[cfg(not(windows))]
    let pb = home_dir().join(".cargo/bin/rustup");

    pb
}
