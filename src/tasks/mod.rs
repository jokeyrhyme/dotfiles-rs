use std::{env::var, path::PathBuf, thread};

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
mod goget;
mod golang;
mod hadolint;
mod hyper;
mod jq;
#[cfg(target_os = "macos")]
mod macos;
mod minikube;
mod nodejs;
mod npm;
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

pub fn all() {
    // let's run GitHub Release tasks in serial,
    // to not exacerbate rate limiting,
    // but in parallel with everything else
    let ghr_handle = thread::spawn(|| {
        for t in ghr_tasks() {
            t.sync_then_update()
        }
    });

    for t in tasks() {
        t.sync_then_update()
    }

    ghr_handle.join().unwrap();
}

fn ghr_tasks() -> Vec<Task> {
    vec![
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

fn tasks() -> Vec<Task> {
    vec![
        dotfiles::task(),  // must be before "config" tasks
        alacritty::task(), // config
        atom::task(),
        bash::task(), // config
        git::task(),
        golang::task(),
        goget::task(), // must be after golang
        hyper::task(), // config
        #[cfg(target_os = "macos")]
        macos::task(),
        nodejs::task(),
        npm::task(),  // must be after nodejs
        psql::task(), // config
        rust::task(),
        ssh::task(),    // config
        tmux::task(),   // config
        vim::task(),    // config
        vscode::task(), // config
        zsh::task(),    // config
        #[cfg(windows)]
        windows::task(),
    ]
}
