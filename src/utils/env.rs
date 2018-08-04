use std;
use std::path::PathBuf;

use dirs;

pub fn home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => panic!("cannot determine home directory"),
    }
}

pub fn path_dirs() -> Vec<PathBuf> {
    match std::env::var_os("PATH") {
        Some(value) => std::env::split_paths(&value).collect(),
        None => Vec::<PathBuf>::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_dir() {
        home_dir();
    }

    #[test]
    fn test_path_dirs() {
        let paths = path_dirs();
        assert!(paths.len() >= 1);
    }
}
