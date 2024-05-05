pub enum Handlers {
    ChangePassword,
    Logout,
    Refresh,
    SignIn,
    SignUp,
    GetDevices,
    CreateSession,
    CloseSession,
}

impl Handlers {
    pub fn to_string(&self) -> String {
        match self {
            Handlers::ChangePassword => "change_password".to_string(),
            Handlers::Logout => "logout".to_string(),
            Handlers::Refresh => "refresh".to_string(),
            Handlers::SignIn => "sign_in".to_string(),
            Handlers::SignUp => "sign_up".to_string(),
            Handlers::GetDevices => "get_devices".to_string(),
            Handlers::CreateSession => "create_session".to_string(),
            Handlers::CloseSession => "close_session".to_string(),
        }
    }
}