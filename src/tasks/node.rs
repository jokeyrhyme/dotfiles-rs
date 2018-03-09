use utils;

const ERROR_MSG: &str = "error: node";

pub fn sync() {
    if !utils::node::has_npm() {
        return;
    }
}

pub fn update() {
    if !utils::node::has_npm() {
        return;
    }

    if utils::node::has_npx() {
        // https://www.npmjs.com/package/npm-windows-upgrade
        #[cfg(windows)]
        utils::process::command_spawn_wait(
            "npx",
            &["-q", "npm-windows-upgrade", "--npm-version", "latest"],
        )
            .expect(ERROR_MSG);
    }
}