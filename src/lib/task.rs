use std::{fmt, io, result};

use colored::*;
use subprocess;

use crate::utils::github;

#[derive(Debug)]
pub enum Error {
    GitHubError(String, github::GitHubError),
    IoError(String, io::Error),
    NoTagsError,
    PopenError(String, subprocess::PopenError),
}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::GitHubError(msg, cause) => {
                write!(f, "{}", format!("{}: {:?}", msg, cause).red())
            }
            Error::IoError(msg, cause) => write!(f, "{}", format!("{}: {:?}", msg, cause).red()),
            Error::NoTagsError => write!(f, "{}", "NoTagsError".red()),
            Error::PopenError(msg, cause) => write!(f, "{}", format!("{}: {:?}", msg, cause).red()),
        }
    }
}
impl From<github::GitHubError> for Error {
    fn from(cause: github::GitHubError) -> Error {
        Error::GitHubError(String::new(), cause)
    }
}
impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::IoError(String::new(), cause)
    }
}
impl From<subprocess::PopenError> for Error {
    fn from(cause: subprocess::PopenError) -> Error {
        Error::PopenError(String::new(), cause)
    }
}

pub type Result = result::Result<Status, Error>;

#[derive(Debug, PartialEq)]
pub enum Status {
    Done,           // started, and finished
    NotImplemented, // never starting
    Skipped,        // never finishing

    Changed(String, String), // finished, with details
    NoChange(String),        // finished, with details

                             // TODO: support async with below statuses
                             // InProgress,              // started, not finished yet
                             // Pending,                 // has not started yet
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Status::Done => write!(f, "{}", "done".green()),
            Status::NotImplemented => write!(f, "{}", "not implemented".dimmed()),
            Status::Skipped => write!(f, "{}", "skipped".blue()),

            Status::Changed(old, new) => {
                write!(f, "{}", format!("changed '{}' -> '{}'", old, new).yellow())
            }
            Status::NoChange(old) => write!(f, "{}", format!("'{}' -> no change", old).green()),
        }
    }
}

pub struct Task {
    pub name: String,
    pub sync: fn() -> Result,
    pub update: fn() -> Result,
}
impl Task {
    pub fn sync_then_update(&self) {
        match (self.sync)() {
            Ok(status) => println!("{}: sync: {}", self.name, status),
            Err(error) => println!("{}: sync error: {:?}", self.name, error),
        }
        // TODO: ensure we can trust the accuracy of `sync()` results
        // TODO: only call `update()` when `sync()` result suggests it is needed
        match (self.update)() {
            Ok(status) => println!("{}: update: {}", self.name, status),
            Err(error) => println!("{}: update error: {:?}", self.name, error),
        }
    }
}
impl Default for Task {
    fn default() -> Task {
        Task {
            name: String::from("unknown"),
            sync: || Ok(Status::NotImplemented),
            update: || Ok(Status::NotImplemented),
        }
    }
}
