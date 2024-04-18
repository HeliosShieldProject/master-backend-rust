#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    LoggedIn,
    LoggedOut,
    Revoked,
}