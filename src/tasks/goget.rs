use std::fs;

use toml;

use crate::{
    lib::{
        favourites::Favourites,
        goget::GoGetFavourites,
        task::{self, Status, Task},
    },
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("goget"),
        sync,
        update,
    }
}

fn read_config() -> GoGetFavourites {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/golang.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("goget: ignoring config: {}", error);
            return Default::default();
        }
    };

    match toml::from_str(&contents) {
        Ok(c) => c,
        Err(error) => {
            println!(
                "warning: goget: unable to parse {}, {}",
                &cfg_path.display(),
                error
            );
            Default::default()
        }
    }
}

fn sync() -> task::Result {
    if !utils::golang::is_installed() {
        return Ok(Status::Done);
    }

    let mut favs = read_config();
    Favourites::fill_and_status(&mut favs)?;
    Favourites::cull_and_status(&mut favs)?;

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !utils::golang::is_installed() {
        return Ok(Status::Skipped);
    }

    let favs = read_config();

    let mut install_args = vec![String::from("get"), String::from("-u")];
    install_args.extend(favs.found());

    utils::process::command_spawn_wait("go", &install_args)?;

    Ok(Status::Changed(
        String::from("unknown"),
        String::from("unknown"),
    ))
}
