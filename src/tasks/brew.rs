use std::{self, path::PathBuf};

use crate::lib::{brew::brew_prefix, env::Exports};

pub fn env(mut exports: Exports) -> Exports {
    match brew_prefix() {
        Some(prefix) => {
            let mut paths: Vec<PathBuf> = vec!["bin", "sbin"]
                .iter()
                .filter_map(|b| {
                    let dir = prefix.join(b);
                    if exports.path.contains(&dir) {
                        None
                    } else {
                        Some(dir)
                    }
                })
                .collect();
            paths.append(&mut exports.path);
            exports.path = paths;

            // TODO: parse and export the output from `brew shellenv`
        }
        None => {}
    }
    exports
}
