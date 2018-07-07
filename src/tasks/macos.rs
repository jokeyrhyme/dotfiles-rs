use utils;

pub fn sync() {
    println!("macos: syncing ...");

    match utils::process::command_spawn_wait("qlmanage", &["-d", "1", "-r", "cache"]) {
        Ok(_) => {}
        Err(error) => println!("macos: unable to wipe Quick Look cache: {}", error),
    }
}

pub fn update() {}
