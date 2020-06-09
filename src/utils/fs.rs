use std::{
    self,
    fmt::Debug,
    io,
    path::{Path, PathBuf},
};
#[cfg(unix)]
use std::{fs::File, os::unix::fs::PermissionsExt};

use crate::lib::task::{self, Status};

pub fn delete_if_exists<P>(path: P)
where
    P: AsRef<Path> + Debug,
{
    let p = path.as_ref();
    let attr = match std::fs::symlink_metadata(&p) {
        Ok(attr) => attr,
        Err(_error) => {
            // might not exist, noop
            return;
        }
    };

    if attr.is_dir() {
        match std::fs::remove_dir_all(&p) {
            Ok(_removed) => {}
            Err(error) => println!(
                "unable to recursively delete directory {}: {:?}",
                p.display(),
                error
            ),
        };
        return;
    }

    match std::fs::remove_file(&p) {
        Ok(_removed) => {}
        Err(error) => println!("unable to delete file {}: {:?}", p.display(), error),
    }
}

#[cfg(unix)]
pub fn set_executable<P>(target: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(target.as_ref()).unwrap();
    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o755); // a+rx, u+w
    file.set_permissions(perms)
}

#[cfg(not(unix))]
pub fn set_executable<P>(_target: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    Ok(())
}

pub fn symbolic_link_if_exists<P>(src: P, dest: P) -> task::Result
where
    P: AsRef<Path> + Debug,
{
    let d = dest.as_ref();
    let s = src.as_ref();
    match std::fs::read_link(&d) {
        Ok(target) => {
            if s == target {
                return Ok(Status::NoChange(format!(
                    "already symlinked: {} -> {}",
                    d.display(),
                    target.display(),
                )));
            }
        }
        Err(_error) => {
            // does not exist, or not a symlink
        }
    };

    if std::fs::symlink_metadata(&s).is_err() {
        return Ok(Status::Skipped);
    }

    if let Some(parent) = d.parent() {
        std::fs::create_dir_all(&parent)?;
    }

    match std::fs::symlink_metadata(&d) {
        Ok(_attr) => {
            delete_if_exists(&d);
        }
        Err(_error) => { /* might not exist, continue */ }
    }

    symbolic_link(&s, &d)?;

    Ok(Status::Changed(
        String::from("previous symlink"),
        format!("symlinked: {} -> {}", d.display(), s.display(),),
    ))
}

#[cfg(not(windows))]

fn symbolic_link<P>(src: P, dest: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    std::os::unix::fs::symlink(src.as_ref(), dest.as_ref())
}

#[cfg(windows)]

fn symbolic_link<P>(src: P, dest: P) -> io::Result<()>
where
    P: AsRef<Path>,
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
        let temp = mktemp::Temp::new_dir()?;
        temp_path = temp.to_path_buf();
        temp.release();
    }
    Ok(temp_path)
}

pub fn mkftemp() -> io::Result<PathBuf> {
    let temp_path;
    {
        let temp = mktemp::Temp::new_file()?;
        temp_path = temp.to_path_buf();
        temp.release();
    }
    Ok(temp_path)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

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
