#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Banned,
    PermanentlyBanned,
    Deleted,
}
