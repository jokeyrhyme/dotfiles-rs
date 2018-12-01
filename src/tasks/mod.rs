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
    dotfiles::task().sync_then_update(); // provides: config; must be first

    let ghr_handle = thread::spawn(|| {
        for t in ghr_tasks() {
            t.sync_then_update();
        }
    });

    let golang_handle = thread::spawn(|| {
        for t in golang_tasks() {
            t.sync_then_update();
        }
    });

    let nodejs_handle = thread::spawn(|| {
        for t in nodejs_tasks() {
            t.sync_then_update();
        }
    });

    let rust_handle = thread::spawn(|| {
        for t in rust_tasks() {
            t.sync_then_update();
        }
    });

    ghr_handle.join().unwrap();
    golang_handle.join().unwrap();
    nodejs_handle.join().unwrap();
    rust_handle.join().unwrap();

    // run remaining tasks in serial,
    // without any other concurrent tasks
    for t in tasks() {
        t.sync_then_update();
    }
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

fn golang_tasks() -> Vec<Task> {
    vec![
        golang::task(),
        goget::task(), // deps: golang
    ]
}

fn nodejs_tasks() -> Vec<Task> {
    vec![
        nodejs::task(),
        npm::task(), // deps: nodejs
    ]
}

fn rust_tasks() -> Vec<Task> {
    vec![rustup::task(), rustc::task(), rust::task()]
}

fn tasks() -> Vec<Task> {
    vec![
        alacritty::task(), // deps: config
        atom::task(),
        #[cfg(not(windows))]
        bash::task(), // deps: config
        git::task(),   // deps: nodejs/npm
        hyper::task(), // deps: config
        #[cfg(target_os = "macos")]
        macos::task(),
        psql::task(),   // deps: config
        ssh::task(),    // deps: config
        tmux::task(),   // deps: config
        vim::task(),    // deps: config; takes over the terminal
        vscode::task(), // deps: config
        #[cfg(not(windows))]
        zsh::task(), // deps: config
        #[cfg(windows)]
        windows::task(),
    ]
}
