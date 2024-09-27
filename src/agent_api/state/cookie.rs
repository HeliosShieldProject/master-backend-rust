use tokio::time::Instant;

#[derive(Clone)]
pub struct Cookie {
    pub cookie: String,
    pub expires_at: Instant,
}
