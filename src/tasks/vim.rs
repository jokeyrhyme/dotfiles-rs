use std::path::Path;

use utils;

pub fn sync() {
    if !has_vim() {
        return;
    }

    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/my_configs.vim"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".vim_runtime/my_configs.vim"));

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}

fn has_vim() -> bool {
    match utils::process::command_output("vim", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // cargo probably not installed
        }
    }
}