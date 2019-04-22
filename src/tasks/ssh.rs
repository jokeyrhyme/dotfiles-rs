use std::{fs, str};

use regex;

use crate::lib::task::{self, Status, Task};
use crate::utils::{self, ssh::Config};

pub fn task() -> Task {
    Task {
        name: String::from("ssh"),
        sync,
        ..Default::default()
    }
}

fn is_blacklist_supported<S>(ssh_version: S) -> bool
where
    S: AsRef<str>,
{
    let re = regex::Regex::new(r"OpenSSH_(\d+\.\d+)").unwrap();
    for caps in re.captures_iter(&ssh_version.as_ref()) {
        let version = caps.get(1).unwrap().as_str();
        if let Ok(v) = version.parse::<f32>() {
            // OpenSSH 7.5 supports blacklists, 7.4 and older doesn't
            return v >= 7.5;
        }
    }
    true
}

fn is_weak_cipher<S>(cipher: S) -> bool
where
    S: AsRef<str>,
{
    let c = cipher.as_ref();
    let cbc_re = regex::Regex::new(r"\bcbc\b").unwrap();
    let rc4_re = regex::Regex::new(r"\b(arcfour|rc4)").unwrap();
    // no ending word-boundary: need to catch "arcfour" and "arcfour128"

    // CBC ciphers are weak: https://access.redhat.com/solutions/420283
    // RC4 ciphers are weak: https://en.wikipedia.org/wiki/RC4#Security
    cbc_re.is_match(&c) || rc4_re.is_match(&c)
}

fn is_weak_kex<S>(kex: S) -> bool
where
    S: AsRef<str>,
{
    let sha1_re = regex::Regex::new(r"\bsha1\b").unwrap();

    // SHA1 is weak: https://en.wikipedia.org/wiki/SHA-1
    sha1_re.is_match(&kex.as_ref())
}

fn is_weak_mac<S>(mac: S) -> bool
where
    S: AsRef<str>,
{
    let m = mac.as_ref();
    let bits96_re = regex::Regex::new(r"\b96\b").unwrap();
    let md5_re = regex::Regex::new(r"\bmd5\b").unwrap();
    let sha1_re = regex::Regex::new(r"\bsha1\b").unwrap();

    // 96-bit algorithms are weak: https://access.redhat.com/solutions/420283
    // MD5 is weak: https://access.redhat.com/solutions/420283
    // SHA1 is weak: https://en.wikipedia.org/wiki/SHA-1
    bits96_re.is_match(&m) || md5_re.is_match(&m) || sha1_re.is_match(&m)
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
            stdout.lines().map(|l| String::from(l.trim())).collect()
        }
        Err(_error) => Vec::<String>::new(),
    }
}

fn supported_ssh_kexs() -> Vec<String> {
    match utils::process::command_output("ssh", &["-Q", "kex"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            stdout.lines().map(|l| String::from(l.trim())).collect()
        }
        Err(_error) => Vec::<String>::new(),
    }
}

fn supported_ssh_macs() -> Vec<String> {
    match utils::process::command_output("ssh", &["-Q", "mac"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            stdout.lines().map(|l| String::from(l.trim())).collect()
        }
        Err(_error) => Vec::<String>::new(),
    }
}

fn sync() -> task::Result {
    if !utils::ssh::has() {
        return Ok(Status::Skipped);
    }

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
    let do_blacklist = is_blacklist_supported(version);

    let mut config = Config::from(target.as_str()) | Config::from(source.as_str());

    let ciphers: Vec<String> = supported_ssh_ciphers()
        .iter()
        .filter_map(|cipher| {
            if !do_blacklist && !is_weak_cipher(cipher as &str) {
                return Some(cipher.clone());
            }
            if do_blacklist && is_weak_cipher(cipher as &str) {
                return Some(format!("-{}", cipher));
            }
            None
        })
        .collect();
    if !ciphers.is_empty() {
        config.Ciphers = Some(ciphers);
    }

    let kexs: Vec<String> = supported_ssh_kexs()
        .iter()
        .filter_map(|kex| {
            if !do_blacklist && !is_weak_kex(kex as &str) {
                return Some(kex.clone());
            }
            if do_blacklist && is_weak_kex(kex as &str) {
                return Some(format!("-{}", kex));
            }
            None
        })
        .collect();
    if !kexs.is_empty() {
        config.KexAlgorithms = Some(kexs);
    }

    let macs: Vec<String> = supported_ssh_macs()
        .iter()
        .filter_map(|mac| {
            if !do_blacklist && !is_weak_mac(mac as &str) {
                return Some(mac.clone());
            }
            if do_blacklist && is_weak_mac(mac as &str) {
                return Some(format!("-{}", mac));
            }
            None
        })
        .collect();
    if !macs.is_empty() {
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

    Ok(Status::Done)
}

#[cfg(test)]
mod tests {
    use super::*;

    use which;

    #[test]
    fn is_blacklist_supported_by_versions() {
        let old_version = "OpenSSH_7.4p1 Debian-10+deb9u3, OpenSSL 1.0.2l  25 May 2017";
        assert_eq!(is_blacklist_supported(old_version), false);
        assert_eq!(is_blacklist_supported(""), true);
    }

    #[test]
    fn ssh_version_is_not_blank() {
        if which::which("ssh").is_ok() {
            let version = ssh_version();
            assert!(!version.is_empty());
        }
    }

}
