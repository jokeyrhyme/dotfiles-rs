use std;
use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    match std::env::home_dir() {
        Some(path) => path,
        None => panic!("no $HOME set"),
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
