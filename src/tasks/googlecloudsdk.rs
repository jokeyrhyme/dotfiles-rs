use crate::{
    lib::{
        env::Exports,
        task::{self, Status, Task},
    },
    utils,
};

pub fn env(mut exports: Exports) -> Exports {
    let dir = utils::env::home_dir()
        .join(".local")
        .join("google-cloud-sdk")
        .join("bin");
    if !exports.path.contains(&dir) {
        let mut paths = vec![dir];
        paths.append(&mut exports.path);
        exports.path = paths;
    }
    exports
}

pub fn task() -> Task {
    Task {
        name: String::from("googlecloudsdk"),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    let sdk_path = utils::env::home_dir()
        .join(".local")
        .join("google-cloud-sdk");
    if utils::git::path_is_git_repository(&sdk_path) {
        return Ok(Status::Skipped);
    }

    utils::fs::delete_if_exists(&sdk_path);

    let sdk_url = "https://github.com/google-cloud-sdk/google-cloud-sdk.git";
    utils::git::shallow_clone(sdk_url, &sdk_path.to_string_lossy())?;

    Ok(Status::Done)
}

fn update() -> task::Result {
    let sdk_path = utils::env::home_dir()
        .join(".local")
        .join("google-cloud-sdk");
    if !utils::git::path_is_git_repository(&sdk_path) {
        return Ok(Status::Skipped);
    }

    utils::git::shallow_fetch(sdk_path.to_string_lossy())?;

    Ok(Status::Done)
}
