use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => panic!("cannot determine home directory"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_dir() {
        home_dir();
    }
}
