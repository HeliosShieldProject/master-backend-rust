use crate::enums::errors::response::{self, Error, ResponseError};

pub enum DeviceError {
    DeviceNotFound,
    DeviceAlreadyExists,
}

impl Error for DeviceError {
    fn as_response(&self) -> ResponseError {
        match self {
            DeviceError::DeviceNotFound => {
                ResponseError::DeviceError(response::DeviceError::DeviceNotFound)
            }
            DeviceError::DeviceAlreadyExists => {
                ResponseError::DeviceError(response::DeviceError::DeviceAlreadyExists)
            }
        }
    }
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::DeviceNotFound => write!(f, "Device not found"),
            DeviceError::DeviceAlreadyExists => write!(f, "Device already exists"),
        }
    }
}
