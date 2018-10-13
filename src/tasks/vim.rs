use std::{self, io, path::PathBuf};

use which::which;

use lib::env::Exports;
use utils;

const ERROR_MSG: &str = "vim";

const PLUG_VIM: &str = "plug.vim";

const VIMS: [Vim; 2] = [
    Vim {
        #[cfg(not(windows))]
        autoload_parent_dir: ".vim",
        #[cfg(windows)]
        autoload_parent_dir: "vimfiles",
        command: "vim",
        #[cfg(not(windows))]
        rc_file: ".vimrc",
        #[cfg(windows)]
        rc_file: "_vimrc",
    },
    Vim {
        // TODO: determine Windows and macOS details
        autoload_parent_dir: ".local/share/nvim/site",
        command: "nvim",
        rc_file: ".config/nvim/init.vim",
    },
];

pub fn env(mut exports: Exports) -> Exports {
    for vim in &VIMS {
        if let Ok(found) = which(&vim.command) {
            exports.editor = found;
        }
    }
    exports
}

pub fn sync() {
    let src = utils::env::home_dir().join(".dotfiles/config/vimrc");

    for vim in &VIMS {
        if !vim.exists() {
            continue;
        }
        println!("{}: syncing...", &vim.command);

        utils::fs::symbolic_link_if_exists(&src, &vim.rc_path());

        if !vim.has_vim_plug() {
            match vim.install_vim_plug() {
                Ok(_) => {}
                Err(error) => {
                    // warn, but continue
                    println!(
                        "error: {}: unable to install vim-plug: {:?}",
                        &vim.command, error
                    );
                }
            }
        }

        utils::process::command_spawn_wait(
            &vim.command,
            &["-E", "-c", "PlugInstall", "-c", "q", "-c", "q"],
        ).expect(ERROR_MSG);
        utils::process::command_spawn_wait(
            &vim.command,
            &["-E", "-c", "PlugClean[!]", "-c", "q", "-c", "q"],
        ).expect(ERROR_MSG);
    }

    // BEGIN: remove old vim configurations
    let vim_runtime = utils::env::home_dir().join(".vim_runtime");
    utils::fs::delete_if_exists(&vim_runtime);
    // END: remove old vim configurations
}

pub fn update() {
    for vim in &VIMS {
        if !vim.exists() {
            continue;
        }
        println!("{}: updating...", &vim.command);

        match vim.install_vim_plug() {
            Ok(_) => {}
            Err(error) => {
                // warn, but continue
                println!(
                    "error: {}: unable to install vim-plug: {:?}",
                    &vim.command, error
                );
            }
        }

        utils::process::command_spawn_wait(
            &vim.command,
            &["-E", "-c", "PlugUpdate", "-c", "q", "-c", "q"],
        ).expect(ERROR_MSG);
    }
}

#[derive(Debug)]
struct Vim<'a> {
    autoload_parent_dir: &'a str, // used in: $HOME/autoload_dir/autoload
    command: &'a str,
    rc_file: &'a str, // used in: $HOME/rc_path
}

impl<'a> Vim<'a> {
    fn autoload_dir(&self) -> PathBuf {
        utils::env::home_dir()
            .join(&self.autoload_parent_dir)
            .join("autoload")
    }

    fn exists(&self) -> bool {
        match utils::process::command_output(&self.command, &["--version"]) {
            Ok(output) => output.status.success(),
            Err(_error) => false,
        }
    }

    fn has_vim_plug(&self) -> bool {
        let vim_plug = self.autoload_dir().join(PLUG_VIM);
        match std::fs::symlink_metadata(&vim_plug) {
            Ok(_metadata) => true,
            Err(_error) => false,
        }
    }

    fn install_vim_plug(&self) -> io::Result<()> {
        std::fs::create_dir_all(self.autoload_dir())?;

        let vim_plug = self.autoload_dir().join(PLUG_VIM);
        let vim_plug_url =
            String::from("https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim");
        utils::http::download(vim_plug_url, vim_plug)?;
        Ok(())
    }

    fn rc_path(&self) -> PathBuf {
        utils::env::home_dir().join(&self.rc_file)
    }
}
