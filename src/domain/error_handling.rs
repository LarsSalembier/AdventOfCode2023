//! Advent of Code Error Handling
//!
//! This module defines the `ProgramError` enum, which encapsulates the various types of errors
//! in the context of this program.

/// The `ProgramError` enum represents all possible errors that can occur in the
/// context of this program.
#[derive(Debug)]
pub enum ProgramError {
    /// Represents an IO error. It wraps the standard `std::io::Error` type.
    IoError(std::io::Error),

    /// Error related to the input data, containing a message describing the issue.
    InputError(String),

    /// Indicates that a certain problem solution is not yet implemented, containing
    /// a message about the missing problem solution.
    NotYetImplemented(String),
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgramError::IoError(e) => write!(f, "IO error: {}", e),
            ProgramError::InputError(msg) => write!(f, "Input error: {}", msg),
            ProgramError::NotYetImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for ProgramError {}

impl From<std::io::Error> for ProgramError {
    /// Converts a `std::io::Error` into an `AocError::IoError`.
    fn from(err: std::io::Error) -> ProgramError {
        ProgramError::IoError(err)
    }
}