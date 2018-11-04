use lib::task::{self, Status, Task};
use utils;

pub fn task() -> Task {
    Task {
        name: "psql".to_string(),
        sync,
        update,
    }
}

fn has_psql() -> bool {
    match utils::process::command_output("psql", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // cargo probably not installed
        }
    }
}

fn sync() -> task::Result {
    if !has_psql() {
        return Ok(Status::Skipped);
    }

    let src = utils::env::home_dir().join(".dotfiles/config/psqlrc");
    let dest = utils::env::home_dir().join(".psqlrc");

    utils::fs::symbolic_link_if_exists(&src, &dest);
    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
