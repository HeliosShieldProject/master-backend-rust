#[derive(Debug)]
pub enum HashError {
    Hash,
    Verify,
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HashError::Hash => write!(f, "Hash error"),
            HashError::Verify => write!(f, "Verify error"),
        }
    }
}
