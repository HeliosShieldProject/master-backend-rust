use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use tracing::{error, info};

use crate::enums::errors::internal::{Error, Hash, Result};

pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(hash).unwrap();

    argon2
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| {
            error!("Password verification failed: {:?}", hash.to_string());
            Error::Hash(Hash::Verify)
        })
        .map(|_| {
            info!("Password verified: {:?}", hash.to_string());
            true
        })
}
