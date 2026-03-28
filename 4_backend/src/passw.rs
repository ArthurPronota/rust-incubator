use anyhow::Result ;

use argon2::{
    password_hash::{
        rand_core::OsRng, 
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier, 
        SaltString
    },
    Argon2,
};

/// получение хеша пароля
pub fn hash_password(password: &str) ->Result<String> {

    if password.is_empty() {
        return Err(anyhow::anyhow!("password is empty"));
    }

    Ok(
        Argon2::default()
            .hash_password(
                password.as_bytes(), 
                &SaltString::generate(&mut OsRng)
            )
            .map_err(|err| anyhow::anyhow!("{}", err))?
            .to_string()
    )
}

/// проверка пароля
pub fn check_password(password: &str, hash_password: &str) ->Result<bool> {

    if password.is_empty() {
        return Err(anyhow::anyhow!("password is empty"));
    } else if hash_password.is_empty() {
        return Err(anyhow::anyhow!("hash_password is empty"));
    }

    Ok(
        Argon2::default()
            .verify_password(
                password.as_bytes(),
                &PasswordHash::new(hash_password) 
                        .map_err(|err| anyhow::anyhow!("{}", err))?
            )
            .is_ok()
    )
}