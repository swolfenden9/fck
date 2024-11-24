use std::{fmt::Display, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnmatchedBracket { position: usize },
    InfiniteLoop { position: usize },
    MemoryOutOfBounds { position: usize },
    Io { internal: io::Error },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnmatchedBracket { position } => {
                write!(f, "unmatched bracket at position {}", position)
            }
            Error::InfiniteLoop { position } => {
                write!(f, "infinite loop detected at position {}", position)
            }
            Error::MemoryOutOfBounds { position } => {
                write!(f, "memory access out of bounds at position {}", position)
            }
            Error::Io { internal } => {
                write!(f, "io error: {}", internal)
            }
        }
    }
}
