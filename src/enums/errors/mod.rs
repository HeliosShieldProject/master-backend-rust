pub mod external;
pub mod internal;

pub trait LogError<T, E> {
    fn log_error(self, message: &str) -> Result<T, E>;
}

impl<T, E: std::fmt::Debug> LogError<T, E> for Result<T, E> {
    fn log_error(self, message: &str) -> Result<T, E> {
        tracing::error!("{}, error: {:?}", message, self.as_ref().err());
        self
    }
}
