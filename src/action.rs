use std::{error::Error, fmt};

mod delta;

#[derive(Debug)]
pub enum ActionError {
    InvalidTimestamp,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::InvalidTimestamp => write!(f, "Invalid timestamp"),
        }
    }
}

impl Error for ActionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

pub type ActionResult<T> = Result<T, ActionError>;
