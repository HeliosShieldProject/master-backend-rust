use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use tracing::{error, info};

use crate::enums::errors::internal::{HashError, InternalError};

pub async fn hash_password(password: &str) -> Result<String, InternalError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| {
            error!("Password hashing failed");
            InternalError::HashError(HashError::Hash)
        })
        .map(|hash| {
            info!("Password hashed: {:?}", hash.to_string());
            hash.to_string()
        })
}
