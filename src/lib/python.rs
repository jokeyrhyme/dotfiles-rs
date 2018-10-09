use std::{path::PathBuf, process::Output};

use which;

use utils;

const PYTHONS: &[&str] = &["python2", "python"];
const VERSION_ARGS: &[&str] = &["--version"];

pub fn which_v2() -> Option<PathBuf> {
    for python in PYTHONS {
        let found = match which::which(&python) {
            Ok(exe) => exe,
            Err(_) => continue,
        };
        let version = match utils::process::command_output(&found, VERSION_ARGS) {
            Ok(output) => extract_version(output),
            Err(_error) => continue,
        };
        if version.starts_with("2.") {
            return Some(found);
        }
    }
    None
}

fn extract_version(o: Output) -> String {
    let stdout = String::from_utf8(o.stdout).unwrap_or_default();
    let first = stdout.lines().next().unwrap_or_default().trim();
    first.replace("Python ", "")
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use utils::process::command_output;

    use super::*;

    #[test]
    fn which_v2_does_not_find_v3() {
        let exe = match which_v2() {
            Some(e) => e,
            None => {
                // skip test if no `python` found
                return;
            }
        };
        let output = match command_output(exe, VERSION_ARGS) {
            Ok(v) => v,
            Err(_error) => {
                assert!(false, "cannot run python 2.x we found");
                return;
            }
        };
        let re = Regex::new(&format!(r"^2\.\d+\.\d+.*$")).unwrap();
        assert!(re.is_match(&extract_version(output)));
    }
}
