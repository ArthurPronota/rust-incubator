
use rand::{
        // Это основной трейт, используемый при генерации случайных значений.
        Rng,
    } ;

/*
Добавить в Cargo.toml именно эту версию rand
[dependencies]
rand = "0.8.5"
*/
use rand::{
    // генератор случайных чисел посеянный системой для локального потока,
    // криптографически стойкий.    
    thread_rng
} ;
use sha3::Digest;

/// Генерация пароля указанной длины из заданного charset
fn generate_password(
                pass_len: u32,          // длина пароля
                pass_charset: &str      // набор символов пароля
            ) ->Result<String, String>
{
    // проверка длины пароля
    if pass_len == 0 {
        return Err(format!("pass_len = {}", pass_len));
    }

    // проверка длины pass_charset и инициализация vec_charset
    let vec_charset ;
    match pass_charset.len() {
        0 => return Err("charset is empty.".to_owned()),
        _ => {
            vec_charset = pass_charset
                            .chars()
                            .collect::<Vec<char>>()
                            ;
        },
    }

    /* так:
    Ok(
        // формирование пароля
        (0..pass_len)
            .map(|_| {
                vec_charset[
                    thread_rng()
                        .gen_range(0..vec_charset.len())
                ]
            })
            .collect::<String>()
    ) 
     */

    // или так:
    // Генератор случайных чисел, который можно явно задать в качестве 
    // начального значения.
    use rand::SeedableRng ;

    /*
    Добавить в Cargo.toml именно эту версию rand
    [dependencies]
    rand = { version ="0.8.5",  features = ["small_rng"] }
     */
    // Быстрый некриптографический генератор псевдослучайных чисел с 
    // небольшим объемом памяти    
    use rand::rngs::SmallRng ;

    // Создает новый экземпляр генератора случайных чисел, начальное 
    // значение которого задается с помощью функции getrandom.
    // getrandom — это библиотека Rust для получения случайных данных из 
    // источников (операционной) системы: https://docs.rs/getrandom/latest/getrandom/
    let mut s_rng = SmallRng::from_entropy() ;

    Ok(
        (0..pass_len)
            .map(|_|
            vec_charset[
                s_rng.gen_range(0..vec_charset.len()) 
            ]  
            )
            .collect()
    )

}

/// Извлекает случайный элемент из заданного среза
fn select_rand_val<T>(slice: &[T]) ->Result<&T, String> {

    if slice.is_empty() {
        return Err("slice is empty".to_string()) ;
    }

    /* так:
    Ok(&slice[
            thread_rng()
                .gen_range(0..slice.len())
            ]
    )
     */
    
    // или так:
    use rand::{
            // Генератор случайных чисел, который можно явно задать в качестве 
            // начального значения.
            SeedableRng
        };

    /*
    Добавить в Cargo.toml именно эту версию rand
    [dependencies]
    rand = { version ="0.8.5",  features = ["small_rng"] }
     */
    // Быстрый некриптографический генератор псевдослучайных чисел с 
    // небольшим объемом памяти
    use rand::rngs::SmallRng;

    Ok(
        &slice[
            // SmallRng::from_entropy() - Создает новый экземпляр генератора 
            // случайных чисел, инициализируемый с помощью функции getrandom.
            // getrandom — это библиотека Rust для получения случайных данных из 
            // источников (операционной) системы: https://docs.rs/getrandom/latest/getrandom/
            SmallRng::from_entropy()    
                .gen_range(0..slice.len())
        ]
    )

}

/// генерирует уникальное криптографически безопасное случайное значение 
/// в наборе символов `a-zA-Z0-9` и содержит ровно `64` символа
fn new_access_token() ->String {

    let charset: &[u8] = 
            b"abcdefghijklmnopqrstuvwxyz\
             ABCDEFGHIJKLMNOPQRSTUVWXYZ\
             0123456789" ;

    (0..64)
        .map(|_| 
            charset[
                thread_rng()
                    .gen_range(0..charset.len())
                ] as char
        )
        .collect()
}

/// возвращает хеш SHA-3 файла, указанного по его пути.
fn get_file_hash(path: &str) ->Result<String, String>{
    // проверка что path не пуст
    if path.is_empty() {
        return Err("path is empty".to_string());
    }

    // Объект, предоставляющий доступ к открытому файлу в файловой системе.
    use std::fs::File ;

    use std::io::{
            BufReader,  // Структура BufReader<R> добавляет буферизацию к любому считывателю.
            Read        // Трейт Read позволяет считывать байты из источника.
        } ;
    // Реализация криптографического алгоритма хеширования SHA-3.
    use sha3::Sha3_256 ;

    // Определение Reader
    let file ;
    // Попытка открыть файл в режиме только для чтения.
    match File::open(path) {
        Ok(v) => file = v,
        Err(err) => return Err(err.to_string()),
    } 

    // Создает новый объект BufReader<R> с емкостью буфера по умолчанию (8 KiB).
    let mut reader = BufReader::new(file) ;

    // Определение буфера для чтения из файла
    let mut buffer = [0; 8192] ;

    // Создать новый экземпляр hasher.
    // Это инкрементальный хешер, который может обрабатывать данные по частям
    let mut hasher = Sha3_256::new() ;

    loop {
        let readed_bytes ;
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(v) => readed_bytes = v,
            Err(err) => return Err(err.to_string()),
        }

        // Обработка данных, обновление внутреннего состояния.
        // Принимает срез байтов (&[u8]) - любые данные
        // Обновляет внутреннее состояние хешера
        // Может вызываться много раз для одного хеша
        hasher.update(&buffer[..readed_bytes]);
    }
    
    /*
        hasher.finalize() - это финализирующий метод, который завершает 
            процесс хеширования и возвращает окончательный хеш.
    1. Добавляет padding (дополнение) к оставшимся данным
    2. Выполняет финальный раунд преобразований Keccak
    3. Извлекает итоговый хеш из внутреннего состояния
    4. Сбрасывает/потребляет хешер (после finalize() хешер нельзя использовать)
     */

    Ok(
        format!("{:x}", hasher.finalize())
    )
}

fn main() {
    println!("pass: {:?}", 
        generate_password(
            20, 
            "abcdefghijklmnopqrstuvwxyz\
         ABCDEFGHIJKLMNOPQRSTUVWXYZ\
         0123456789\
         !@#$%^&*_-+"            
        )
    ) ;

    println!("{:?}", select_rand_val(&[1, 2, 3])) ;

    println!("{:?}", select_rand_val(&['a', 'b', 'c'])) ;

    println!("{:?}", select_rand_val(&["abc", "def", "gkj"])) ;

    println!("{:?}", select_rand_val(&[(1, 2), (3, 4), (5, 6)])) ;

    println!("access_token: {}", new_access_token()) ;

    println!("{:?}", get_file_hash("Cargo.toml"))
}
