use std::collections::HashMap;

use crate::lib::{env::Exports, task::Task};

mod alacritty;
mod atlantis;
mod atom;
mod bash;
mod bazel;
mod brew;
mod brewbundle;
mod brewfile;
mod dep;
mod dotfiles;
mod fccache;
mod fira;
mod firacode;
mod git;
mod gitleaks;
mod gitsizer;
mod goget;
mod golang;
mod googlecloudsdk;
mod hack;
mod hadolint;
mod hasklig;
mod hyper;
mod inter;
mod jq;
mod local;
mod macos;
mod minikube;
mod nodejs;
mod npm;
mod overpass;
mod pip;
mod psql;
mod publicsans;
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
        brewfile::task().name, // deps: config
        brewbundle::task().name,
        brew::task().name, // deps: brewfile, brewbundle
        golang::task().name,
        goget::task().name, // deps: config,golang
        nodejs::task().name,
        npm::task().name, // deps: config,nodejs
        pip::task().name,
        rustup::task().name,
        rustc::task().name, // deps: rustup
        rust::task().name,  // deps: config,rustc
        // fonts
        fira::task().name,
        firacode::task().name,
        hack::task().name,
        hasklig::task().name,
        inter::task().name,
        overpass::task().name,
        publicsans::task().name,
        fccache::task().name, // deps: all other font tasks
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
        alacritty::task().name, // deps: config
        atom::task().name,
        bash::task().name, // deps: config
        git::task().name,  // deps: nodejs/npm
        googlecloudsdk::task().name,
        hyper::task().name, // deps: config
        macos::task().name,
        psql::task().name,   // deps: config
        ssh::task().name,    // deps: config
        tmux::task().name,   // deps: config,brewbundle
        vim::task().name,    // deps: config,pip; takes over the terminal
        vscode::task().name, // deps: config
        zsh::task().name,    // deps: config,brewbundle
        windows::task().name,
    ]
}

fn mapping() -> HashMap<String, Task> {
    let mut map = HashMap::<String, Task>::new();
    map.insert(String::from("alacritty"), alacritty::task());
    map.insert(String::from("atlantis"), atlantis::task());
    map.insert(String::from("atom"), atom::task());
    map.insert(String::from("bash"), bash::task());
    map.insert(String::from("bazel"), bazel::task());
    map.insert(String::from("brew"), brew::task());
    map.insert(String::from("brewbundle"), brewbundle::task());
    map.insert(String::from("brewfile"), brewfile::task());
    map.insert(String::from("dep"), dep::task());
    map.insert(String::from("dotfiles"), dotfiles::task());
    map.insert(String::from("fccache"), fccache::task());
    map.insert(String::from("fira"), fira::task());
    map.insert(String::from("firacode"), firacode::task());
    map.insert(String::from("git"), git::task());
    map.insert(String::from("gitleaks"), gitleaks::task());
    map.insert(String::from("gitsizer"), gitsizer::task());
    map.insert(String::from("goget"), goget::task());
    map.insert(String::from("golang"), golang::task());
    map.insert(String::from("googlecloudsdk"), googlecloudsdk::task());
    map.insert(String::from("hack"), hack::task());
    map.insert(String::from("hadolint"), hadolint::task());
    map.insert(String::from("hasklig"), hasklig::task());
    map.insert(String::from("hyper"), hyper::task());
    map.insert(String::from("inter"), inter::task());
    map.insert(String::from("jq"), jq::task());
    map.insert(String::from("macos"), macos::task());
    map.insert(String::from("minikube"), minikube::task());
    map.insert(String::from("nodejs"), nodejs::task());
    map.insert(String::from("npm"), npm::task());
    map.insert(String::from("overpass"), overpass::task());
    map.insert(String::from("pip"), pip::task());
    map.insert(String::from("psql"), psql::task());
    map.insert(String::from("publicsans"), publicsans::task());
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
