use std;
use std::fs::File;
use std::io::Error;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn delete_if_exists(path: &Path) {
    let attr = match std::fs::symlink_metadata(path) {
        Ok(attr) => attr,
        Err(_error) => {
            // might not exist, noop
            return;
        }
    };

    if attr.is_dir() {
        match std::fs::remove_dir_all(path) {
            Ok(_removed) => {
                println!("deleted {}", path.display());
                return;
            }
            Err(error) => {
                println!(
                    "unable to recursively delete directory {}: {:?}",
                    path.display(),
                    error
                );
                return;
            }
        }
    }

    match std::fs::remove_file(path) {
        Ok(_removed) => {
            println!("deleted {:?}", path);
            return;
        }
        Err(error) => {
            println!("unable to delete file {}: {:?}", path.display(), error);
            return;
        }
    }
}

#[cfg(unix)]
pub fn set_executable(target: &Path) -> std::io::Result<()> {
    let file = File::open(target).unwrap();
    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o755); // a+rx, u+w
    file.set_permissions(perms)
}

#[cfg(not(unix))]
pub fn set_executable(target: &Path) -> std::io::Result<()> {
    Ok(())
}

pub fn symbolic_link_if_exists(src: &Path, dest: &Path) {
    match std::fs::read_link(dest.to_path_buf()) {
        Ok(target) => {
            if src == target {
                println!(
                    "already symlinked: {:?} -> {:?}",
                    dest.display(),
                    target.display(),
                );
                return;
            }
        }
        Err(_error) => {
            // does not exist, or not a symlink
        }
    };

    match std::fs::symlink_metadata(src) {
        Ok(attr) => attr,
        Err(error) => {
            println!("unable to access {}: {:?}", src.display(), error);
            return;
        }
    };

    match dest.parent() {
        Some(parent) => {
            std::fs::create_dir_all(&parent).expect(&format!(
                "unable to create directories {}",
                &parent.display()
            ).as_str());
        }
        None => { /* probably at root directory, nothing to do */ }
    };

    match std::fs::symlink_metadata(dest) {
        Ok(_attr) => {
            delete_if_exists(dest);
        }
        Err(_error) => { /* might not exist, continue */ }
    }

    match symbolic_link(&src, &dest) {
        Ok(()) => {
          println!(
              "symlinked: {} -> {}",
              dest.display(),
              src.display(),
          );
      }
        Err(error) => {
            println!(
                "unable to symlink {} to {}: {:?}",
                dest.display(),
                src.display(),
                error
            );
            return;
        }
    };
}

#[cfg(not(windows))]
fn symbolic_link(src: &Path, dest: &Path) -> Result<(), Error> {
    return std::os::unix::fs::symlink(src, dest);
}

#[cfg(windows)]
fn symbolic_link(src: &Path, dest: &Path) -> Result<(), Error> {
    let src_attr = match std::fs::symlink_metadata(src) {
        Ok(attr) => attr,
        Err(error) => {
            return Err(error);
        }
    };

    if src_attr.is_dir() {
        return std::os::windows::fs::symlink_dir(src, dest);
    }

    return std::os::windows::fs::symlink_file(src, dest);
}