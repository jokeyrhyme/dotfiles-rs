use std::{env::consts::EXE_SUFFIX, io, path::PathBuf};

use serde_derive::Deserialize;

use crate::{
    lib::favourites::Favourites,
    utils::{fs::delete_if_exists, golang::gopath, process::command_spawn_wait},
};

#[derive(Debug, Deserialize)]
pub struct GoGetFavourites {
    install: Vec<String>,
    uninstall: Vec<String>,
}

impl Default for GoGetFavourites {
    fn default() -> GoGetFavourites {
        GoGetFavourites {
            install: Vec::<String>::new(),
            uninstall: Vec::<String>::new(),
        }
    }
}

impl Favourites for GoGetFavourites {
    fn cull(&mut self) -> io::Result<()> {
        for pkg in self.surplus() {
            let p = PathBuf::from(pkg);
            let src = gopath().join("src").join(&p);
            delete_if_exists(src);
            // TODO: cleanup empty ancestor directories, too

            if let Some(f) = p.file_name() {
                let bin =
                    gopath()
                        .join("bin")
                        .join(format!("{}{}", f.to_string_lossy(), EXE_SUFFIX));
                delete_if_exists(bin);
            }
        }
        Ok(())
    }
    fn fill(&mut self) -> io::Result<()> {
        for pkg in self.missing() {
            command_spawn_wait("go", &["get", "-u", "-v", &pkg])?;
        }
        Ok(())
    }
    fn found(&self) -> Vec<String> {
        // not really feasible to do this accurately,
        // only for things specifically listed in install/uninstall
        let mut results = Vec::<String>::new();
        for pkg in self.wanted() {
            if pkg_found(pkg.as_str()) {
                results.push(pkg);
            }
        }
        for pkg in self.unwanted() {
            if pkg_found(pkg.as_str()) {
                results.push(pkg);
            }
        }
        results
    }
    fn wanted(&self) -> Vec<String> {
        self.install.clone()
    }
    fn unwanted(&self) -> Vec<String> {
        self.uninstall.clone()
    }
}

fn pkg_found<S>(pkg: S) -> bool
where
    S: AsRef<str>,
{
    let p = PathBuf::from(pkg.as_ref());
    match p.file_name() {
        Some(f) => {
            let bin = gopath()
                .join("bin")
                .join(format!("{}{}", f.to_string_lossy(), EXE_SUFFIX));

            let src = gopath().join("src").join(&p);
            bin.is_file() && src.is_dir()
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use std::env::consts::OS;

    use super::*;

    #[test]
    fn found_does_not_find_missing() {
        let missing_pkg = String::from("github.com/jokeyrhyme/missing-does-not-exist");
        let favs = GoGetFavourites {
            install: vec![],
            uninstall: vec![missing_pkg.clone()],
        };
        assert_eq!(false, favs.found().contains(&missing_pkg));
    }

    #[test]
    fn found_finds_goimports() {
        let goimports_pkg = String::from("golang.org/x/tools/cmd/goimports");
        let favs = GoGetFavourites {
            install: vec![goimports_pkg.clone()],
            uninstall: vec![],
        };

        let bin = gopath().join("bin").join(if OS == "windows" {
            "goimports.exe"
        } else {
            "goimports"
        });

        let src = gopath().join("src").join(goimports_pkg.clone());
        assert_eq!(
            bin.is_file() && src.is_dir(),
            favs.found().contains(&goimports_pkg)
        );
    }
}
