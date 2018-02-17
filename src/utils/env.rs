use std;

pub fn home_dir() -> std::path::PathBuf {
    return match std::env::home_dir() {
        Some(path) => path,
        None => panic!("no $HOME set"),
    };
}
