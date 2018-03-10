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
    println!("`git pull`ing in {} ...", path.to_str().unwrap());
    if let Ok(_status) = utils::process::command_spawn_wait(
        "git",
        &["-C", path.to_str().unwrap(), "pull"],
    )
    {
        println!("`git pull` done!");
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::*;

    #[test]
    fn has_git_is_true_for_unix() {
        let got = has_git();

        assert!(got);
    }
}
