use utils;

pub fn has_npm() -> bool {
    match utils::process::command_output("npm", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}

pub fn has_npx() -> bool {
    match utils::process::command_output("npx", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}

pub fn has_yarn() -> bool {
    match utils::process::command_output("yarn", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}