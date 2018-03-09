use utils;

const ERROR_MSG: &str = "error: git";

#[derive(Debug, Deserialize)]
struct Config {
    disable: Vec<String>,
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn sync() {
    // TODO: synchronise git settings

    if !utils::git::has_git() {
        return;
    }

    if !has_npx() {
        return;
    }

    // https://www.npmjs.com/package/npm-merge-driver
    utils::process::command_spawn_wait("npx", &["-q", "npm-merge-driver", "install", "--global"])
        .expect(ERROR_MSG);

    if !has_yarn() {
        return;
    }

    // https://www.npmjs.com/package/npm-merge-driver
    utils::process::command_spawn_wait(
        "npx",
        &[
            "-q",
            "npm-merge-driver",
            "install",
            "--global",
            "--driver-name",
            "yarn-merge-driver",
            "--driver",
            r#""npx npm-merge-driver merge %A %O %B %P -c yarn""#,
            "--files",
            "yarn.lock",
        ],
    ).expect(ERROR_MSG);
}

pub fn update() {}

fn has_npx() -> bool {
    match utils::process::command_output("npx", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}

fn has_yarn() -> bool {
    match utils::process::command_output("yarn", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}