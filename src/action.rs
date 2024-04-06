use std::{error::Error, fmt};

mod delta;

#[derive(Debug)]
pub enum ActionError {
    DatabaseError(rusqlite::Error),
    InvalidTimestamp,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::DatabaseError(e) => write!(f, "Database error: {}", e),
            ActionError::InvalidTimestamp => write!(f, "Invalid timestamp"),
        }
    }
}

impl Error for ActionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ActionError::DatabaseError(e) => Some(e),
            _ => None,
        }
    }
}

pub type ActionResult<T> = Result<T, ActionError>;
