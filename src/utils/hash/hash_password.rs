use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::enums::errors::internal::{HashError, InternalError};

pub fn hash_password(password: &str) -> Result<String, InternalError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| InternalError::HashError(HashError::Hash))?;

    Ok(hash.to_string())
}
