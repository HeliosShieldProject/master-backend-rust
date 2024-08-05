#[derive(Debug)]
pub enum DeviceError {
    DeviceNotFound,
    DeviceAlreadyExists,
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::DeviceNotFound => write!(f, "Device not found"),
            DeviceError::DeviceAlreadyExists => write!(f, "Device already exists"),
        }
    }
}
