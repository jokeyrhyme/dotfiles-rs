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
    // must be first
    dotfiles::sync();

    for t in tasks() {
        println!("{}: sync: ...", t.name);
        match (t.sync)() {
            Ok(status) => println!("{}: sync: {}", t.name, status),
            Err(error) => println!("{}: sync error: {:?}", t.name, error),
        }
    }

    atom::sync();
    git::sync();
    golang::sync();
    hyper::sync();
    #[cfg(target_os = "macos")]
    macos::sync();
    nodejs::sync();
    psql::sync();
    rust::sync();
    ssh::sync();
    tmux::sync();
    vim::sync();
    vscode::sync();
    #[cfg(windows)]
    windows::sync();
    zsh::sync();
}

pub fn update() {
    // must be first
    dotfiles::update();

    for t in tasks() {
        println!("{}: update: ...", t.name);
        match (t.update)() {
            Ok(status) => println!("{}: update: {}", t.name, status),
            Err(error) => println!("{}: update error: {:?}", t.name, error),
        }
    }

    atom::update();
    git::update();
    golang::update();
    hyper::update();
    #[cfg(target_os = "macos")]
    macos::update();
    nodejs::update();
    psql::update();
    rust::update();
    ssh::update();
    tmux::update();
    vim::update();
    vscode::update();
    #[cfg(windows)]
    windows::update();
    zsh::update();
}

fn tasks() -> Vec<Task> {
    vec![
        alacritty::task(),
        bash::task(),
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
