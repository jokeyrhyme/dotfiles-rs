use std::thread;

use crate::lib::{env::Exports, task::Task};

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
mod local;
#[cfg(target_os = "macos")]
mod macos;
mod minikube;
mod nodejs;
mod npm;
mod psql;
mod rust;
mod rustc;
mod rustup;
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

pub fn env() -> Exports {
    let mut exports: Exports = Default::default();
    exports = golang::env(exports);
    exports = local::env(exports);
    exports = nodejs::env(exports);
    exports = rustup::env(exports);
    exports = vim::env(exports);
    exports
}

pub fn all() {
    // let's run GitHub Release tasks in serial,
    // to not exacerbate rate limiting,
    // but in parallel with everything else
    let ghr_handle = thread::spawn(|| {
        for t in ghr_tasks() {
            t.sync_then_update();
        }
    });

    let rust_handle = thread::spawn(|| {
        for t in rust_tasks() {
            t.sync_then_update();
        }
    });

    for t in tasks() {
        t.sync_then_update();
    }

    ghr_handle.join().unwrap();
    rust_handle.join().unwrap();
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

fn rust_tasks() -> Vec<Task> {
    vec![rustup::task(), rustc::task(), rust::task()]
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
        npm::task(),    // must be after nodejs
        psql::task(),   // config
        ssh::task(),    // config
        tmux::task(),   // config
        vim::task(),    // config
        vscode::task(), // config
        zsh::task(),    // config
        #[cfg(windows)]
        windows::task(),
    ]
}
