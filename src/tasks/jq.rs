use std;
use std::env::consts::{ARCH, OS};

use utils;
use utils::github::{Asset, Release};

pub fn sync() {
    println!("jq: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"stedolan", &"jq") {
            Ok(r) => r,
            Err(error) => {
                println!("error: jq: {}", error);
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

    println!("jq: updating ...");

    match utils::process::command_output("jq", &["--version"]) {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

            match utils::github::release_versus_current(&stdout, &"stedolan", &"jq") {
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
            println!("jq: no asset matches OS and ARCH");
            return;
        }
    };

    println!("jq: installing ...");

    #[cfg(windows)]
    let bin_path = utils::env::home_dir().join(".local/bin/jq.exe");
    #[cfg(not(windows))]
    let bin_path = utils::env::home_dir().join(".local/bin/jq");

    utils::github::download_release_asset(asset, &bin_path);
}

fn is_installed() -> bool {
    match utils::process::command_output("jq", &["--version"]) {
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
            let name = format!("jq-{}.exe", os_arch());
            #[cfg(not(windows))]
            let name = format!("jq-{}", os_arch());

            asset.name == name
        })
        .next();
}

// this is unfortunately only true for jq 1.5,
// may need to make this smarter to match all past and future versions
fn os_arch() -> String {
    if ARCH == "x86_64" && OS == "macos" {
        return String::from("osx-amd64");
    }
    format!(
        "{}{}",
        match OS {
            "macos" => "darwin",
            "windows" => "win",
            _ => OS,
        },
        match ARCH {
            "x86_64" => "64",
            "x86" => "32",
            _ => ARCH,
        }
    )
}
