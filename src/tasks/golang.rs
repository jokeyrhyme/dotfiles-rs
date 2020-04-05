use std::env::consts::OS;

use crate::lib::{
    env::Exports,
    task::{self, Status, Task},
};
use crate::utils::{
    self,
    fs::mkftemp,
    golang::{arch, bin_dir, gopath, goroot, os},
};

pub fn env(mut exports: Exports) -> Exports {
    let gp = gopath();
    if gp.is_dir() {
        exports.gopath = gp;
    }

    let gr = goroot();
    if gr.is_dir() {
        exports.goroot = gr;
    }

    for dir in vec![bin_dir(), gopath().join("bin")] {
        if !exports.path.contains(&dir) {
            let mut paths = vec![dir];
            paths.append(&mut exports.path);
            exports.path = paths;
        }
    }

    exports
}

pub fn task() -> Task {
    Task {
        name: String::from("golang"),
        sync,
        update,
    }
}

fn install_golang<S>(version: S) -> task::Result
where
    S: AsRef<str>,
{
    let current = if utils::golang::is_installed() {
        utils::golang::current_version()
    } else {
        String::from("absent")
    };

    let temp_path = mkftemp()?;

    let v = version.as_ref();
    let remote_url = format!(
        "https://dl.google.com/go/{}.{}-{}.{}",
        &v,
        os(),
        arch(),
        if OS == "windows" { "zip" } else { "tar.gz" },
    );
    utils::http::download(remote_url, &temp_path)?;

    let local_path = utils::env::home_dir().join(".local");
    let target_path = local_path.join("go");
    utils::fs::delete_if_exists(&target_path);

    if OS == "windows" {
        utils::archive::extract_zip(&temp_path, &local_path)?;
    } else {
        utils::archive::extract_tar_gz(&temp_path, &local_path)?;
    }

    utils::fs::delete_if_exists(&temp_path);

    Ok(Status::Changed(current, String::from(v)))
}

fn sync() -> task::Result {
    if utils::golang::is_installed() {
        Ok(Status::NoChange(String::from("present")))
    } else {
        let latest_version = match utils::golang::latest_version() {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };

        install_golang(latest_version)
    }

    // TODO: cleanup GOPATH/pkg: https://github.com/golang/go/issues/4719
}

fn update(_: Status) -> task::Result {
    if !utils::golang::is_installed() {
        return Ok(Status::Skipped);
    }

    let current = utils::golang::current_version();
    let latest = match utils::golang::latest_version() {
        Ok(v) => v,
        Err(error) => {
            return Err(error);
        }
    };

    if current == latest {
        Ok(Status::NoChange(current))
    } else {
        install_golang(latest)
    }
}
