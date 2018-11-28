use crate::lib::task::{self, Status, Task};
use crate::utils;

const ERROR_MSG: &str = "error: git";

pub fn task() -> Task {
    Task {
        name: "git".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    // TODO: synchronise git settings

    if !utils::git::has_git() {
        return Ok(Status::Skipped);
    }
    if !utils::nodejs::has_npx() {
        return Ok(Status::Skipped);
    }

    // https://www.npmjs.com/package/npm-merge-driver
    utils::process::command_spawn_wait("npx", &["-q", "npm-merge-driver", "install", "--global"])
        .expect(ERROR_MSG);

    if utils::nodejs::has_yarn() {
        // https://www.npmjs.com/package/npm-merge-driver
        utils::process::command_spawn_wait(
            "npx",
            &[
                "-q",
                "npm-merge-driver",
                "install",
                "--global",
                "--driver-name",
                "yarn-merge-driver",
                "--driver",
                "npx npm-merge-driver merge %A %O %B %P -c yarn",
                "--files",
                "yarn.lock",
            ],
        ).expect(ERROR_MSG);
    }

    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
