use std::process::Command;

use regex::Regex;

pub fn sync() {}

pub fn update() {
    const ERROR_MSG: &str = "error: rust: update";
    match Command::new("rustup").arg("--version").spawn() {
        Ok(_child) => {
            println!("pkg: rust: updating to latest stable...");

            Command::new("rustup")
                .args(&["override", "set", "stable"])
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
            Command::new("rustup")
                .args(&["update", "stable"])
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
        }
        Err(_error) => {
            // rustup probably not installed, skip!
        }
    }

    match Command::new("cargo").arg("--version").spawn() {
        Ok(_child) => {
            println!("pkg: rust: updating crates...");

            let output = Command::new("cargo")
                .args(&["install", "--list"])
                .output()
                .expect(ERROR_MSG);
            let stdout = String::from_utf8_lossy(&output.stdout);

            let krates = parse_installed(&stdout);

            let mut install_args = vec!["install", "--force"];
            install_args.extend(krates);

            Command::new("cargo")
                .args(install_args)
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
        }
        Err(_error) => {
            // cargo probably not installed, skip!
        }
    }
}

fn parse_installed(stdout: &str) -> Vec<&str> {
    let krate = Regex::new(r"^(?P<name>\S+)\sv\d+").unwrap();
    let lines = stdout.lines();
    return lines
        .filter_map(|line| match krate.captures(line) {
            Some(caps) => Some(caps.get(1).unwrap().as_str()),
            None => None,
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_installed() {
        let input = "
racer v2.0.12:
    racer
rustfmt v0.10.0:
    cargo-fmt
    rustfmt
rustsym v0.3.2:
    rustsym
";
        assert_eq!(parse_installed(input), vec!["racer", "rustfmt", "rustsym"]);
    }
}

