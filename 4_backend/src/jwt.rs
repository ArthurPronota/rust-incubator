/* 
    Реализация JSON Web Token
*/

use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use chrono::{   // Импорт типов из крейта chrono для работы с датой и временем
        Duration,   // Импорт типа Duration для представления промежутков времени
        Utc,        // Импорт типа Utc для работы с UTC временем
} ;

use jsonwebtoken::{    // Импорт компонентов из крейта jsonwebtoken для работы с JWT токенами
        decode,        // Импорт функции decode для верификации и расшифровки JWT токена
        encode,        // Импорт функции encode для создания и подписи нового JWT токена
        DecodingKey,   // Импорт типа DecodingKey для ключа верификации подписи токена
        EncodingKey,   // Импорт типа EncodingKey для ключа подписи токена
        Header,        // Импорт типа Header для заголовка JWT (алгоритм, тип токена)
        Validation,    // Импорт типа Validation для настройки правил проверки токена
};

use serde::{            // Импорт трейтов из крейта serde для сериализации/десериализации
        Serialize,      // Импорт трейта Serialize для преобразования данных в JSON/другие форматы
        Deserialize,    // Импорт трейта Deserialize для преобразования JSON/данных в структуры
    };


/// cookie для подтверждения сессии
#[derive(
    Serialize,      // Автоматически реализует трейт Serialize для преобразования структуры в JSON
    Deserialize,    // Автоматически реализует трейт Deserialize для преобразования JSON из JWT в структуру
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
    /// Получить sub
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

        // Сформировать результат
        Ok(
            AuthService { 
                secrent_phrase: sec_phr.to_string(), 
                expiration: expir 
            }
        )
    }

    // генерация токена
    pub fn generate_token(&self, user_id: u32) ->Result<String> {

        // определяем время истечения
        let expir = 
                match Utc::now() // Получаем текущее время в UTC
                    .checked_add_signed(    // Прибавляем к текущему времени заданный интервал, проверяя переполнение
                        Duration::hours(self.expiration as i64) // Преобразуем часы из self.expiration в интервал Duration
                    ) 
        {
            Some(v) => v.timestamp() as usize,  // Если сложение успешно - получаем Unix timestamp и преобразуем в usize
            None => return Err(anyhow::anyhow!("Invalid expiration: {}", self.expiration)), // Если произошло переполнение - возвращаем ошибку
        } ;

        // создание содержимого Jwt Cookie
        let cookie = JwtCookieStr { // Создаем экземпляр структуры JwtCookieStr с claims
                sub:    user_id,    // Устанавливаем subject (субъект) - ID пользователя
                exp:    expir,      // Устанавливаем expiration (время истечения) - вычисленный timestamp
                iat:    Utc::now().timestamp() as usize,    // Устанавливаем iat (issued at) - текущее время создания токена 
        } ;

        // Формируем результат
        encode( // Вызываем функцию encode для создания подписанного JWT токена
                &Header::default(),     // Используем стандартный заголовок JWT (алгоритм HS256 по умолчанию)
                &cookie,        // Передаем ссылку на claims (полезную нагрузку)
                &EncodingKey::from_secret(  // Создаем ключ для подписи из секретной фразы
                        self.secrent_phrase.as_bytes()  // Преобразуем секретную фразу в байты
                    )
            )
            .map_err(|err| anyhow::anyhow!("{}", err))  // Преобразуем ошибку в формат anyhow
    }

    /// Проверка токена
    pub fn validate_token(&self, token: &str) ->Result<JwtCookieStr> {

        if token.is_empty() {   // Проверяем, является ли переданный токен пустой строкой
            return Err(anyhow::anyhow!("token is empty"));  // Если токен пустой - возвращаем ошибку с сообщением
        }


        let token_data = 
                decode::<JwtCookieStr>( // Вызываем функцию декодирования JWT с указанием типа claims
                    token,  // Передаем строку токена для декодирования
                    &DecodingKey::from_secret(  // Создаем ключ для верификации из секретной фразы
                            self.secrent_phrase.as_bytes()  // Создаем ключ для верификации из секретной фразы
                    ),
                    &Validation::default()  // Используем стандартные настройки валидации (проверка exp, алгоритма)
                )?
                .claims // Извлекаем claims (полезную нагрузку) из результата декодирования
                ;

        // Проверяем, истек ли срок действия токена
        if token_data.exp < Utc::now().timestamp() as usize {
            return Err(anyhow::anyhow!("JSON Web Token expired."));
        }

        // формирование результата
        Ok(token_data)
    }
}

// Условная компиляция - этот модуль включается только при запуске тестов (cargo test)
#[cfg(test)]
mod tests {     // Определение модуля для Unit тестов
    use super::* ;  // Импортируем все элементы из родительского модуля (выше mod tests)

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверить создание нового AuthService (базовые параметры для создания JWT)
    fn check_new_authservice() {
        
        assert!(AuthService::new("abcdasdasfdqwedff", 1).is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверить генерацию нового токена
    fn check_generate_token() {
        
        let auth_serice = 
                AuthService::new("abcdasdasfdqwedff", 1).unwrap() ;

        assert!(auth_serice.generate_token(10).is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверка валидного JWT токена
    fn valid_check_validate_token() {
        let auth_serice = 
                AuthService::new("abcdasdasfdqwedff", 1).unwrap() ;

        let jwt_token = 
                auth_serice.generate_token(10).unwrap() ;

        assert!(auth_serice.validate_token(&jwt_token).is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверка не валидного JWT токена
    fn invalid_check_validate_token() {
        let auth_serice = 
                AuthService::new("abcdasdasfdqwedff", 1).unwrap() ;

        let mut jwt_token = 
                auth_serice.generate_token(10).unwrap() ;

        jwt_token.push_str("abc");

        assert!(auth_serice.validate_token(&jwt_token).is_err()) ;
    }    
}