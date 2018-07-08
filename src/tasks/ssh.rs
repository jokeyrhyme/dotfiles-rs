use std::{fs, str};

use regex;

use utils::{self, ssh::Config};

pub fn sync() {
    if !utils::ssh::has_ssh() {
        return;
    }

    println!("ssh: syncing ...");

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

    let version = ssh_version();
    let do_blacklist = is_blacklist_supported(&version);

    let mut config = Config::from(target.as_str()) | Config::from(source.as_str());

    let ciphers: Vec<String> = supported_ssh_ciphers().iter().filter_map(|cipher| {
        if !do_blacklist && !is_weak_cipher(cipher) {
            return Some(cipher.clone());
        }
        if do_blacklist && is_weak_cipher(cipher) {
            return Some(format!("-{}", cipher));
        }
        None
    }).collect();
    if ciphers.len() > 0 {
        config.Ciphers = Some(ciphers);
    }

    let kexs: Vec<String> = supported_ssh_kexs().iter().filter_map(|kex| {
        if !do_blacklist && !is_weak_kex(kex) {
            return Some(kex.clone());
        }
        if do_blacklist && is_weak_kex(kex) {
            return Some(format!("-{}", kex));
        }
        None
    }).collect();
    if kexs.len() > 0 {
        config.KexAlgorithms = Some(kexs);
    }

    let macs: Vec<String> = supported_ssh_macs().iter().filter_map(|mac| {
        if !do_blacklist && !is_weak_mac(mac) {
            return Some(mac.clone());
        }
        if do_blacklist && is_weak_mac(mac) {
            return Some(format!("-{}", mac));
        }
        None
    }).collect();
    if macs.len() > 0 {
        config.MACs = Some(macs);
    }

    match fs::create_dir_all(&target_path.parent().unwrap()) {
        Ok(()) => match fs::write(&target_path, String::from(&config)) {
            Ok(()) => {}
            Err(error) => {
                println!("error: ssh: unable to write config: {}", error);
            }
        },
        Err(error) => {
            println!("error: ssh: unable to create ~/.ssh: {}", error);
        }
    };
}

pub fn update() {}

fn is_blacklist_supported(ssh_version: &str) -> bool {
    let re = regex::Regex::new(r"OpenSSH_(\d+\.\d+)").unwrap();
    for caps in re.captures_iter(ssh_version) {
        let version = caps.get(1).unwrap().as_str();
        match version.parse::<f32>() {
            Ok(v) => {
                // OpenSSH 7.5 supports blacklists, 7.4 and older doesn't
                return v >= 7.5;
            }
            Err(_) => {}
        }
    }
    true
}

fn is_weak_cipher(cipher: &str) -> bool {
    let cbc_re = regex::Regex::new(r"\bcbc\b").unwrap();
    let rc4_re = regex::Regex::new(r"\b(arcfour|rc4)").unwrap();
    // no ending word-boundary: need to catch "arcfour" and "arcfour128"

    // CBC ciphers are weak: https://access.redhat.com/solutions/420283
    // RC4 ciphers are weak: https://en.wikipedia.org/wiki/RC4#Security
    cbc_re.is_match(cipher) || rc4_re.is_match(cipher)
}

fn is_weak_kex(kex: &str) -> bool {
    let sha1_re = regex::Regex::new(r"\bsha1\b").unwrap();

    // SHA1 is weak: https://en.wikipedia.org/wiki/SHA-1
    sha1_re.is_match(kex)
}

fn is_weak_mac(mac: &str) -> bool {
    let bits96_re = regex::Regex::new(r"\b96\b").unwrap();
    let md5_re = regex::Regex::new(r"\bmd5\b").unwrap();
    let sha1_re = regex::Regex::new(r"\bsha1\b").unwrap();

    // 96-bit algorithms are weak: https://access.redhat.com/solutions/420283
    // MD5 is weak: https://access.redhat.com/solutions/420283
    // SHA1 is weak: https://en.wikipedia.org/wiki/SHA-1
    bits96_re.is_match(mac) || md5_re.is_match(mac) || sha1_re.is_match(mac)
}

fn ssh_version() -> String {
    match utils::process::command_output("ssh", &["-V"]) {
        Ok(output) => String::from_utf8(output.stderr).unwrap_or_default(),
        Err(_) => String::from(""),
    }
}

fn supported_ssh_ciphers() -> Vec<String> {
    match utils::process::command_output("ssh", &["-Q", "cipher"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            stdout
                .lines()
                .map(|l| String::from(l.trim()))
                .collect()
        }
        Err(_error) => Vec::<String>::new(),
    }
}

fn supported_ssh_kexs() -> Vec<String> {
    match utils::process::command_output("ssh", &["-Q", "kex"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            stdout
                .lines()
                .map(|l| String::from(l.trim()))
                .collect()
        }
        Err(_error) => Vec::<String>::new(),
    }
}

fn supported_ssh_macs() -> Vec<String> {
    match utils::process::command_output("ssh", &["-Q", "mac"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            stdout
                .lines()
                .map(|l| String::from(l.trim()))
                .collect()
        }
        Err(_error) => Vec::<String>::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use which;

    #[test]
    fn is_blacklist_supported_by_versions() {
        let old_version = "OpenSSH_7.4p1 Debian-10+deb9u3, OpenSSL 1.0.2l  25 May 2017";
        assert_eq!(is_blacklist_supported(&old_version), false);
        assert_eq!(is_blacklist_supported(&""), true);
    }

    #[test]
    fn ssh_version_is_not_blank() {
        match which::which("ssh") {
            Ok(_) => {
                let version = ssh_version();
                assert!(version.len() > 0);
            }
            Err(_) => {}
        }
    }

}
