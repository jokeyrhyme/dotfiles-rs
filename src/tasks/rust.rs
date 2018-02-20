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

    let krate = Regex::new(r"^(?P<name>\S+)\sv").unwrap();
    match Command::new("cargo").arg("--version").spawn() {
        Ok(_child) => {
            println!("pkg: rust: updating crates...");

            let output = Command::new("cargo")
                .args(&["install", "--list"])
                .output()
                .expect(ERROR_MSG);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut lines = stdout.lines();

            let krates: Vec<&str> = lines
                .filter_map(|line| match krate.captures(line) {
                    Some(caps) => Some(caps.get(1).unwrap().as_str()),
                    None => None,
                })
                .collect();

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
