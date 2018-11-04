use std::{fmt, io, result};

use colored::*;

use utils::github::GitHubError;

#[derive(Debug)]
pub enum Error {
    GitHubError(String, GitHubError),
    IOError(String, io::Error),
}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::GitHubError(msg, cause) => {
                write!(f, "{}", format!("error: {}: {:?}", msg, cause).red())
            }
            Error::IOError(msg, cause) => {
                write!(f, "{}", format!("error: {}: {:?}", msg, cause).red())
            }
        }
    }
}
impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::IOError("".to_string(), cause)
    }
}

pub type Result = result::Result<Status, Error>;

#[derive(Debug)]
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
