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

#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn valid_hash_password() {
        assert!(hash_password("abc").is_ok()) ;
    }

    #[test]
    fn invalid_hash_password() {
        assert!(hash_password("").is_err()) ;
    }

    #[test]
    fn valid_check_password() {
        let password = "abc" ;

        let hash_passw = hash_password(password).unwrap() ;

        match check_password(password, &hash_passw) {
            Ok(res) => assert!(res),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn invalid_check_password() {
        let password = "abc" ;
        let password2 = "abcd" ;

        let hash_passw = hash_password(password).unwrap() ;

        match check_password(password2, &hash_passw) {
            Ok(res) => assert!(!res),
            Err(err) => panic!("{}", err),
        }
    }    
}