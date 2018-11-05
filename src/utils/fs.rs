use std;
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fmt::Debug,
    io,
    path::{Path, PathBuf},
};

use mktemp;

use lib::task::{self, Status};

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn delete_if_exists<P>(path: P)
where
    P: Into<PathBuf> + AsRef<Path> + Debug,
{
    let attr = match std::fs::symlink_metadata(path.as_ref()) {
        Ok(attr) => attr,
        Err(_error) => {
            // might not exist, noop
            return;
        }
    };

    if attr.is_dir() {
        match std::fs::remove_dir_all(path.as_ref()) {
            Ok(_removed) => {
                println!("deleted {}", path.as_ref().display());
                return;
            }
            Err(error) => {
                println!(
                    "unable to recursively delete directory {}: {:?}",
                    path.as_ref().display(),
                    error
                );
                return;
            }
        }
    }

    match std::fs::remove_file(path.as_ref()) {
        Ok(_removed) => {
            println!("deleted {:?}", path.as_ref());
            return;
        }
        Err(error) => {
            println!(
                "unable to delete file {}: {:?}",
                path.as_ref().display(),
                error
            );
            return;
        }
    }
}

#[cfg(unix)]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn set_executable<P>(target: P) -> std::io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path>,
{
    let file = File::open(target).unwrap();
    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o755); // a+rx, u+w
    file.set_permissions(perms)
}

#[cfg(not(unix))]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn set_executable<P>(_target: P) -> std::io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path> + PartialEq,
{
    Ok(())
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn symbolic_link_if_exists<P>(src: P, dest: P) -> task::Result
where
    P: Into<PathBuf> + AsRef<Path> + Debug,
{
    match std::fs::read_link(dest.as_ref()) {
        Ok(target) => {
            if src.as_ref() == target {
                return Ok(Status::NoChange(format!(
                    "already symlinked: {} -> {}",
                    dest.as_ref().display(),
                    target.display(),
                )));
            }
        }
        Err(_error) => {
            // does not exist, or not a symlink
        }
    };

    if std::fs::symlink_metadata(src.as_ref()).is_err() {
        return Ok(Status::Skipped);
    }

    if let Some(parent) = dest.as_ref().parent() {
        std::fs::create_dir_all(&parent)?;
    }

    match std::fs::symlink_metadata(dest.as_ref()) {
        Ok(_attr) => {
            delete_if_exists(dest.as_ref());
        }
        Err(_error) => { /* might not exist, continue */ }
    }

    symbolic_link(src.as_ref(), dest.as_ref())?;

    Ok(Status::Changed(
        "previous symlink".to_string(),
        format!(
            "symlinked: {} -> {}",
            dest.as_ref().display(),
            src.as_ref().display(),
        ),
    ))
}

#[cfg(not(windows))]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn symbolic_link<P>(src: P, dest: P) -> io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path>,
{
    std::os::unix::fs::symlink(src, dest)
}

#[cfg(windows)]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn symbolic_link<P>(src: P, dest: P) -> io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path>,
{
    let src_attr = std::fs::symlink_metadata(&src)?;
    if src_attr.is_dir() {
        return std::os::windows::fs::symlink_dir(&src, dest);
    }

    std::os::windows::fs::symlink_file(&src, dest)
}

pub fn mkdtemp() -> io::Result<PathBuf> {
    let temp_path;
    {
        let mut temp = mktemp::Temp::new_dir()?;
        temp_path = temp.to_path_buf();
        temp.release();
    }
    Ok(temp_path)
}

pub fn mktemp() -> io::Result<PathBuf> {
    let temp_path;
    {
        let mut temp = mktemp::Temp::new_file()?;
        temp_path = temp.to_path_buf();
        temp.release();
    }
    Ok(temp_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cargo() {
        let file_path = Path::new(env!("CARGO"));
        assert!(!file_path.is_dir());
    }

    #[test]
    fn check_cargo_manifest_dir() {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(project_dir.is_dir());
    }
}
