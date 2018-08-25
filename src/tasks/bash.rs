use utils;

pub fn sync() {
    if !has_bash() {
        return;
    }

    println!("bash: syncing ...");

    let it_path = utils::env::home_dir().join(".bash_it");
    if !utils::git::path_is_git_repository(&it_path) {
        utils::fs::delete_if_exists(&it_path);
        let it_url = "https://github.com/Bash-it/bash-it.git";
        match utils::git::shallow_clone(it_url, &it_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("bash: unable to install bash-it: {}", error),
        }
    }

    match utils::process::command_spawn_wait(
        "bash",
        &[
            utils::env::home_dir()
                .join(".bash_it/install.sh")
                .to_string_lossy()
                .as_ref(),
            "--silent",
            "--no-modify-config",
        ],
    ) {
        Ok(_status) => {}
        Err(error) => println!("bash: unable to run bash-it installer: {}", error),
    }

    // TODO: deprecated, deleted this later
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/inputrc"),
        utils::env::home_dir().join(".inputrc"),
    );
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/profile"),
        utils::env::home_dir().join(".profile"),
    );
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/bashrc"),
        utils::env::home_dir().join(".bashrc"),
    );

    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/inputrc"),
        utils::env::home_dir().join(".inputrc"),
    );
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/profile"),
        utils::env::home_dir().join(".profile"),
    );
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/bashrc"),
        utils::env::home_dir().join(".bashrc"),
    );
}

pub fn update() {
    if !has_bash() {
        return;
    }

    println!("bash: updating ...");

    let it_path = utils::env::home_dir().join(".bash_it");
    if utils::git::path_is_git_repository(&it_path) {
        match utils::git::shallow_fetch(it_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("bash: unable to update bash-it: {}", error),
        }
    }
}

fn has_bash() -> bool {
    match utils::process::command_output("bash", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // bash probably not installed
        }
    }
}
