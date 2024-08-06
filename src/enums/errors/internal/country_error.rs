#[derive(Debug, Clone)]
pub enum CountryError {
    CountryNotFound,
}

impl std::fmt::Display for CountryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountryError::CountryNotFound => write!(f, "Country not found"),
        }
    }
}
