use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("macos"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    if OS != "macos" {
        return Ok(Status::Skipped);
    }

    utils::process::command_spawn_wait("qlmanage", &["-d", "1", "-r", "cache"])?;

    Ok(Status::Done)
}
