use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug)]
pub enum HashError {
    HashError,
}

pub fn hash_password(password: &str) -> Result<String, HashError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(_) => return Err(HashError::HashError),
    };

    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(hash).unwrap();

    match argon2.verify_password(password.as_bytes(), &hash) {
        Ok(_) => true,
        Err(_) => false,
    }
}
