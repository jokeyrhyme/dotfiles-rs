use std::{self, path::PathBuf};

use crate::lib::{brew::brew_exe, env::Exports};

pub fn env(mut exports: Exports) -> Exports {
    match brew_exe() {
        Some(exe) => {
            // these `.expect()`s are fine, we trust `brew_exe()`
            let install_dir = exe
                .parent()
                .expect("brew: no ..")
                .parent()
                .expect("brew: no ..");
            let mut paths: Vec<PathBuf> = vec!["bin", "sbin"]
                .iter()
                .filter_map(|b| {
                    let dir = install_dir.join(b);
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
