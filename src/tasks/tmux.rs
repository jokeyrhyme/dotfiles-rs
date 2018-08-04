use utils;

const ERROR_MSG: &str = "error: tmux";

pub fn sync() {
    if !has_tmux() {
        return;
    }

    println!("tmux: syncing ...");

    let src = utils::env::home_dir().join(".dotfiles/config/tmux.conf");
    let dest = utils::env::home_dir().join(".tmux.conf");

    utils::fs::symbolic_link_if_exists(&src, &dest);

    let tpm_path = utils::env::home_dir().join(".tmux/plugins/tpm");

    if !tpm_path.is_dir() {
        utils::fs::delete_if_exists(&tpm_path);
    }

    if !utils::git::path_is_git_repository(&tpm_path) {
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
        ).expect(ERROR_MSG);

        let tpm_clean_path = tpm_path.join("bin/clean_plugins");
        utils::process::command_spawn_wait(
            tpm_clean_path.into_os_string().to_str().unwrap(),
            &empty_args,
        ).expect(ERROR_MSG);
    }
}

pub fn update() {
    if !has_tmux() {
        return;
    }

    println!("tmux: updating ...");

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
        ).expect(ERROR_MSG);
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
