use std;

use utils;
use utils::github::{Asset, Release};
use utils::golang::{arch, os};

pub fn sync() {
    println!("skaffold: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"GoogleCloudPlatform", &"skaffold") {
            Ok(r) => r,
            Err(error) => {
                println!("error: skaffold: {}", error);
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

    println!("skaffold: updating ...");

    match utils::process::command_output("skaffold", &["version"]) {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

            match utils::github::release_versus_current(
                &stdout,
                &"GoogleCloudPlatform",
                &"skaffold",
            ) {
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
            println!("skaffold: no asset matches OS and ARCH");
            return;
        }
    };

    println!("skaffold: installing ...");

    #[cfg(windows)]
    let bin_path = utils::env::home_dir().join(".local/bin/skaffold.exe");
    #[cfg(not(windows))]
    let bin_path = utils::env::home_dir().join(".local/bin/skaffold");

    utils::github::download_release_asset(asset, &bin_path);
}

fn is_installed() -> bool {
    match utils::process::command_output("skaffold", &["version"]) {
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
            let name = format!("skaffold-{}-{}.exe", os(), arch());
            #[cfg(not(windows))]
            let name = format!("skaffold-{}-{}", os(), arch());

            asset.name == name
        })
        .next();
}
