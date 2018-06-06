use std;

use utils;
use utils::github::{Asset, Release};
use utils::golang::{arch, os};

pub fn sync() {
    println!("shfmt: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"mvdan", &"sh") {
            Ok(r) => r,
            Err(error) => {
                println!("error: shfmt: {}", error);
                return;
            }
        };
        install_release_asset(release);
    }
}

pub fn update() {
    if !is_installed() {
        return;
    }

    println!("shfmt: updating ...");

    match utils::process::command_output("shfmt", &["--version"]) {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

            match utils::github::release_versus_current(&stdout, &"mvdan", &"sh") {
                Some(r) => install_release_asset(r),
                None => {}
            }
        }
        Err(_error) => {}
    };
}

fn install_release_asset(release: Release) {
    let asset = match latest_asset(&release) {
        Some(a) => a,
        None => {
            println!("shfmt: no asset matches OS and ARCH");
            return;
        }
    };

    println!("shfmt: installing ...");

    #[cfg(windows)]
    let bin_path = utils::env::home_dir().join(".local/bin/shfmt.exe");
    #[cfg(not(windows))]
    let bin_path = utils::env::home_dir().join(".local/bin/shfmt");

    utils::github::download_release_asset(asset, &bin_path);
}

fn is_installed() -> bool {
    match utils::process::command_output("shfmt", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    }
}


fn latest_asset(release: &Release) -> Option<Asset> {
    return release
        .assets
        .to_vec()
        .into_iter()
        .filter(|asset| {
            #[cfg(windows)]
            let name = format!("shfmt_{}_{}_{}.exe", release.tag_name, os(), arch());
            #[cfg(not(windows))]
            let name = format!("shfmt_{}_{}_{}", release.tag_name, os(), arch());

            asset.name == name
        })
        .next();
}
