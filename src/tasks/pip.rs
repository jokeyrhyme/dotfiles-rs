use std::{fs, io, path::PathBuf};

use crate::{
    lib::{
        env::Exports,
        favourites::Favourites,
        pip::{self, PipFavourites},
        python::exe_output,
        task::{self, Status, Task},
    },
    utils,
};

// python -c "import site; print(site.USER_BASE)"
// python -c "import site; print(site.USER_SITE)"

pub fn env(mut exports: Exports) -> Exports {
    let user_base = match exe_output(&["-c", "import site; print(site.USER_BASE)"]) {
        Ok(ub) => ub,
        Err(_) => return exports,
    };
    let dir = PathBuf::from(user_base.trim()).join("bin");
    if !exports.path.contains(&dir) {
        let mut paths = vec![dir];
        paths.append(&mut exports.path);
        exports.path = paths;
    }
    exports
}

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
    if !pip::has() {
        return Ok(Status::Skipped);
    }

    let mut favs = read_config()?;

    Favourites::fill_and_status(&mut favs)?;
    Favourites::cull_and_status(&mut favs)?;

    Ok(Status::Done)
}

fn update(_: Status) -> task::Result {
    if !pip::has() {
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
