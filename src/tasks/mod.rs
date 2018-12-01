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
    for t in serial_tasks() {
        t.sync_then_update();
    }

    // let's run GitHub Release tasks in serial,
    // to not exacerbate rate limiting,
    // but in parallel with everything else
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

    for t in tasks() {
        t.sync_then_update();
    }

    ghr_handle.join().unwrap();
    golang_handle.join().unwrap();
    nodejs_handle.join().unwrap();
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

fn golang_tasks() -> Vec<Task> {
    vec![
        golang::task(),
        goget::task(), // must be after golang
    ]
}

fn nodejs_tasks() -> Vec<Task> {
    vec![
        nodejs::task(),
        npm::task(), // must be after nodejs
    ]
}

fn rust_tasks() -> Vec<Task> {
    vec![rustup::task(), rustc::task(), rust::task()]
}

// these tasks should not be run concurrently with others
fn serial_tasks() -> Vec<Task> {
    vec![
        dotfiles::task(), // must be before "config" tasks
        vim::task(),      // config, causes vim to take over the terminal
    ]
}

fn tasks() -> Vec<Task> {
    vec![
        alacritty::task(), // config
        atom::task(),
        #[cfg(not(windows))]
        bash::task(), // config
        git::task(),
        hyper::task(), // config
        #[cfg(target_os = "macos")]
        macos::task(),
        psql::task(),   // config
        ssh::task(),    // config
        tmux::task(),   // config
        vscode::task(), // config
        #[cfg(not(windows))]
        zsh::task(), // config
        #[cfg(windows)]
        windows::task(),
    ]
}
