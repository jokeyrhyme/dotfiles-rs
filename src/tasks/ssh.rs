use std::{fs, str};
use std::path::Path;

use regex;

use utils::{self, ssh::Config};

pub fn sync() {
    if !utils::ssh::has_ssh() {
        return;
    }

    println!("pkg: ssh: syncing ...");

    let bits96_re = regex::Regex::new(r"\b96\b").unwrap();
    let cbc_re = regex::Regex::new(r"\bcbc\b").unwrap();
    let md5_re = regex::Regex::new(r"\bmd5\b").unwrap();
    let rc4_re = regex::Regex::new(r"\b(arcfour|rc4)\b").unwrap();
    let sha1_re = regex::Regex::new(r"\bsha1\b").unwrap();

    // TODO: ensure contents of ~/.dotfiles/config/ssh is present in ~/.ssh/config

    let source_path = utils::env::home_dir()
        .join(".dotfiles")
        .join("config")
        .join("ssh");
    let source = match fs::read_to_string(&source_path) {
        Ok(s) => s,
        Err(_error) => String::from(""),
    };

    let target_path = utils::env::home_dir().join(".ssh").join("config");
    let target = match fs::read_to_string(&target_path) {
        Ok(s) => s,
        Err(_error) => String::from(""),
    };

    let mut config = Config::from(target.as_str()) | Config::from(source.as_str());

    match utils::process::command_output("ssh", &["-Q", "cipher"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            config.Ciphers = Some(stdout.lines().filter_map(|l| {
                let cipher = l.trim();
                if cbc_re.is_match(cipher) {
                    // CBC ciphers are weak: https://access.redhat.com/solutions/420283
                    return Some(format!("-{}", cipher));
                }
                if rc4_re.is_match(cipher) {
                    // RC4 ciphers are weak: https://en.wikipedia.org/wiki/RC4#Security
                    return Some(format!("-{}", cipher));
                }
                None
            }).collect());
        }
        Err(_error) => {}
    }

    match utils::process::command_output("ssh", &["-Q", "kex"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            config.KexAlgorithms = Some(stdout.lines().filter_map(|l| {
                let kex = l.trim();
                if sha1_re.is_match(kex) {
                    // SHA1 is weak: https://en.wikipedia.org/wiki/SHA-1
                    return Some(format!("-{}", kex));
                }
                None
            }).collect());
        }
        Err(_error) => {}
    }

    match utils::process::command_output("ssh", &["-Q", "mac"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            config.MACs = Some(stdout.lines().filter_map(|l| {
                let mac = l.trim();
                if bits96_re.is_match(mac) {
                    // 96-bit algorithms are weak: https://access.redhat.com/solutions/420283
                    return Some(format!("-{}", mac));
                }
                if md5_re.is_match(mac) {
                    // MD5 is weak: https://access.redhat.com/solutions/420283
                    return Some(format!("-{}", mac));
                }
                if sha1_re.is_match(mac) {
                    // SHA1 is weak: https://en.wikipedia.org/wiki/SHA-1
                    return Some(format!("-{}", mac));
                }
                None
            }).collect());
        }
        Err(_error) => {}
    }

    match fs::write(&target_path, String::from(&config)) {
        Ok(()) => {}
        Err(error) => {
            println!("error: pkg: ssh: unable to write config: {}", error);
        }
    }
}

pub fn update() {}