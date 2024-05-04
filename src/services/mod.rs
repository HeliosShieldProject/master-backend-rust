pub mod config_service;
pub mod device_service;
pub mod server_service;
pub mod session_service;
pub mod user_service;

pub enum Services {
    ConfigService,
    DeviceService,
    ServerService,
    SessionService,
    UserService,
}

impl Services {
    pub fn to_string(&self) -> String {
        match self {
            Services::ConfigService => "config_service".to_string(),
            Services::DeviceService => "device_service".to_string(),
            Services::ServerService => "server_service".to_string(),
            Services::SessionService => "session_service".to_string(),
            Services::UserService => "user_service".to_string(),
        }
    }
}
