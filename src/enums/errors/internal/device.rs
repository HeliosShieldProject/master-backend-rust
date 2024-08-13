#[derive(Debug, Clone)]
pub enum Device {
    NotFound,
    SelfRevocation,
    AlreadyRevoked,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::NotFound => write!(f, "Device not found"),
            Device::SelfRevocation => write!(f, "Device self revocation"),
            Device::AlreadyRevoked => write!(f, "Device already revoked"),
        }
    }
}
