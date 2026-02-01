// заруск программы:
// cargo run 1
// где 1 - загаданное число

use std::{cmp::Ordering, env, io};

/// получить  секретное число
fn get_secret_number() -> u32 {
    
    // получить агрумент командной строки являющийся secret number
    let secret_number = env::args() // Возвращает аргументы с которыми была стартована эта программа, возвращает итератор
        .skip(1)    // создаёт итератор что пропускает  1-ин элемент
        .take(1)    // создаётитератор что уступает 1-ин элемент
        .last() // взять последнее число из итератора
        .expect("No secret number is specified");

    // преобразовать строку в u32
    secret_number
        .trim() // удаление пробельных символов
        .parse()    // преобразует строку в u32
        .ok() // преобразование Result<u32, ParseIntError > в Option<u32>
        .expect("Secret number is not a number")
}

/// получить секретный номер
fn get_guess_number() -> Option<u32> {
    
    let mut guess = String::new();

    // считать строку из io::stdin
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // преобразовать String в u32
    guess
        .trim()
        .parse()
        .ok()   // преобразование Result<u32, ParseIntError> в Option<u32>
}

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}