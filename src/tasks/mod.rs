mod alacritty;
mod atom;
mod dotfiles;
mod git;
mod hyper;
mod nodejs;
mod psql;
mod rust;
mod ssh;
mod tmux;
mod vim;
mod vscode;

pub fn sync() {
    // must be first
    dotfiles::sync();

    alacritty::sync();
    atom::sync();
    git::sync();
    hyper::sync();
    nodejs::sync();
    psql::sync();
    rust::sync();
    ssh::sync();
    tmux::sync();
    vim::sync();
    vscode::sync();
}

pub fn update() {
    // must be first
    dotfiles::update();

    alacritty::update();
    atom::update();
    git::update();
    hyper::update();
    nodejs::update();
    psql::update();
    rust::update();
    ssh::update();
    tmux::update();
    vim::update();
    vscode::update();
}