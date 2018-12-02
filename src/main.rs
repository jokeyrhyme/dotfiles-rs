extern crate cabot;
extern crate clap;
extern crate colored;
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

use std::env::var;

use clap::{App, SubCommand};

mod lib {
    pub mod cargo;
    pub mod env;
    pub mod favourites;
    pub mod ghratask;
    pub mod ghrtask;
    pub mod goget;
    pub mod python;
    pub mod rust;
    pub mod ssh;
    pub mod task;
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

use crate::lib::env::Shell;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("all")
                .about("sync / update my settings and packages on this computer"),
        ).subcommand(SubCommand::with_name("env").about("export generated environment variables"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("all") {
        tasks::all();
        return;
    }

    if let Some(_matches) = matches.subcommand_matches("env") {
        let exports = tasks::env();
        let shell = var("SHELL").unwrap_or_default();
        println!("{}", exports.to_shell(Shell::from(shell.as_str())));
        return;
    }
}
