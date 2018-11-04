use lib::task::{self, Status, Task};
use utils;

pub fn task() -> Task {
    Task {
        name: "hyper".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    let src = utils::env::home_dir().join(".dotfiles/config/hyper.js");
    let dest = utils::env::home_dir().join(".hyper.js");

    utils::fs::symbolic_link_if_exists(&src, &dest);
    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
