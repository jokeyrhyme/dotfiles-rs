use std::env::consts::OS;

use crate::lib::task::{self, Status, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: "windows".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    if OS != "windows" {
        return Ok(Status::Skipped);
    }

    println!("windows: manually configure %PATH% to include:");

    let bin_path = utils::env::home_dir().join(".local").join("bin");
    println!("- {}", bin_path.display());

    let go_bin_path = utils::env::home_dir().join(".local").join("go").join("bin");
    println!("- {}", go_bin_path.display());

    let node_bin_path = utils::env::home_dir().join(".local").join("node");
    println!("- {}", node_bin_path.display());

    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
