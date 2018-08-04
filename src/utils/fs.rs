use std;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::{fmt::Debug, fs::File, io};

use mktemp;

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

pub fn is_dir<P>(target: P) -> bool
where
    P: Into<PathBuf> + AsRef<Path>,
{
    match std::fs::metadata(target) {
        Ok(m) => m.is_dir(),
        Err(_error) => false,
    }
}

#[cfg(unix)]
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
pub fn set_executable<P>(_target: P) -> std::io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path> + PartialEq,
{
    Ok(())
}

pub fn symbolic_link_if_exists<P>(src: P, dest: P)
where
    P: Into<PathBuf> + AsRef<Path> + Debug,
{
    match std::fs::read_link(dest.as_ref()) {
        Ok(target) => {
            if src.as_ref() == target {
                println!(
                    "already symlinked: {} -> {}",
                    dest.as_ref().display(),
                    target.display(),
                );
                return;
            }
        }
        Err(_error) => {
            // does not exist, or not a symlink
        }
    };

    match std::fs::symlink_metadata(src.as_ref()) {
        Ok(attr) => attr,
        Err(error) => {
            println!("unable to access {}: {:?}", src.as_ref().display(), error);
            return;
        }
    };

    if let Some(parent) = dest.as_ref().parent() {
        std::fs::create_dir_all(&parent).unwrap_or_else(|_| {
            println!("unable to create directories {}", parent.display());
        });
    };

    match std::fs::symlink_metadata(dest.as_ref()) {
        Ok(_attr) => {
            delete_if_exists(dest.as_ref());
        }
        Err(_error) => { /* might not exist, continue */ }
    }

    match symbolic_link(src.as_ref(), dest.as_ref()) {
        Ok(()) => {
            println!(
                "symlinked: {} -> {}",
                dest.as_ref().display(),
                src.as_ref().display(),
            );
        }
        Err(error) => {
            println!(
                "unable to symlink {} to {}: {:?}",
                dest.as_ref().display(),
                src.as_ref().display(),
                error
            );
            return;
        }
    };
}

#[cfg(not(windows))]
fn symbolic_link<P>(src: P, dest: P) -> io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path>,
{
    std::os::unix::fs::symlink(src, dest)
}

#[cfg(windows)]
fn symbolic_link<P>(src: P, dest: P) -> io::Result<()>
where
    P: Into<PathBuf> + AsRef<Path>,
{
    let src_attr = std::fs::symlink_metadata(src)?;
    if src_attr.is_dir() {
        return std::os::windows::fs::symlink_dir(src, dest);
    }

    std::os::windows::fs::symlink_file(src, dest)
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
        assert!(!is_dir(&file_path));
    }

    #[test]
    fn check_cargo_manifest_dir() {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(is_dir(&project_dir));
    }
}
