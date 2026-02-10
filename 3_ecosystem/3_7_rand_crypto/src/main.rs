use rand::Rng ;

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
}

/// Извлекает случайный элемент из заданного среза
fn select_rand_val<T>(slice: &[T]) ->Result<&T, String> {

    if slice.is_empty() {
        return Err("slice is empty".to_string()) ;
    }

    Ok(&slice[
            thread_rng()
                .gen_range(0..slice.len())
            ]
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
}
