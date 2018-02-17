use std::path::Path;

use utils;

pub fn sync() {
    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/my_configs.vim"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".vim_runtime/my_configs.vim"));

    utils::fs::symbolic_link(&src, &dest);
}
