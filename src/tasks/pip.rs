use std::{fs, io};

use toml;

use crate::{
    lib::{
        favourites::Favourites,
        pip::{self, PipFavourites},
        task::{self, Status, Task},
    },
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("pip"),
        sync,
        update,
    }
}

fn read_config() -> io::Result<PipFavourites> {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/pip.toml");
    let contents = fs::read_to_string(&cfg_path)?;

    Ok(toml::from_str(&contents).expect("cannot parse .../pip.toml"))
}

fn sync() -> task::Result {
    if !pip::has_pip() {
        return Ok(Status::Skipped);
    }

    let mut favs = read_config()?;

    Favourites::fill_and_status(&mut favs)?;
    Favourites::cull_and_status(&mut favs)?;

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !pip::has_pip() {
        return Ok(Status::Done);
    }

    let favs = read_config()?;
    let mut args = vec![
        String::from("install"),
        String::from("--upgrade"),
        String::from("--user"),
    ];
    args.extend(favs.found());
    pip::pip(&args)?;

    Ok(Status::Done)
}
