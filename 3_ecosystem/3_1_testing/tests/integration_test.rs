use core::panic;
//use core::panic;
use std::{io::{Read, Write}, process::{Command, Stdio}} ;

#[test]
/// проверка отсутствия секретного номера в аргументах командной строки
fn no_secret_number() {
    match Command::new("../../target/debug/deps/step_3_1") 
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
#[test]
fn secret_number_isnt_number() {

    match Command::new("../../target/debug/deps/step_3_1") 
                .arg("a")
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

#[test]
fn doesnt_win() {
    match Command::new("../../target/debug/deps/step_3_1") 
                .arg("1")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn() // Выполняет команду как дочерний процесс, возвращая обработчик к нему.
    {
        Ok(mut child) => {
            // запись в stdin угаданного числа
            match child.stdin.as_mut() {
                Some(stdin) => {
                    if let Err(err) = stdin.write_all(b"10\n") {
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
                        println!("Строка из процесса: {}", line.unwrap());
                    }
                    panic!("child process crash") ;
                },
                None => panic!("stdout access error"),
            }
        },
        Err(err) => panic!("{}", err),
    }
    
}