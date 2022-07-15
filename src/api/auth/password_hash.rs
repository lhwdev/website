// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use rocket::http::Status;

use crate::api::utils::ApiDbError;

pub fn hash_password(password: &str) -> Result<String, ApiDbError> {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password, &salt)
        .map_err(|err| ApiDbError::new(Status::InternalServerError, err.to_string()))?
        .to_string())
}

pub fn verify_password(encrypted_phc: &str, password: &[u8]) -> Result<(), ApiDbError> {
    let hash = PasswordHash::new(encrypted_phc)
        .map_err(|err| ApiDbError::new(Status::InternalServerError, err.to_string()))?;

    let result = Argon2::default().verify_password(password, &hash);
    if let Err(err) = result {
        match err {
            Error::Password => Err(ApiDbError::new(
                Status::Unauthorized,
                "Wrong password".to_string(),
            )),
            _ => Err(ApiDbError::new(
                Status::InternalServerError,
                err.to_string(),
            )),
        }
    } else {
        Ok(())
    }
}
