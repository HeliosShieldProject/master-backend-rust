use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

use crate::{
    enums::errors::internal::{HashError, InternalError},
    logger::{enums::Services::HashUtils, ResultExt},
};

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, InternalError> {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(hash).unwrap();

    argon2
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| InternalError::HashError(HashError::Verify))
        .log_error(HashUtils)
        .await?;
    Ok(true)
}
