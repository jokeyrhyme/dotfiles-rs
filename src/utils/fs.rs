use std;
use std::io::Error;
use std::path::Path;

pub fn create_dir_all_or_panic(target: std::option::Option<&Path>) {
    let target = match target {
        Some(target) => target,
        None => {
            // no path, or request to create root, so noop
            return;
        }
    };

    match std::fs::create_dir_all(target) {
        Ok(_created) => _created,
        Err(error) => {
            panic!(
                "unable to create directories {}: {:?}",
                target.to_str().unwrap_or("nil"),
                error
            );
        }
    }
}

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
                println!("deleted {}", path.to_str().unwrap_or("nil"));
                return;
            }
            Err(error) => {
                println!(
                    "unable to recursively delete directory {}: {:?}",
                    path.to_str().unwrap_or("nil"),
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
            println!(
                "unable to delete file {}: {:?}",
                path.to_str().unwrap_or("nil"),
                error
            );
            return;
        }
    }
}

pub fn symbolic_link_if_exists(src: &Path, dest: &Path) {
    match std::fs::symlink_metadata(src) {
        Ok(attr) => attr,
        Err(error) => {
            println!(
                "unable to access {}: {:?}",
                src.to_str().unwrap_or("nil"),
                error
            );
            return;
        }
    };

    create_dir_all_or_panic(dest.parent());

    match std::fs::symlink_metadata(dest) {
        Ok(_attr) => {
            delete_if_exists(dest);
        }
        Err(_error) => { /* might not exist, continue */ }
    }

    match symbolic_link(&src, &dest) {
      Ok(()) => {
          println!(
              "symlinked {} to {}",
              dest.to_str().unwrap_or("nil"),
              src.to_str().unwrap_or("nil"),
          );
      },
      Err(error) => {
          println!(
              "unable to symlink {} to {}: {:?}",
              dest.to_str().unwrap_or("nil"),
              src.to_str().unwrap_or("nil"),
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