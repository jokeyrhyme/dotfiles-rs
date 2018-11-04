use lib::task::{self, Status, Task};
use utils;

pub fn task() -> Task {
    Task {
        name: "macos".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    match utils::process::command_spawn_wait("qlmanage", &["-d", "1", "-r", "cache"]) {
        Ok(_) => Ok(Status::Done),
        Err(error) => task::Error::IOError("unable to wipe Quick Look cache", error),
    }
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
