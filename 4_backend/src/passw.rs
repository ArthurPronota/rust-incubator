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

    //let salt = SaltString::generate(&mut OsRng) ;

    //let arg = Argon2::default() ;

    if password.is_empty() {
        return Err(anyhow::anyhow!("password is empty"));
    }

    match // arg
            Argon2::default()
                .hash_password(
                    password.as_bytes(), 
                    &SaltString::generate(&mut OsRng)        // &salt
                ) 
    {
        Ok(v) => Ok(v.to_string()),
        Err(err) => Err(anyhow::anyhow!("{}", err))
    }
}
