mod alacritty;
mod atom;
mod hyper;
mod psql;
mod rust;
mod ssh;
mod tmux;
mod vim;
mod vscode;

pub fn sync() {
    alacritty::sync();
    atom::sync();
    hyper::sync();
    psql::sync();
    rust::sync();
    ssh::sync();
    tmux::sync();
    vim::sync();
    vscode::sync();
}

pub fn update() {
    alacritty::update();
    atom::update();
    hyper::update();
    psql::update();
    rust::update();
    ssh::update();
    tmux::update();
    vim::update();
    vscode::update();
}
