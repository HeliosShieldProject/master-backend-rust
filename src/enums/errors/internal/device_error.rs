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