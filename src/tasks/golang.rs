use mktemp;

use utils::{self, golang::{arch, os}};

pub fn sync () {
    if utils::golang::is_installed() {
        return;
    }

    println!("pkg: golang: syncing ...");

    let latest_version = match utils::golang::latest_version() {
        Ok(v) => v,
        Err(error) => {
            println!("error: pkg: golang: unable to check for updates: {:?}", error);
            return;
        }
    };

    match install_golang(&latest_version) {
        Ok(_) => {}
        Err(error) => {
            println!("error: pkg: golang: unable to install: {:?}", error);
            return;
        }
    };
}

pub fn update () {
    if !utils::golang::is_installed() {
        return;
    }

    println!("pkg: golang: updating ...");

    let current_version = utils::golang::current_version();
    let latest_version = match utils::golang::latest_version() {
        Ok(v) => v,
        Err(error) => {
            println!("error: pkg: golang: unable to check for updates: {:?}", error);
            return;
        }
    };

    println!("current={} latest={}", &current_version, &latest_version);
    if current_version != latest_version {
        match install_golang(&latest_version) {
            Ok(_) => {}
            Err(error) => {
                println!("error: pkg: golang: unable to install: {:?}", error);
                return;
            }
        };
    }
}

fn install_golang(version: &str) -> Result<(), utils::golang::GolangError> {
    println!("pkg: golang: installing {} ...", &version);

    let temp_path;
    {
        let mut temp = mktemp::Temp::new_file()?;
        temp_path = temp.to_path_buf();
        temp.release();
    }

    #[cfg(windows)]
    let remote_url = format!("https://dl.google.com/go/{}.{}-{}.zip", version, os(), arch());
    #[cfg(not(windows))]
    let remote_url = format!("https://dl.google.com/go/{}.{}-{}.tar.gz", version, os(), arch());

    utils::http::download(&remote_url, &temp_path)?;

    let local_path = utils::env::home_dir().join(".local");
    let target_path = local_path.join("go");
    utils::fs::delete_if_exists(&target_path);

    #[cfg(windows)]
    utils::archive::extract_zip(&temp_path, &local_path)?;
    #[cfg(not(windows))]
    utils::archive::extract_tar_gz(&temp_path, &local_path)?;

    utils::fs::delete_if_exists(&temp_path);

    Ok(())
}