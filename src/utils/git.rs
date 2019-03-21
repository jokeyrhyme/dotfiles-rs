use std::{io, path::PathBuf};

use crate::utils;

pub fn has_git() -> bool {
    match utils::process::command_output("git", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    }
}

pub fn path_is_git_repository<P>(path: P) -> bool
where
    P: Into<PathBuf>,
{
    let p = path.into();
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
    P: Into<PathBuf>,
{
    match utils::process::command_spawn_wait("git", &["-C", path.into().to_str().unwrap(), "pull"])
    {
        Ok(_) => {}
        Err(error) => println!("`git pull` failed: {}", error),
    }
}

pub fn shallow_clone<S>(source: S, target: S) -> io::Result<()>
where
    S: Into<String>,
{
    match utils::process::command_spawn_wait(
        "git",
        &["clone", "--depth", "1", &source.into(), &target.into()],
    ) {
        Ok(_status) => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn shallow_fetch<S>(target: S) -> io::Result<()>
where
    S: Into<String>,
{
    let t = target.into();
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

        let temp_path = mkdtemp().unwrap();
        let source_path = Path::new(env!("CARGO_MANIFEST_DIR"));

        shallow_clone(source_path.to_string_lossy(), temp_path.to_string_lossy())
            .expect("unable to shallow_clone()");

        assert!(path_is_git_repository(&temp_path), true);

        shallow_fetch(temp_path.to_string_lossy()).unwrap();

        utils::fs::delete_if_exists(&temp_path);
    }
}
