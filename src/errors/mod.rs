use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct KecPInitError;

impl fmt::Display for KecPInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keccak-p: width(b) is bad")
    }
}

impl error::Error for KecPInitError {}
