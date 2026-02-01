const PROG_PATH: &str = "../../target/debug/deps/step_3_1" ;
const TOO_SMALL: &str = "Too small!" ;
const TOO_BIG: &str = "Too big!" ;
const YOU_WIN: &str = "You win!" ;

use core::panic;

use std::{
        io::Write, 
        process::{Command, Stdio}
    } ;

/// проверка отсутствия секретного номера в аргументах командной строки
/// 
/// запуск теста: 
/// 
/// cargo test no_secret_number -- --nocapture
/// 
/// cargo test no_secret_number
#[test]
fn no_secret_number() {
    match Command::new(PROG_PATH) 
                .output() {

        Ok(out) => {

            if out.status.success() {
                panic!("The program completed successfully.") ;
            }

            match str::from_utf8(&out.stderr) {
                Ok(text) => {
                    assert!(text.contains("No secret number is specified")) ;
                },
                Err(err) => panic!("{}", err),
            }
        },
        Err(err) => panic!("{}", err),
    }
}

/// проверка что секретный номер не чмсло
/// 
/// запуск теста: 
/// 
/// cargo test secret_number_isnt_number -- --nocapture
/// 
/// cargo test secret_number_isnt_number
#[test]
fn secret_number_isnt_number() {

    match Command::new(PROG_PATH) 
                .arg("a") // секретный номер не число
                .output() {
        Ok(out) => {
            if out.status.success() {
                panic!("The program completed successfully.") ;
            }

            match str::from_utf8(&out.stderr) {
                Ok(text) => {
                    assert!(text.contains("Secret number is not a number")) ;
                },
                Err(err) => panic!("{}", err),
            }
        },
        Err(err) => panic!("{}", err),
    }
}

/// проверка что секретный номер не угадан
/// 
/// запуск теста: 
/// 
/// cargo test doesnt_win -- --nocapture
/// 
/// cargo test doesnt_win
#[test]
fn doesnt_win() {

    // определяем секретный номер
    let secret_number = 1u32 ;    

    match Command::new(PROG_PATH) 
                .arg(format!("{}", secret_number))   // загаданное нами число
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn() // Выполняет команду как дочерний процесс, возвращая обработчик к нему.
    {
        Ok(mut child) => {
            // запись в stdin угаданного числа
            match child.stdin.as_mut() {
                Some(stdin) => {
                    if let Err(err) = stdin.write_all(
                                                format!("{}\n", secret_number + 1).as_bytes()
                                            ) {
                        panic!("stdin.write_all error: {}", err) ;
                    }
                },
                None => panic!("child.stdin error"),
            }

            match child.stdout.as_mut() {
                Some(stdout) => {
                    use std::io::{BufRead, BufReader};

                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        match line {
                            Ok(text) => {

                                if text.contains(TOO_SMALL) { // число угадано неверно
                                    let _ = child.kill() ;  // Пинуждаем child процесс чтобы завершиьтся.
                                    return ;    // тест прошёл успешно
                                } else if text.contains(TOO_BIG) {  // число угадано неверно
                                    let _ = child.kill() ;    // Пинуждаем child процесс чтобы завершиьтся.
                                    return ;    // тест прошёл успешно
                                } else if text.contains(YOU_WIN) {
                                    panic!("You guessed the number") ;
                                }
                            },
                            Err(err) => panic!("Error reading from stdout: {}", err),
                        }
                    }
                    panic!("child process crash") ;
                },
                None => panic!("stdout access error"),
            }
        },
        Err(err) => panic!("{}", err),
    }
    
}

/// проверка что секретный номер угадан
/// 
/// запуск теста: 
/// 
/// cargo test you_win -- --nocapture
/// 
/// cargo test you_win
#[test]
fn you_win() {

    // определяем секретный номер
    let secret_number = 1u32 ;

    match Command::new(PROG_PATH) 
                .arg(format!("{}", secret_number))   // загаданное нами число
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn() // Выполняет команду как дочерний процесс, возвращая обработчик к нему.
    {
        Ok(mut child) => {
            // запись в stdin угаданного числа
            match child.stdin.as_mut() {
                Some(stdin) => {
                    if let Err(err) = stdin.write_all(
                                                    format!("{}\n", secret_number).as_bytes()
                                                ) {
                        panic!("stdin.write_all error: {}", err) ;
                    }
                },
                None => panic!("child.stdin error"),
            }

            match child.stdout.as_mut() {
                Some(stdout) => {
                    use std::io::{BufRead, BufReader};

                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        match line {
                            Ok(text) => {

                                if text.contains(TOO_SMALL) {   // число угадано неверно
                                    panic!("{}", TOO_SMALL) ;
                                } else if text.contains(TOO_BIG) {  // число угадано неверно
                                    panic!("{}", TOO_BIG) ;
                                } else if text.contains(YOU_WIN) {  // число угадано верно
                                    return ;    // тест прошёл успешно
                                }
                            },
                            Err(err) => panic!("Error reading from stdout: {}", err),
                        }
                    }
                    panic!("child process crash") ;
                },
                None => panic!("stdout access error"),
            }
        },
        Err(err) => panic!("{}", err),
    }
    
}