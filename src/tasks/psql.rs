use utils;

pub fn sync() {
    if !has_psql() {
        return;
    }

    println!("pkg: psql: syncing ...");

    let src = utils::env::home_dir().join(".dotfiles/config/psqlrc");
    let dest = utils::env::home_dir().join(".psqlrc");

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}

fn has_psql() -> bool {
    match utils::process::command_output("psql", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // cargo probably not installed
        }
    }
}