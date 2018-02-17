use std::path::Path;

use utils;

pub fn sync() {
    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/vscode.json"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".config/Code/User/settings.json"));

    utils::fs::symbolic_link(&src, &dest);
}
