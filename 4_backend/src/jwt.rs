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
use serde::Serialize;


// cookie для подтверждения сессии
#[derive(
    Serialize,
    Debug,
)]
struct JwtCookieStr {
    sub:    String,     // субъект, user_id
    exp:    usize,      // годен до
    iat:    usize,      // момент создания
}

// Аутоидентификационный сервис
pub struct AuthService {
    // секретная фраза
    secrent_phrase:   String,
    // время жизни тикена в часах
    expiration:       u32,
}


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
                sub:    user_id.to_string(),
                exp:    expir,
                iat:    Utc::now().timestamp() as usize
        } ;

        println!("{:?}", cookie) ;

        Ok(
            encode(
                    &Header::default(), 
                    &cookie, 
                    &EncodingKey::from_secret(
                            self.secrent_phrase.as_bytes()
                            //self.secrent_phrase.as_ref()
                        )
            )?
        )
    }
}