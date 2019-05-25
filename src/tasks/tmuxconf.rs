use crate::lib::task::{self, Status, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: String::from("tmuxconf"),
        sync,
        ..Default::default()
    }
}

fn has_tmux() -> bool {
    match utils::process::command_output("tmux", &["-V"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // tmux probably not installed
        }
    }
}

fn sync() -> task::Result {
    if !has_tmux() {
        return Ok(Status::Skipped);
    }

    let src = utils::env::home_dir().join(".dotfiles/config/tmux.conf");
    let dest = utils::env::home_dir().join(".tmux.conf");

    utils::fs::symbolic_link_if_exists(&src, &dest)
}
