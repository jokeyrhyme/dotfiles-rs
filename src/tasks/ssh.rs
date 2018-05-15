use std::fs;
use std::path::Path;

use utils::{self, ssh::Config};

pub fn sync() {
    if !utils::ssh::has_ssh() {
        return;
    }

    println!("pkg: ssh: syncing ...");

    // TODO: ensure contents of ~/.dotfiles/config/ssh is present in ~/.ssh/config

    let source_path = utils::env::home_dir()
        .join(".dotfiles")
        .join("config")
        .join("ssh");
    let source = match fs::read_to_string(&source_path) {
        Ok(s) => s,
        Err(_error) => String::from(""),
    };

    let target_path = utils::env::home_dir().join(".ssh").join("config");
    let target = match fs::read_to_string(&target_path) {
        Ok(s) => s,
        Err(_error) => String::from(""),
    };

    let config = Config::from(target.as_str()) | Config::from(source.as_str());

    match fs::write(&target_path, String::from(config)) {
        Ok(()) => {}
        Err(error) => {
            println!("error: pkg: ssh: unable to write config: {}", error);
        }
    }
}

pub fn update() {}