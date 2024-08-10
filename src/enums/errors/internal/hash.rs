#[derive(Debug, Clone)]
pub enum Hash {
    Hash,
    Verify,
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Hash::Hash => write!(f, "Hash error"),
            Hash::Verify => write!(f, "Verify error"),
        }
    }
}
