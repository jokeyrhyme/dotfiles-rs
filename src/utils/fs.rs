use std;
use std::path::Path;

use utils;

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
                utils::strings::unoption(target.to_str()),
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
                println!("deleted {}", utils::strings::unoption(path.to_str()));
                return;
            }
            Err(error) => {
                println!(
                    "unable to recursively delete directory {}: {:?}",
                    utils::strings::unoption(path.to_str()),
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
                utils::strings::unoption(path.to_str()),
                error
            );
            return;
        }
    }
}

pub fn symbolic_link(src: &Path, dest: &Path) {
    let src_attr = match std::fs::symlink_metadata(src) {
        Ok(attr) => attr,
        Err(error) => {
            println!(
                "unable to access {}: {:?}",
                utils::strings::unoption(src.to_str()),
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

    #[cfg(not(windows))]
    let linked = std::os::unix::fs::symlink(src, dest);
    match linked {
        Ok(linked) => linked,
        Err(error) => {
            println!(
                "unable to symlink {} to {}: {:?}",
                utils::strings::unoption(dest.to_str()),
                utils::strings::unoption(src.to_str()),
                error
            );
            return;
        }
    }

    if src_attr.is_dir() {
        #[cfg(windows)]
        let linked = std::os::windows::fs::symlink_dir(src, dest);
        match linked {
            Ok(linked) => linked,
            Err(error) => {
                println!(
                    "unable to symlink {} to {}: {:?}",
                    utils::strings::unoption(dest.to_str()),
                    utils::strings::unoption(src.to_str()),
                    error
                );
                return;
            }
        }
    }

    #[cfg(windows)]
    let linked = std::os::windows::fs::symlink_file(src, dest);
    match linked {
        Ok(linked) => linked,
        Err(error) => {
            println!(
                "unable to symlink {} to {}: {:?}",
                utils::strings::unoption(dest.to_str()),
                utils::strings::unoption(src.to_str()),
                error
            );
            return;
        }
    }

    #[cfg(any(unix,windows))]
    println!(
        "symlinked {} to {}",
        utils::strings::unoption(dest.to_str()),
        utils::strings::unoption(src.to_str()),
    );
}
