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

impl CountryError {
    pub fn to_string(&self) -> String {
        match self {
            CountryError::CountryNotFound => "Country not found".to_string(),
        }
    }
}