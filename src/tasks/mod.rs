use std::{env::var, path::PathBuf};

use lib::{
    env::{Exports, Shell},
    task::Task,
};

mod alacritty;
mod atlantis;
mod atom;
mod bash;
mod bazel;
mod dep;
mod dotfiles;
mod git;
mod gitleaks;
mod gitsizer;
mod golang;
mod hadolint;
mod hyper;
mod jq;
#[cfg(target_os = "macos")]
mod macos;
mod minikube;
mod nodejs;
mod psql;
mod rust;
mod shfmt;
mod skaffold;
mod ssh;
mod tmux;
mod vale;
mod vim;
mod vscode;
#[cfg(windows)]
mod windows;
mod yq;
mod zsh;

pub fn env() {
    let mut exports = Exports {
        editor: PathBuf::new(),
        path: Vec::<PathBuf>::new(),
    };
    exports = vim::env(exports);
    let shell = var("SHELL").unwrap_or_default();
    println!("{}", exports.to_shell(Shell::from(shell.as_str())));
}

pub fn sync() {
    for t in tasks() {
        println!("{}: sync: ...", t.name);
        match (t.sync)() {
            Ok(status) => println!("{}: sync: {}", t.name, status),
            Err(error) => println!("{}: sync error: {:?}", t.name, error),
        }
    }
}

pub fn update() {
    for t in tasks() {
        println!("{}: update: ...", t.name);
        match (t.update)() {
            Ok(status) => println!("{}: update: {}", t.name, status),
            Err(error) => println!("{}: update error: {:?}", t.name, error),
        }
    }
}

fn tasks() -> Vec<Task> {
    vec![
        dotfiles::task(),  // must be before "config" tasks
        alacritty::task(), // config
        atom::task(),
        bash::task(), // config
        git::task(),
        golang::task(),
        hyper::task(), // config
        #[cfg(target_os = "macos")]
        macos::task(),
        nodejs::task(),
        psql::task(), // config
        rust::task(),
        ssh::task(),    // config
        tmux::task(),   // config
        vim::task(),    // config
        vscode::task(), // config
        zsh::task(),    // config
        #[cfg(windows)]
        windows::task(),
        // GitHub Release tasks
        atlantis::task(),
        bazel::task(),
        dep::task(),
        gitleaks::task(),
        gitsizer::task(),
        hadolint::task(),
        jq::task(),
        minikube::task(),
        shfmt::task(),
        skaffold::task(),
        vale::task(),
        yq::task(),
    ]
}
