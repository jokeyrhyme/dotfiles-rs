use subprocess::{Exec, Redirection};

use crate::lib::{
    brew,
    task::{self, Status, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("brewbundle"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    if !brew::has_brew() {
        return Ok(Status::Skipped);
    }
    let check = Exec::cmd("brew")
        .args(&["bundle", "check", "--global", "--no-upgrade"])
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?;
    if check.exit_status.success() {
        return Ok(Status::NoChange(String::from("already installed")));
    }
    brew::brew(&["bundle", "install", "--global", "--no-upgrade"])?;
    Ok(Status::Changed(
        String::from("unknown"),
        String::from("installed"),
    ))
}
