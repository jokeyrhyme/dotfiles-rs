use std;
use std::env::consts::{ARCH, OS};
use std::path::Path;

use utils;
use utils::github::{Asset, Release};

pub fn sync() {
    println!("pkg: skaffold: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"GoogleCloudPlatform", &"skaffold") {
            Ok(r) => r,
            Err(error) => {
                println!("error: pkg: skaffold: {}", error);
                return;
            }
        };
        install_release_asset(&release);
    }
}

pub fn update() {
    if !is_installed() {
        return;
    }

    println!("pkg: skaffold: updating ...");

    match utils::process::command_output("skaffold", &["version"]) {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

            let release =
                match utils::github::latest_release(&"GoogleCloudPlatform", &"skaffold") {
                    Ok(r) => r,
                    Err(error) => {
                        println!("error: pkg: skaffold: {}", error);
                        return;
                    }
                };

            {
                let installed = stdout.trim_left_matches("v").trim();
                let latest = release.tag_name.trim_left_matches("v").trim();

                println!("pkg: skaffold: current={} latest={}", &installed, &latest);

                if installed == latest {
                    return;
                }
            }

            install_release_asset(&release);
        }
        Err(_error) => {}
    };
}

fn install_release_asset(release: &Release) {
    let asset = match latest_asset(&release) {
        Some(a) => a,
        None => {
            println!("pkg: skaffold: no asset matches OS and ARCH");
            return;
        }
    };

    println!("pkg: skaffold: installing ...");

    let bin_path = utils::env::home_dir().join(Path::new(".local/bin/skaffold"));
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
        .filter_map(|asset| {
            let arch = if ARCH == "x86_64" { "amd64" } else { ARCH };
            let os = if OS == "macos" { "darwin" } else { OS };

            #[cfg(windows)]
            let name = format!("skaffold-{}-{}.exe", os, arch);
            #[cfg(not(windows))]
            let name = format!("skaffold-{}-{}", os, arch);

            if asset.name == name {
                Some(asset)
            } else {
                None
            }
        })
        .next();
}