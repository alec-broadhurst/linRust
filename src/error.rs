use std::fmt;

#[derive(Debug)]
pub enum MatrixError {
    DimensionMismatch(String),
    InvalidIndex(String),
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::DimensionMismatch(msg) => write!(f, "Dimension Mismatch: {}", msg),
            MatrixError::InvalidIndex(msg) => write!(f, "Invalid Index: {}", msg),
        }
    }
}

impl std::error::Error for MatrixError {}
