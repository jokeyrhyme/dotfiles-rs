use std::path::Path;

use utils;

pub fn sync() {
    if !has_psql() {
        return;
    }

    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/psqlrc"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".psqlrc"));

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}

fn has_psql() -> bool {
    match utils::process::command_output("psql", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // cargo probably not installed
        }
    }
}