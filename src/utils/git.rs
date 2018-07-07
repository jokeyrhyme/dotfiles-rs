use std::io;
use std::path::Path;

use utils;

pub fn has_git() -> bool {
    return match utils::process::command_output("git", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    };
}

pub fn path_is_git_repository(path: &Path) -> bool {
    return match utils::process::command_output("git", &["-C", path.to_str().unwrap(), "status"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    };
}

pub fn pull(path: &Path) {
    println!("`git pull`ing in {} ...", path.display());
    if let Ok(_status) =
        utils::process::command_spawn_wait("git", &["-C", path.to_str().unwrap(), "pull"])
    {
        println!("`git pull` done!");
    }
}

pub fn shallow_clone<S: AsRef<str>, T: AsRef<str>>(source: &S, target: &T) -> io::Result<()> {
    match utils::process::command_spawn_wait(
        "git",
        &["clone", "--depth", "1", source.as_ref(), target.as_ref()],
    ) {
        Ok(_status) => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn shallow_fetch<T: AsRef<str>>(target: &T) -> io::Result<()> {
    match utils::process::command_spawn_wait(
        "git",
        &["-C", target.as_ref(), "fetch", "--depth", "1"],
    ) {
        Ok(_status) => {}
        Err(error) => {
            return Err(error);
        }
    }
    match utils::process::command_spawn_wait(
        "git",
        &["-C", target.as_ref(), "reset", "--hard", "FETCH_HEAD"],
    ) {
        Ok(_status) => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use mktemp;

    use super::*;

    #[test]
    #[cfg(unix)]
    fn has_git_is_true_for_unix() {
        let got = has_git();

        assert!(got);
    }

    #[test]
    fn path_is_git_repository_here() {
        if !has_git() {
            return;
        }

        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));

        assert!(path_is_git_repository(&project_path), true);
    }

    #[test]
    fn shallow_clone_and_shallow_fetch() {
        if !has_git() {
            return;
        }

        let temp_path;
        {
            let mut temp = mktemp::Temp::new_dir().unwrap();
            temp_path = temp.to_path_buf();
            temp.release();
        }
        let source_path = Path::new(env!("CARGO_MANIFEST_DIR"));

        shallow_clone(&source_path.to_str().unwrap(), &temp_path.to_str().unwrap()).unwrap();

        assert!(path_is_git_repository(&temp_path), true);

        shallow_fetch(&temp_path.to_str().unwrap()).unwrap();

        utils::fs::delete_if_exists(&temp_path);
    }
}
