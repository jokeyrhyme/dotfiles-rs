extern crate clap;
use clap::{App, SubCommand};

mod tasks;
mod utils {
    pub mod env;
    pub mod fs;
    pub mod strings;
}

fn main() {
    let matches = App::new("jokeyrhyme-dotfiles")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("sync"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sync") {
        println!("syncing...");
        tasks::sync();
    }
}
