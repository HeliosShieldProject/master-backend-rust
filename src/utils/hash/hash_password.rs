use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::enums::HashError;

pub fn hash_password(password: &str) -> Result<String, HashError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(_) => return Err(HashError::HashError),
    };

    Ok(hash.to_string())
}
