use std::path::Path;

use utils;

pub fn sync() {
    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/tmux.conf"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".tmux.conf"));

    utils::fs::symbolic_link(&src, &dest);
}
