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

impl DeviceError {
    pub fn to_string(&self) -> String {
        match self {
            DeviceError::DeviceNotFound => "Device not found".to_string(),
            DeviceError::DeviceAlreadyExists => "Device already exists".to_string(),
        }
    }
}