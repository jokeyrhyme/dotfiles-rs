use lib::task::{self, Status, Task};
use utils;

pub fn task() -> Task {
    Task {
        name: "tmux".to_string(),
        sync,
        update,
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

    utils::fs::symbolic_link_if_exists(&src, &dest);

    let tpm_path = utils::env::home_dir().join(".tmux/plugins/tpm");
    if !utils::git::path_is_git_repository(&tpm_path) {
        utils::fs::delete_if_exists(&tpm_path);
        let tpm_url = "https://github.com/tmux-plugins/tpm.git";
        match utils::git::shallow_clone(tpm_url, &tpm_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("tmux: unable to install tpm: {}", error),
        }
    }

    if utils::git::path_is_git_repository(&tpm_path) {
        let empty_args: &[&str] = &[];

        let tpm_install_path = tpm_path.join("bin/install_plugins");
        utils::process::command_spawn_wait(
            tpm_install_path.into_os_string().to_str().unwrap(),
            &empty_args,
        )?;

        let tpm_clean_path = tpm_path.join("bin/clean_plugins");
        utils::process::command_spawn_wait(
            tpm_clean_path.into_os_string().to_str().unwrap(),
            &empty_args,
        )?;
    }

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !has_tmux() {
        return Ok(Status::Skipped);
    }

    let tpm_path = utils::env::home_dir().join(".tmux/plugins/tpm");
    if utils::git::path_is_git_repository(&tpm_path) {
        match utils::git::shallow_fetch(tpm_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("tmux: unable to update tpm: {}", error),
        }

        let tpm_update_path = tpm_path.join("bin/update_plugins");
        utils::process::command_spawn_wait(
            tpm_update_path.into_os_string().to_str().unwrap(),
            &["all"],
        )?;
    }

    Ok(Status::Done)
}
