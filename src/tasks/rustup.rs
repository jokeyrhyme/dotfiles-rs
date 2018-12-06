use crate::lib::{
    env::Exports,
    rust,
    task::{self, Status, Task},
};

pub fn env(mut exports: Exports) -> Exports {
    let dir = rust::bin_dir();
    if !exports.path.contains(&dir) {
        let mut paths = vec![dir];
        paths.append(&mut exports.path);
        exports.path = paths;
    }
    exports
}

pub fn task() -> Task {
    Task {
        name: "rustup".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    if rust::has_rustup() {
        Ok(Status::NoChange(rust::rustup_version()))
    } else {
        // TODO: automate installation from https://rustup.rs
        Ok(Status::Skipped)
    }
}

fn update() -> task::Result {
    if !rust::has_rustup() {
        return Ok(Status::Skipped);
    }

    let current = rust::rustup_version();

    rust::rustup(&["self", "update"])?;

    let latest = rust::rustup_version();

    if current == latest {
        Ok(Status::NoChange(current))
    } else {
        Ok(Status::Changed(current, latest))
    }
}
