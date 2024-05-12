use crate::enums::errors::response::{self, Error, ResponseError};

pub enum CountryError {
    CountryNotFound,
}

impl Error for CountryError {
    fn as_response(&self) -> ResponseError {
        match self {
            CountryError::CountryNotFound => {
                ResponseError::CountryError(response::CountryError::CountryNotFound)
            }
        }
    }
}

impl std::fmt::Display for CountryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountryError::CountryNotFound => write!(f, "Country not found"),
        }
    }
}
