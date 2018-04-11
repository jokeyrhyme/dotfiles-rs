mod alacritty;
mod atom;
mod dotfiles;
mod git;
mod hyper;
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
    #[cfg(windows)] windows::sync();

    alacritty::sync();
    atom::sync();
    git::sync();
    hyper::sync();
    nodejs::sync();
    psql::sync();
    rust::sync();
    shfmt::sync();
    skaffold::sync();
    ssh::sync();
    tmux::sync();
    vim::sync();
    vscode::sync();
    yq::sync();
}

pub fn update() {
    // must be first
    dotfiles::update();
    #[cfg(windows)] windows::update();

    alacritty::update();
    atom::update();
    git::update();
    hyper::update();
    nodejs::update();
    psql::update();
    rust::update();
    shfmt::update();
    skaffold::update();
    ssh::update();
    tmux::update();
    vim::update();
    vscode::update();
    yq::update();
}
