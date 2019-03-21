use std::{self, env::consts::OS, io};

use crate::lib::{
    env::Exports,
    task::{self, Status, Task},
};
use crate::utils::{
    self,
    fs::mkftemp,
    nodejs::{arch, bin_dir, os},
};

pub fn env(mut exports: Exports) -> Exports {
    let dir = bin_dir();
    if !exports.path.contains(&dir) {
        let mut paths = vec![dir];
        paths.append(&mut exports.path);
        exports.path = paths;
    }
    exports
}

pub fn task() -> Task {
    Task {
        name: String::from("nodejs"),
        sync,
        update,
    }
}

fn install_nodejs<S>(version: S) -> io::Result<()>
where
    S: Into<String>,
{
    let temp_path = mkftemp()?;

    let v = version.into();
    let prefix = format!("node-{}-{}-{}", &v, os(), arch());

    let remote_url = format!(
        "https://nodejs.org/dist/{}/{}.{}",
        &v,
        &prefix,
        if OS == "windows" { "zip" } else { "tar.gz" },
    );
    match utils::http::download(remote_url, &temp_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot download: {}", error);
            return Err(error);
        }
    };

    let local_path = utils::env::home_dir().join(".local");

    // archive contains a directory with name matching `prefix`
    let interim_path = local_path.join(&prefix);
    utils::fs::delete_if_exists(&interim_path);

    if OS == "windows" {
        utils::archive::extract_zip(&temp_path, &local_path)?;
    } else {
        utils::archive::extract_tar_gz(&temp_path, &local_path)?;
    }

    let target_path = local_path.join("node");
    utils::fs::delete_if_exists(&target_path);

    std::fs::rename(&interim_path, &target_path)?;

    utils::fs::delete_if_exists(&temp_path);

    Ok(())
}

fn sync() -> task::Result {
    if utils::nodejs::has_node() {
        return Ok(Status::Skipped);
    }

    let latest = utils::nodejs::latest_version();
    match install_nodejs(latest.clone()) {
        Ok(()) => Ok(Status::Changed(String::from("unknown"), latest)),
        Err(error) => Err(task::Error::IoError(
            String::from("unable to install Node.js"),
            error,
        )),
    }
}

fn update() -> task::Result {
    if !utils::nodejs::has_node() {
        return Ok(Status::Skipped);
    }

    let current = utils::nodejs::current_version();
    let latest = utils::nodejs::latest_version();

    if current == latest {
        Ok(Status::NoChange(current))
    } else {
        match install_nodejs(latest.clone()) {
            Ok(()) => Ok(Status::Changed(current, latest)),
            Err(error) => Err(task::Error::IoError(
                String::from("unable to install Node.js"),
                error,
            )),
        }
    }
}
