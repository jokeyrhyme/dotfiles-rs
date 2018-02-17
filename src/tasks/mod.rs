mod alacritty;
mod hyper;
mod psql;
mod ssh;
mod tmux;
mod vim;
mod vscode;

pub fn sync() {
    alacritty::sync();
    hyper::sync();
    psql::sync();
    ssh::sync();
    tmux::sync();
    vim::sync();
    vscode::sync();
}
