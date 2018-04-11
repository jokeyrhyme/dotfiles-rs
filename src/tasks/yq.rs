use std;
use std::path::Path;

use utils;
use utils::github::{Asset, Release};
use utils::golang::{arch, os};

pub fn sync() {
    println!("pkg: yq: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"mikefarah", &"yq") {
            Ok(r) => r,
            Err(error) => {
                println!("error: pkg: yq: {}", error);
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

    println!("pkg: yq: updating ...");

    match utils::process::command_output("yq", &["--version"]) {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

            let release = match utils::github::latest_release(&"mikefarah", &"yq") {
                Ok(r) => r,
                Err(error) => {
                    println!("error: pkg: yq: {}", error);
                    return;
                }
            };

            {
                // installed stdout: yaml version 1.14.1
                let installed = stdout.trim_left_matches(|c: char| !c.is_digit(10)).trim();
                let latest = release.tag_name.trim_left_matches("v").trim();

                println!("pkg: yq: current={} latest={}", &installed, &latest);

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
            println!("pkg: yq: no asset matches OS and ARCH");
            return;
        }
    };

    println!("pkg: yq: installing ...");

    let bin_path = utils::env::home_dir().join(Path::new(".local/bin/yq"));
    utils::github::download_release_asset(asset, &bin_path);
}

fn is_installed() -> bool {
    match utils::process::command_output("yq", &["--version"]) {
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
            #[cfg(windows)]
            let name = format!("yq_{}_{}.exe", os(), arch());
            #[cfg(not(windows))]
            let name = format!("yq_{}_{}", os(), arch());

            if asset.name == name {
                Some(asset)
            } else {
                None
            }
        })
        .next();
}