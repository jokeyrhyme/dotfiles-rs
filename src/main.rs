extern crate clap;
use clap::{App, SubCommand};

mod tasks;
mod utils {
    pub mod env;
    pub mod fs;
    pub mod strings;
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("sync").about(
            "install / update my settings on this computer",
        ))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sync") {
        println!("syncing...");
        tasks::sync();
    }
}
