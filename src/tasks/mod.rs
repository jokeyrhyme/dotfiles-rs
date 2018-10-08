use std::{env::var, path::PathBuf};

use lib::env::{Exports, Shell};

mod alacritty;
mod atom;
mod bash;
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

    alacritty::sync();
    atom::sync();
    bash::sync();
    dep::sync();
    git::sync();
    gitleaks::sync();
    gitsizer::sync();
    golang::sync();
    hadolint::sync();
    hyper::sync();
    jq::sync();
    #[cfg(target_os = "macos")]
    macos::sync();
    minikube::sync();
    nodejs::sync();
    psql::sync();
    rust::sync();
    shfmt::sync();
    skaffold::sync();
    ssh::sync();
    tmux::sync();
    vale::sync();
    vim::sync();
    vscode::sync();
    #[cfg(windows)]
    windows::sync();
    yq::sync();
    zsh::sync();
}

pub fn update() {
    // must be first
    dotfiles::update();

    alacritty::update();
    atom::update();
    bash::update();
    dep::update();
    git::update();
    gitleaks::update();
    gitsizer::update();
    golang::update();
    hadolint::update();
    hyper::update();
    jq::update();
    #[cfg(target_os = "macos")]
    macos::update();
    minikube::update();
    nodejs::update();
    psql::update();
    rust::update();
    shfmt::update();
    skaffold::update();
    ssh::update();
    tmux::update();
    vale::update();
    vim::update();
    vscode::update();
    #[cfg(windows)]
    windows::update();
    yq::update();
    zsh::update();
}
