use regex;

use crate::lib::{
    rust,
    task::{self, Status, Task},
};

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
}

const TOOLCHAINS: &[&str] = &["stable", "nightly"];

pub fn task() -> Task {
    Task {
        name: "rustc".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    if !rust::has_rustup() {
        return Ok(Status::Skipped);
    }

    let toolchains = rust::rustup_output(&["toolchain", "list"])?;

    for t in TOOLCHAINS {
        let re = regex::Regex::new(&format!("^{}-", t)).unwrap();
        if !re.is_match(&toolchains) {
            rust::rustup(&["toolchain", "install", t])?;
        }
    }

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !rust::has_rustup() {
        return Ok(Status::Skipped);
    }

    rust::rustup(&["update"])?;

    Ok(Status::Done)
}
