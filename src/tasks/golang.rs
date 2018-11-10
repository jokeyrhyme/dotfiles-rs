use lib::task::{self, Status, Task};
use utils::{
    self,
    fs::mktemp,
    golang::{arch, os},
};

pub fn task() -> Task {
    Task {
        name: "golang".to_string(),
        sync,
        update,
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn install_golang<S>(version: S) -> task::Result
where
    S: Into<String> + AsRef<str>,
{
    let current = if utils::golang::is_installed() {
        utils::golang::current_version()
    } else {
        "absent".to_string()
    };

    let temp_path = mktemp()?;

    #[cfg(windows)]
    let remote_url = format!(
        "https://dl.google.com/go/{}.{}-{}.zip",
        version.as_ref(),
        os(),
        arch()
    );
    #[cfg(not(windows))]
    let remote_url = format!(
        "https://dl.google.com/go/{}.{}-{}.tar.gz",
        version.as_ref(),
        os(),
        arch()
    );

    utils::http::download(remote_url, &temp_path)?;

    let local_path = utils::env::home_dir().join(".local");
    let target_path = local_path.join("go");
    utils::fs::delete_if_exists(&target_path);

    #[cfg(windows)]
    utils::archive::extract_zip(&temp_path, &local_path)?;
    #[cfg(not(windows))]
    utils::archive::extract_tar_gz(&temp_path, &local_path)?;

    utils::fs::delete_if_exists(&temp_path);

    Ok(Status::Changed(current, version.as_ref().to_string()))
}

fn sync() -> task::Result {
    if utils::golang::is_installed() {
        Ok(Status::NoChange("present".to_string()))
    } else {
        let latest_version = match utils::golang::latest_version() {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };

        install_golang(latest_version)
    }
}

fn update() -> task::Result {
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
