use std::{io, path::Path};

use crate::utils;

pub fn has() -> bool {
    match utils::process::command_output("git", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    }
}

pub fn path_is_git_repository<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let p = path.as_ref();
    if !p.is_dir() {
        return false;
    }
    match utils::process::command_output("git", &["-C", &p.to_string_lossy(), "status"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    }
}

pub fn pull<P>(path: P)
where
    P: AsRef<Path>,
{
    match utils::process::command_spawn_wait(
        "git",
        &["-C", path.as_ref().to_str().unwrap(), "pull"],
    ) {
        Ok(_) => {}
        Err(error) => println!("`git pull` failed: {}", error),
    }
}

pub fn shallow_clone<S>(source: S, target: S) -> io::Result<()>
where
    S: AsRef<str>,
{
    match utils::process::command_spawn_wait(
        "git",
        &["clone", "--depth", "1", &source.as_ref(), &target.as_ref()],
    ) {
        Ok(_status) => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn shallow_fetch<S>(target: S) -> io::Result<()>
where
    S: AsRef<str>,
{
    let t = target.as_ref();
    match utils::process::command_spawn_wait("git", &["-C", &t, "fetch", "--depth", "1"]) {
        Ok(_status) => {}
        Err(error) => {
            return Err(error);
        }
    }
    match utils::process::command_spawn_wait("git", &["-C", &t, "reset", "--hard", "FETCH_HEAD"]) {
        Ok(_status) => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::utils::fs::mkdtemp;

    use super::*;

    #[test]
    #[cfg(unix)]
    fn has_is_true_for_unix() {
        let got = has();

        assert!(got);
    }

    #[test]
    fn path_is_git_repository_here() {
        if !has() {
            return;
        }

        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));

        assert!(path_is_git_repository(&project_path), true);
    }

    #[test]
    fn shallow_clone_and_shallow_fetch() {
        if !has() {
            return;
        }

        let temp_path = mkdtemp().unwrap();
        let source_path = Path::new(env!("CARGO_MANIFEST_DIR"));

        shallow_clone(source_path.to_string_lossy(), temp_path.to_string_lossy())
            .expect("unable to shallow_clone()");

        assert!(path_is_git_repository(&temp_path), true);

        shallow_fetch(temp_path.to_string_lossy()).unwrap();

        utils::fs::delete_if_exists(&temp_path);
    }
}
