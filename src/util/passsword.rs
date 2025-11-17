use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
pub fn hash_password(password: impl Into<String>) -> Option<String> {
    let pass = password.into();

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(pass.as_bytes(), &salt);
    match password_hash {
        Ok(p) => {
            return Some(p.to_string());
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn compare_password(password: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hashed_password);
    if let Ok(parsed) = parsed_hash {
        return Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok();
    }
    return false;
}
