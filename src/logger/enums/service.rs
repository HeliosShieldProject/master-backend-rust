pub enum Services {
    ConfigService,
    DeviceService,
    ServerService,
    SessionService,
    UserService,
    HashUtils,
    TokenUtils,
}

impl Services {
    pub fn to_string(&self) -> String {
        match self {
            Services::ConfigService => "config_service".to_string(),
            Services::DeviceService => "device_service".to_string(),
            Services::ServerService => "server_service".to_string(),
            Services::SessionService => "session_service".to_string(),
            Services::UserService => "user_service".to_string(),
            Services::HashUtils => "hash_utils".to_string(),
            Services::TokenUtils => "token_utils".to_string(),
        }
    }
}
