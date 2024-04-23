use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(hash).unwrap();

    match argon2.verify_password(password.as_bytes(), &hash) {
        Ok(_) => true,
        Err(_) => false,
    }
}
