use dirs;
use which::which;

use crate::{
    lib::task::{self, Status, Task},
    utils::process::command_spawn_wait,
};

pub fn task() -> Task {
    Task {
        name: String::from("fccache"),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    Ok(Status::Skipped)
}

fn update() -> task::Result {
    if which("fc-cache").is_err() {
        return Ok(Status::Skipped);
    }
    if dirs::font_dir().is_none() {
        return Ok(Status::Skipped);
    };
    if let Err(e) = command_spawn_wait("fc-cache", &["--really-force"]) {
        println!("error: unable to rebuild font caches: {:?}", e);
    }
    Ok(Status::Changed(
        String::from("unknown"),
        String::from("rebuilt"),
    ))
}
