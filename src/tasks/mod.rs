use std::collections::HashMap;

use crate::lib::{env::Exports, task::Task};

mod atlantis;
mod bash;
mod bazel;
mod brew;
mod dep;
mod dotfiles;
mod git;
mod gitleaks;
mod gitsizer;
mod goget;
mod golang;
mod googlecloudsdk;
mod hadolint;
mod jq;
mod local;
mod macos;
mod minikube;
mod nodejs;
mod npm;
mod pip;
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
mod vscodejson;
mod windows;
mod yq;
mod zsh;

pub fn env() -> Exports {
    let mut exports: Exports = Default::default();
    exports = brew::env(exports);
    exports = golang::env(exports);
    exports = googlecloudsdk::env(exports);
    exports = local::env(exports);
    exports = nodejs::env(exports);
    exports = pip::env(exports);
    exports = rustup::env(exports);
    exports = vim::env(exports);
    exports
}

pub fn all() {
    // resource utilisation goals:
    // - distinguish CPU-heavy from I/O-heavy tasks
    // - run a CPU-heavy task concurrently with an I/O-heavy task
    // - serialise tasks of the same type to avoid clogging pipes
    // TODO: realise these goals :)
    // TODO: maybe queue I/O within GitHub and HTTP helpers

    let tasks = mapping();
    for s in sequence() {
        if let Some(t) = tasks.get(&s) {
            t.sync_then_update();
        }
    }
}

pub fn some<S>(input: S)
where
    S: AsRef<str>,
{
    let tasks = mapping();
    for name in input.as_ref().split(',') {
        if let Some(task) = tasks.get(name) {
            task.sync_then_update();
        }
    }
}

fn sequence() -> Vec<String> {
    vec![
        dotfiles::task().name, // provides: config; must be first
        brew::task().name,
        golang::task().name,
        goget::task().name, // deps: config,golang
        nodejs::task().name,
        npm::task().name, // deps: config,nodejs
        pip::task().name,
        rustup::task().name,
        rustc::task().name, // deps: rustup
        rust::task().name,  // deps: config,rustc
        // these are GitHub Release tasks,
        // that are mostly I/O-heavy,
        // and serialising such things avoids clogging our pipes
        atlantis::task().name,
        bazel::task().name,
        dep::task().name,
        gitleaks::task().name,
        gitsizer::task().name,
        hadolint::task().name,
        jq::task().name,
        minikube::task().name,
        shfmt::task().name,
        skaffold::task().name,
        vale::task().name,
        yq::task().name,
        bash::task().name, // deps: bashprofile,bashrc,inputrc,profile
        git::task().name,  // deps: nodejs/npm
        googlecloudsdk::task().name,
        macos::task().name,
        ssh::task().name, // deps: config
        tmux::task().name,
        vim::task().name,        // deps: config,pip; takes over the terminal
        vscodejson::task().name, // deps: config
        vscode::task().name,     // deps: config,vscodejson
        zsh::task().name,        // deps: profile,zshenv,zshlogin,zshprofile,zshrc,brewbundle
        windows::task().name,
    ]
}

fn mapping() -> HashMap<String, Task> {
    let mut map = HashMap::<String, Task>::new();
    map.insert(String::from("atlantis"), atlantis::task());
    map.insert(String::from("bash"), bash::task());
    map.insert(String::from("bazel"), bazel::task());
    map.insert(String::from("brew"), brew::task());
    map.insert(String::from("dep"), dep::task());
    map.insert(String::from("dotfiles"), dotfiles::task());
    map.insert(String::from("git"), git::task());
    map.insert(String::from("gitleaks"), gitleaks::task());
    map.insert(String::from("gitsizer"), gitsizer::task());
    map.insert(String::from("goget"), goget::task());
    map.insert(String::from("golang"), golang::task());
    map.insert(String::from("googlecloudsdk"), googlecloudsdk::task());
    map.insert(String::from("hadolint"), hadolint::task());
    map.insert(String::from("jq"), jq::task());
    map.insert(String::from("macos"), macos::task());
    map.insert(String::from("minikube"), minikube::task());
    map.insert(String::from("nodejs"), nodejs::task());
    map.insert(String::from("npm"), npm::task());
    map.insert(String::from("pip"), pip::task());
    map.insert(String::from("rust"), rust::task());
    map.insert(String::from("rustc"), rustc::task());
    map.insert(String::from("rustup"), rustup::task());
    map.insert(String::from("shfmt"), shfmt::task());
    map.insert(String::from("skaffold"), skaffold::task());
    map.insert(String::from("ssh"), ssh::task());
    map.insert(String::from("tmux"), tmux::task());
    map.insert(String::from("vale"), vale::task());
    map.insert(String::from("vim"), vim::task());
    map.insert(String::from("vscode"), vscode::task());
    map.insert(String::from("vscodejson"), vscodejson::task());
    map.insert(String::from("windows"), windows::task());
    map.insert(String::from("yq"), yq::task());
    map.insert(String::from("zsh"), zsh::task());
    map
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    #[test]
    fn sequence_lists_all_tasks() {
        let seq = sequence();

        let entries = fs::read_dir(PathBuf::from("src/tasks")).expect("must read");
        let mut count = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
                    .replace(".rs", "");
                if name == "local" || name == "mod" {
                    continue;
                }
                count += 1;
                if !seq.contains(&name) {
                    assert_eq!("", name);
                }

                assert!(seq.contains(&name));
            }
        }
        assert_eq!(count, seq.len());
    }

    #[test]
    fn mapping_maps_all_tasks() {
        let tasks = mapping();
        let seq = sequence();

        assert_eq!(seq.len(), tasks.len());
        for s in seq {
            assert!(tasks.contains_key(&s));
        }
    }
}
