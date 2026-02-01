/* Тип из стандартной библиотеки макросов.
   Представляет поток токенов, полученный компилятором на вход макроса.
   Это "сырые" токены, которые нужно разобрать.
 */
use proc_macro::TokenStream;

/*
  Ключевой макрос из библиотеки `quote`. Позволяет генерировать код
  с помощью интерполяции. Аналогично format!(), но для генерации кода Rust.
  Преобразует: #keys → подставляет выражение для ключа
 */
use quote::quote;

use syn::parse::{
        // Parse - трейт для типов, которые можно разобрать из потока токенов
        Parse,
        // ParseStream - входной поток для парсера (как курсор по токенам)
        ParseStream
    };

use syn::{
        // parse_macro_input! - макрос для разбора входных данных
        parse_macro_input,
        // Token - типы для токенов Rust (=>, ,, и т.д.)
        Token,
        // Expr - AST-узел для выражений Rust (1 + 2, x, "string" и т.д.)
        Expr,
        // Result - алиас для Result<T, syn::Error>
        Result,
        // Punctuated - коллекция элементов, разделенных пунктуацией (запятыми)
        punctuated::Punctuated
    };

// Структура для представления одной пары "key => value"
struct MapEntry {
    key: Expr,      // AST выражения для ключа (например: "hello" или 1 + 2)
    value: Expr,    // AST выражения для значения (Abstract syntax tree)
}

// Реализуем парсинг для одной пары
impl Parse for MapEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        /* Парсим выражение для ключа
           input.parse() автоматически вызывает Expr::parse()
           и продвигает ParseStream вперед        
        */
        let key: Expr = input.parse()?;

        /* Парсим оператор =>
           Token![=>] - специальный тип для токена =>
           parse::<T>() пытается разобрать токен типа T
        */
        input.parse::<Token![=>]>()?;

        // Парсим выражение для значения
        let value: Expr = input.parse()?;

        // возвразаем сформированный MapEntry
        Ok(MapEntry { key, value })
    }
}

// Основная функция макроса
#[proc_macro]
pub fn btreemap(input: TokenStream) -> TokenStream {
    // Парсим входные данные как список пар, разделенных запятыми
    // Punctuated автоматически обрабатывает завершающую запятую

    /* Парсим весь входной поток
       Разбирает последовательность MapEntry, разделенных запятыми
       `parse_terminated` автоматически обрабатывает завершающую запятую
    */
    let entries = 
        parse_macro_input!(input with Punctuated::<MapEntry, Token![,]>::parse_terminated);

    /* Извлекаем ключи и значения
       Создаем итераторы по AST-узлам ключей и значений
    */
    let keys = entries.iter().map(|e| &e.key);
    let values = entries.iter().map(|e| &e.value);

    // Генерируем код
    let expanded = quote! {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            #(
                _map.insert(#keys, #values);
            )*
            _map
        }
    };
    // Предположим, вход: btreemap!("a" => 1, "b" => 2)
    // Тогда:
    // keys = итератор по ["a", "b"] (как Expr)
    // values = итератор по [1, 2] (как Expr)
    /*
    Интерполяция #(...)*:
    #( ... )* — повторяет внутренний блок для каждого элемента
    #keys и #values — подставляют соответствующие выражения
     */

    /* преобразование сгенерированного кода из промежуточного 
       представления обратно в формат, понятный компилятору Rust.
    */
    TokenStream::from(expanded)
}
