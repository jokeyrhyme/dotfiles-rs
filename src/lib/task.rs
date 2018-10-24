use std::{fmt, result};

#[derive(Debug)]
pub struct Error {}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task::Error::...")
    }
}

pub type Result = result::Result<Status, Error>;

#[derive(Debug)]
pub enum Status {
    Done,           // started, and finished
    NotImplemented, // never starting
    Skipped,        // never finishing

    // TODO: bubble version information upwards
    // Changed(String, String), // finished, with details
    // NoChange(String),        // finished, with details

    // TODO: support async with below statuses
    // InProgress,              // started, not finished yet
    // Pending,                 // has not started yet
}

pub struct Task {
    pub sync: fn() -> Result,
    pub update: fn() -> Result,
}
