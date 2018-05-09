use utils;

pub fn sync() {
    println!("pkg: windows: manually configure %PATH% to include:");

    let bin_path = utils::env::home_dir().join(".local").join("bin");
    println!("- {}", bin_path.display());

    let go_bin_path = utils::env::home_dir().join(".local").join("go").join("bin");
    println!("- {}", go_bin_path.display());

    let node_bin_path = utils::env::home_dir().join(".local").join("node").join(
        "bin",
    );
    println!("- {}", node_bin_path.display());
}

pub fn update() {}
