extern crate cabot;
extern crate clap;
extern crate dirs;
extern crate inflector;
extern crate libflate;
extern crate mktemp;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tar;
extern crate textwrap;
extern crate toml;
extern crate which;
extern crate zip;

use clap::{App, SubCommand};

mod lib {
    pub mod env;
    pub mod ghratask;
    pub mod ghrtask;
    pub mod python;
    pub mod version;
}
mod tasks;
mod utils {
    pub mod archive;
    pub mod env;
    pub mod fs;
    pub mod git;
    pub mod github;
    pub mod golang;
    pub mod http;
    pub mod nodejs;
    pub mod process;
    pub mod ssh;
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("sync").about("install / update my settings on this computer"),
        ).subcommand(SubCommand::with_name("update").about("update packages on this computer"))
        .subcommand(SubCommand::with_name("env").about("export generated environment variables"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("env") {
        tasks::env();
        return;
    }

    if let Some(_matches) = matches.subcommand_matches("sync") {
        println!("syncing...");
        tasks::sync();
        return;
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!("updating...");
        tasks::update();
        return;
    }
}
