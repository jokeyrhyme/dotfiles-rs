use std;
use std::path::Path;

use utils;

const ERROR_MSG: &str = "pkg: vim";

pub fn sync() {
    if !has_vim() {
        return;
    }

    println!("pkg: vim: syncing...");

    // BEGIN: remove old vim configurations
    let vim_runtime = utils::env::home_dir().join(Path::new(".vim_runtime"));
    utils::fs::delete_if_exists(&vim_runtime);
    // END: remove old vim configurations

    let src = utils::env::home_dir().join(Path::new(".dotfiles/config/vimrc"));
    let vimrc = utils::env::home_dir().join(Path::new(".vimrc"));
    utils::fs::symbolic_link_if_exists(&src, &vimrc);

    fetch_vim_plug(true);

    utils::process::command_spawn_wait(
        "vim",
        &["-X", "-E", "-c", "PlugInstall", "-c", "q", "-c", "q"],
    ).expect(ERROR_MSG);
    utils::process::command_spawn_wait(
        "vim",
        &["-X", "-E", "-c", "PlugClean[!]", "-c", "q", "-c", "q"],
    ).expect(ERROR_MSG);
}

pub fn update() {
    if !has_vim() {
        return;
    }

    println!("pkg: vim: updating...");

    fetch_vim_plug(false);

    utils::process::command_spawn_wait(
        "vim",
        &["-X", "-E", "-c", "PlugUpdate", "-c", "q", "-c", "q"],
    ).expect(ERROR_MSG);
}

fn has_vim() -> bool {
    match utils::process::command_output("vim", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // vim probably not installed
        }
    }
}

fn fetch_vim_plug(skip_if_exists: bool) {
    #[cfg(not(windows))]
    let autoload = utils::env::home_dir().join(Path::new(".vim/autoload"));
    #[cfg(windows)]
    let autoload = utils::env::home_dir().join(Path::new("vimfiles/autoload"));

    match std::fs::create_dir_all(&autoload) {
        Ok(_created) => _created,
        Err(error) => {
            panic!(
                "unable to create directories {}: {:?}",
                autoload.to_str().unwrap_or("nil"),
                error
            );
        }
    }

    let vim_plug = autoload.join(Path::new("plug.vim"));
    match std::fs::symlink_metadata(&vim_plug) {
        Ok(_metadata) => {
            if skip_if_exists {
                return; // already exists, and we want to skip
            }
        }
        Err(_error) => {}
    }

    let vim_plug_url = String::from(
        "https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim",
    );
    match utils::http::download(&vim_plug_url, &vim_plug) {
        // ignore the outcome of this command
        Ok(_) => {}
        Err(_) => {}
    };
}