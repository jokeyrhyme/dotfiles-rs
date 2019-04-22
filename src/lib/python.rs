use std::{ffi::OsStr, io, path::PathBuf};

use subprocess::{self, Exec, Redirection};
use which;

const VERSION_ARGS: &[&str] = &["--version"];

/*
pub fn has() -> bool {
    exe_output(VERSION_ARGS).is_ok()
}
*/

/*
pub fn python<O>(args: &[O]) -> subprocess::Result<()>
where
    O: AsRef<OsStr>,
{
    match which_v3() {
        // explicitly not using our own pre-computed env,
        // as it would stackoverflow
        Some(p) => {
            Exec::cmd(p).args(args).join()?;
            Ok(())
        }
        None => Err(io::Error::new(io::ErrorKind::NotFound, "python >=3.x").into()),
    }
}
*/

pub fn exe_output<O>(args: &[O]) -> subprocess::Result<String>
where
    O: AsRef<OsStr>,
{
    match which_v3() {
        // explicitly not using our own pre-computed env,
        // as it would stackoverflow
        Some(p) => Ok(Exec::cmd(p)
            .args(args)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()?
            .stdout_str()),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "python >=3.x").into()),
    }
}

pub fn which_v2() -> Option<PathBuf> {
    for python in &["python2", "python"] {
        let found = match which::which(&python) {
            Ok(exe) => exe,
            Err(_) => continue,
        };
        let version = match Exec::cmd(&found)
            .args(VERSION_ARGS)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
        {
            Ok(c) => extract_version(c.stdout_str()),
            Err(_error) => continue,
        };
        if version.starts_with("2.") {
            return Some(found);
        }
    }
    None
}

pub fn which_v3() -> Option<PathBuf> {
    for python in &["python", "python3"] {
        let found = match which::which(&python) {
            Ok(exe) => exe,
            Err(_) => continue,
        };
        let version = match Exec::cmd(&found)
            .args(VERSION_ARGS)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
        {
            Ok(c) => extract_version(c.stdout_str()),
            Err(_error) => continue,
        };
        if !version.starts_with("2.") {
            return Some(found);
        }
    }
    None
}

fn extract_version<S>(s: S) -> String
where
    S: AsRef<str>,
{
    let first = s.as_ref().lines().next().unwrap_or_default().trim();
    first.replace("Python ", "")
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn which_v2_does_not_find_v3() {
        let exe = match which_v2() {
            Some(e) => e,
            None => {
                // skip test if no `python` found
                return;
            }
        };
        let cap = Exec::cmd(exe)
            .args(VERSION_ARGS)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
            .expect("should execute");
        let re = Regex::new(&r"^2\.\d+\.\d+.*$").unwrap();
        assert!(re.is_match(&extract_version(cap.stdout_str())));
    }

    #[test]
    fn which_v3_is_different_to_which_v2() {
        if let (Some(v2), Some(v3)) = (which_v2(), which_v3()) {
            assert_ne!(v2, v3);
        }
    }
}
