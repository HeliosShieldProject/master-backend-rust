use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use tracing::{error, info};

use crate::enums::errors::internal::{Error, Hash, Result};

pub async fn hash_password(password: &str) -> Result<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| {
            error!("Password hashing failed");
            Error::Hash(Hash::Hash)
        })
        .map(|hash| {
            info!("Password hashed: {:?}", hash.to_string());
            hash.to_string()
        })
}
