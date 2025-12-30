
/// Adds two numbers together.
///
/// # Examples
/// 
/// В качестве crate указывать то что прописано в файде ```Cargo.toml```
/// 
/// в параметре ```name = "step_1_1"```
/// 
/// Запук тестов из документации: ```cargo test --doc```
/// 
/// Создание документации: ```cargo doc```
/// ```
/// let result = step_1_1::my_doc_test_add(2, 3) ;
/// assert_eq!(result, 5);
/// ```
pub fn my_doc_test_add(a: i32, b: i32) ->i32 {
        a + b
}

/// Using hidden `try_main` in doc tests.
/// 
/// Пример срытого но компилируемого кода: `# `
/// 
/// В документации видно только: ```let res = step_1_1::try_div_2(10, 2)? ;```
///
/// Пример будет работать и без символа `# `
/// 
/// Запук тестов из документации: ```cargo test --doc```
/// 
/// Создание документации: ```cargo doc```
/// ```
/// # fn try_main() ->Result<(), String> {
///      let res = step_1_1::try_div_2(10, 2)? ;
/// #    Ok(())
/// # }
/// # fn main() {
/// #    try_main().unwrap() ;
/// # }
/// ```
pub fn try_div_2(a: i32, b: i32) ->Result<i32, String> {
    if b == 0 {
        Err("Dived by zero".to_string())
    } else {
        Ok(a / b)
    }
}