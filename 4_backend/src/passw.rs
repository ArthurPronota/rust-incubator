use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use argon2::{   // Импорт модулей из крейта argon2 (алгоритм хэширования паролей)
    password_hash::{    // Импорт подмодуля password_hash для работы с хэшами паролей
        rand_core::OsRng,   // Импорт OsRng - генератор криптостойких случайных чисел от операционной системы
        PasswordHash,       // Импорт PasswordHash - тип для представления хэша пароля
        PasswordHasher,     // Импорт трейта PasswordHasher - предоставляет метод хэширования пароля
        PasswordVerifier,   // Импорт трейта PasswordVerifier - предоставляет метод верификации пароля
        SaltString,         // Импорт SaltString - тип для генерации и хранения соли (случайной строки)
    },
    Argon2,     // Импорт Argon2 - основная структура алгоритма хэширования
};

/// получение хеша пароля
pub fn hash_password(password: &str) ->Result<String> {

    if password.is_empty() {    // Проверяем, пустой ли передан пароль
        return Err(anyhow::anyhow!("password is empty"));   // Если пароль пустой - возвращаем ошибку с сообщением
    }

    // Формируем результат
    Ok(
        Argon2::default()   // Создаем экземпляр Argon2 с настройками по умолчанию
            .hash_password(     // Вызываем метод хэширования пароля
                password.as_bytes(),    // Преобразуем пароль в байтовый массив (срез)
                &SaltString::generate(&mut OsRng)   // Генерируем случайную соль (salt) используя криптостойкий генератор
            )
            .map_err(|err| anyhow::anyhow!("{}", err))? // Преобразуем ошибку argon2 в anyhow::Error
            .to_string()    // Преобразуем хэш в строку для хранения в базе данных
    )
}

/// проверка пароля
pub fn check_password(password: &str, hash_password: &str) ->Result<bool> {

    if password.is_empty() {    // Проверяем, пустой ли передан пароль
        return Err(anyhow::anyhow!("password is empty"));   // Если пароль пустой - возвращаем ошибку
    } else if hash_password.is_empty() {    // Иначе проверяем, пустой ли передан хэш пароля
        return Err(anyhow::anyhow!("hash_password is empty"));  // Если хэш пустой - возвращаем ошибку
    }

    // Формируем результат
    Ok(
        Argon2::default() // Создаем экземпляр Argon2 с настройками по умолчанию
            .verify_password(   // Вызываем метод верификации пароля
                password.as_bytes(),    // Преобразуем проверяемый пароль в байтовый массив
                &PasswordHash::new(hash_password)   // Парсим строку хэша в структуру PasswordHash
                        .map_err(|err| anyhow::anyhow!("{}", err))? // Преобразуем ошибку парсинга в anyhow::Error
            )
            .is_ok()    // Проверяем, успешна ли верификация (возвращает true если пароль совпадает с хэшем)
    )
}

// Условная компиляция - этот модуль включается только при запуске тестов (cargo test)
#[cfg(test)]
mod tests {     // Определение модуля для Unit тестов
    use super::* ;  // Импортируем все элементы из родительского модуля (выше mod tests)

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверка валидного хеша пароля
    fn valid_hash_password() {
        assert!(hash_password("abc").is_ok()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверка инвалидного хеша пароля
    fn invalid_hash_password() {
        assert!(hash_password("").is_err()) ;
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверка соответствия пароля хешу
    fn valid_check_password() {
        let password = "abc" ;

        let hash_passw = hash_password(password).unwrap() ;

        match check_password(password, &hash_passw) {
            Ok(res) => assert!(res),
            Err(err) => panic!("{}", err),
        }
    }

    // Атрибут, указывающий что следующая функция является тестом
    #[test]
    // проверка несоответствия пароля хешу
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