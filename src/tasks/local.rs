use crate::lib::env::Exports;
use crate::utils::env::home_dir;

pub fn env(mut exports: Exports) -> Exports {
    let dir = home_dir().join(".local").join("bin");
    if !exports.path.contains(&dir) {
        let mut paths = vec![dir];
        paths.append(&mut exports.path);
        exports.path = paths;
    }
    exports
}
