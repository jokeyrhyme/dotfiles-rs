use std::{self, path::PathBuf};

use crate::{
    lib::{
        brew,
        env::Exports,
        task::{self, Status, Task},
    },
    utils::fs,
};

pub fn env(mut exports: Exports) -> Exports {
    if let Some(prefix) = brew::brew_prefix() {
        // TODO: consider parsing the output from `brew shellenv`

        // we add homebrew to the end of ...PATH,
        // as we want to treat it more like a cross-distribution fallback,
        // rather than the primary source of packages (for now, at least)

        exports.info_path.push(prefix.join("info"));
        exports.man_path.push(prefix.join("man"));

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
        exports.path.append(&mut paths);
    }
    exports
}

pub fn task() -> Task {
    Task {
        name: String::from("brew"),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    if !brew::has_brew() {
        return Ok(Status::Skipped);
    }

    brew::brew(&["cleanup"])?;

    Ok(Status::Done)
}

fn update(_: Status) -> task::Result {
    if !brew::has_brew() {
        return Ok(Status::Skipped);
    }

    let before = brew::brew_output(&["--version"])?;

    // this may fail if a tap cannot be accessed,
    // and this is fine, hence ignoring this error
    brew::brew(&["update"]).unwrap_or(());

    brew::brew(&["upgrade"])?;

    // can't get here otherwise, so this `.expect()` is fine
    let prefix = brew::brew_prefix().expect("no $HOMEBREW_PREFIX");

    // zsh complains if certains paths are not secure enough
    // chmod g-w /usr/local/share/zsh /usr/local/share/zsh/site-functions
    fs::set_executable(prefix.join("share/zsh"))?;
    fs::set_executable(prefix.join("share/zsh/site-functions"))?;

    let after = brew::brew_output(&["--version"])?;

    if before == after {
        Ok(Status::NoChange(before))
    } else {
        Ok(Status::Changed(before, after))
    }
    // TODO: separate task/status for packages
}
