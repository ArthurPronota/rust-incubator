/* 
    Реализация JSON Web Token
*/

use anyhow::Result ;

use chrono::{
        Duration,
        Utc
} ;

use jsonwebtoken::{
        decode, 
        encode, 
        DecodingKey, 
        EncodingKey, 
        Header, 
        Validation
};
use serde::{
        Serialize,
        Deserialize
    };


/// cookie для подтверждения сессии
#[derive(
    Serialize,
    Deserialize,
    Debug,
)]
pub struct JwtCookieStr {
    /// субъект, user_id
    sub:    u32,
    /// годен до
    exp:    usize,
    /// момент создания
    iat:    usize,
}

/// Реализация методов для JwtCookieStr
impl JwtCookieStr {
    /// Полдучить sub
    pub fn get_sub(&self) ->u32 {
        self.sub
    }
}

/// Аутоидентификационный сервис
pub struct AuthService {
    /// секретная фраза
    secrent_phrase:   String,
    /// время жизни тикена в часах
    expiration:       u32,
}

// Реализация методов для AuthService
impl AuthService {
    // Создать новый объект 
    pub fn new(
            sec_phr: &str,  // секретная фраза
            expir: u32      // время жизи токена
          ) 
          ->Result<Self> {

        if sec_phr.is_empty() {
            return Err(anyhow::anyhow!("sec_phr is empty."));
        }
        else if expir <= 0 {
            return Err(anyhow::anyhow!("Invalid value: {} of expiration", expir));
        }

        Ok(AuthService { secrent_phrase: sec_phr.to_string(), expiration: expir })
    }

    // генерация токена
    pub fn generate_token(&self, user_id: u32) ->Result<String> {

        let expir = 
                match Utc::now() 
                    .checked_add_signed(Duration::hours(self.expiration as i64)) 
        {
            Some(v) => v.timestamp() as usize,
            None => return Err(anyhow::anyhow!("Invalid expiration: {}", self.expiration))
        } ;

        // создание содержимого Jwt Cookie
        let cookie = JwtCookieStr {
                sub:    user_id,
                exp:    expir,
                iat:    Utc::now().timestamp() as usize
        } ;

        //println!("{:?}", cookie) ;

        Ok(
            encode(
                &Header::default(), 
                &cookie, 
                &EncodingKey::from_secret(
                        self.secrent_phrase.as_bytes()
                    )
            )?
        )
    }

    /// Проверка токена
    pub fn validate_token(&self, token: &str) ->Result<JwtCookieStr> {

        if token.is_empty() {
            return Err(anyhow::anyhow!("token is empty"));
        }

        let token_data = 
                decode::<JwtCookieStr>(
                    token,
                    &DecodingKey::from_secret(
                            self.secrent_phrase.as_bytes()
                    ),
                    &Validation::default()
                )?
                .claims ;

        if token_data.exp < Utc::now().timestamp() as usize {
            return Err(anyhow::anyhow!("JSON Web Token expired."));
        }

        Ok(token_data)
    }
}

#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn check_new_authservice() {
        
        assert!(AuthService::new("abcdasdasfdqwedff", 1).is_ok()) ;
    }

    #[test]
    fn check_generate_token() {
        
        let auth_serice = 
                AuthService::new("abcdasdasfdqwedff", 1).unwrap() ;

        assert!(auth_serice.generate_token(10).is_ok()) ;
    }

    #[test]
    fn valid_check_validate_token() {
        let auth_serice = 
                AuthService::new("abcdasdasfdqwedff", 1).unwrap() ;

        let jwt_token = 
                auth_serice.generate_token(10).unwrap() ;

        assert!(auth_serice.validate_token(&jwt_token).is_ok()) ;
    }

    #[test]
    fn invalid_check_validate_token() {
        let auth_serice = 
                AuthService::new("abcdasdasfdqwedff", 1).unwrap() ;

        let mut jwt_token = 
                auth_serice.generate_token(10).unwrap() ;

        jwt_token.push_str("abc");

        assert!(auth_serice.validate_token(&jwt_token).is_err()) ;
    }    
}