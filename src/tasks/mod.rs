mod alacritty;
mod atom;
mod dep;
mod dotfiles;
mod git;
mod golang;
mod hadolint;
mod hyper;
mod jq;
#[cfg(target_os = "macos")]
mod macos;
mod nodejs;
mod psql;
mod rust;
mod shfmt;
mod skaffold;
mod ssh;
mod tmux;
mod vim;
mod vscode;
#[cfg(windows)]
mod windows;
mod yq;

pub fn sync() {
    // must be first
    dotfiles::sync();

    alacritty::sync();
    atom::sync();
    dep::sync();
    git::sync();
    golang::sync();
    hadolint::sync();
    hyper::sync();
    jq::sync();
    #[cfg(target_os = "macos")]
    macos::sync();
    nodejs::sync();
    psql::sync();
    rust::sync();
    shfmt::sync();
    skaffold::sync();
    ssh::sync();
    tmux::sync();
    vim::sync();
    vscode::sync();
    #[cfg(windows)]
    windows::sync();
    yq::sync();
}

pub fn update() {
    // must be first
    dotfiles::update();

    alacritty::update();
    atom::update();
    dep::update();
    git::update();
    golang::update();
    hadolint::update();
    hyper::update();
    jq::update();
    #[cfg(target_os = "macos")]
    macos::update();
    nodejs::update();
    psql::update();
    rust::update();
    shfmt::update();
    skaffold::update();
    ssh::update();
    tmux::update();
    vim::update();
    vscode::update();
    #[cfg(windows)]
    windows::update();
    yq::update();
}
