Шаг 1.4: Клонирование при записи (Clone-on-write)
========================

__Estimated time__: 1 day




## Clone-on-write

[Rust] has a [`Cow`] (clone-on-write) smart pointer in its standard library. Understanding how to use it is _essential to write idiomatic and ergonomic_ [Rust] code.

In a nutshell: 
- it allows to combine usage of owned and borrowed data in a single abstraction, which __leads to better ergonomics and minimize performance penalties asap__ (as much as possible);
- it encloses and provides immutable access to borrowed data, and __clones the data lazily when mutation or ownership is required__.

```rust
use std::borrow::Cow;

fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        // Returning &'str - a borrowed reference to static str.
        Error::NotFound => "Error: Not found".into(),
        
        // Returning String - an owned String allocated in heap.
        Error::Custom(e) => format!("Error: {}", e).into(),
    }
}
```

To better understand [`Cow`]'s purpose, design, limitations and use cases, read through:
- [Official `Cow` docs][`Cow`]
- [Pascal Hertleif: The Secret Life of Cows][1]
- [Yashodhan Joshi: Using `Cow` in Rust for efficient memory utilization][3]
- [Konstantin Grechishchev: 6 things you can do with the Cow 🐄 in Rust 🦀][2]




## Alternative implementations

[`beef`] crate provides alternative `Cow` types, being faster and leaner.

> There are two versions of `Cow` exposed by this crate:
>
> - `beef::Cow` is 3 words wide: pointer, length, and capacity. It stores the ownership tag in capacity.
> - `beef::lean::Cow` is 2 words wide, storing length, capacity, and the ownership tag all in one word.
> 
> Both versions are leaner than the `std::borrow::Cow`:
> ```rust
> use std::mem::size_of;
> 
> const WORD: usize = size_of::<usize>();
> 
> assert_eq!(size_of::<std::borrow::Cow<str>>(), 4 * WORD);
> assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
> 
> // Lean variant is two words on 64-bit architecture
> #[cfg(target_pointer_width = "64")]
> assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
> ```

Read implementation details and design insights in [its README][4].




## Task

Напишите простую программу, которая выводит путь к файлу конфигурации. Путь должен определяться в следующем порядке приоритета:
1. путь по умолчанию — `/etc/app/app.conf`;

2. если указана переменная окружения `APP_CONF` (и она не пуста), то использовать её с более высоким приоритетом, чем путь по умолчанию;

3. если указан аргумент командной строки `--conf` (ошибка, если пуст), то использовать его с наивысшим приоритетом.

Если ни переменная окружения `APP_CONF`, ни аргумент командной строки `--conf` не указаны, то выделение памяти для определения пути не должно происходить.

Пример запуска программы: `cargo run -- --conf c:\abc\file.txt`

## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [`Что такое Cow? Как это работает?`](#что-такое-cow-как-это-работает)
- When [`Cow`] is useful and why? Give some meaningful examples.
- [`Когда Cow полезно и почему? Приведите несколько показательных примеров.`]()

<hr>

<h3>Что такое Cow? Как это работает?</h3>

Cow (сокращение от Clone-on-Write — «клонирование при записи») — это смарт-указатель в Rust, реализованный как перечисление (enum), которое позволяет работать с данными либо как со ссылкой (заимствование), либо как с собственными данными (владение).
Оно находится в модуле std::borrow::Cow.

Как это устроено внутри?

Тип Cow имеет два варианта:

- `Borrowed(&'a B)`: Хранит ссылку на данные. Память не выделяется, данные просто читаются из существующего источника.
- `Owned(<B as ToOwned>::Owned)`: Хранит данные, которыми владеет сам (например, String или Vec). Память выделена в куче.

```rust
use std::borrow::Cow;

// Cow - это enum с двумя вариантами:
enum Cow<'a, B> {
    Borrowed(&'a B),      // Заимствованная ссылка
    Owned(<B as ToOwned>::Owned), // Владеемые данные
}
```

Основная идея: Экономия ресурсов

Cow позволяет программе быть максимально эффективной: она использует ссылку до тех пор, пока данные не понадобится изменить. Только в момент изменения (или если данные изначально нужно создать динамически) происходит выделение памяти (аллокация).

Как это работает на практике?

Представьте функцию, которая очищает строку от лишних пробелов:
- Если в строке нет пробелов, нет смысла создавать новую строку String. Можно просто вернуть ссылку на оригинал.
- Если пробелы есть, нужно создать новую строку String без них.

```rust
use std::borrow::Cow;

fn remove_spaces(input: &str) -> Cow<str> {
    if input.contains(' ') {
        // Если есть пробелы, создаем новую строку (Owned)
        let new_string: String = input.replace(" ", "");
        Cow::Owned(new_string)
    } else {
        // Если пробелов нет, просто возвращаем ссылку (Borrowed)
        Cow::Borrowed(input)
    }
}

fn main() {
    // Здесь аллокации не будет, так как пробелов нет
    let res1 = remove_spaces("Rust"); 
    
    // Здесь произойдет аллокация String
    let res2 = remove_spaces("R u s t"); 
}
```

Ключевые методы

1. `to_mut()`: Позволяет получить изменяемую ссылку.
- Если это был `Borrowed`, Rust клонирует данные, превращает их в `Owned` и дает вам ссылку на них.
- Если это уже был `Owned`, он просто дает ссылку.

2. `into_owned()`: Превращает Cow в полноценный владеющий тип (например, в String), клонируя данные только при необходимости.

Когда использовать Cow?

- Оптимизация пути "по умолчанию": Как в вашем примере с конфигурацией, когда путь /etc/app/app.conf — это статическая строка (не требует памяти), а путь из переменной окружения — динамическая.
- Чтение из файлов или сети: Когда вы парсите данные и большинство из них можно оставить ссылками на исходный буфер.
- Возврат разных типов строк: Когда функция может вернуть либо строковый литерал &'static str, либо сформированный String.

Итог

Cow — это способ сказать компилятору: "Я хочу использовать ссылку, чтобы не тратить память, но если мне вдруг понадобится изменить эти данные, я готов их склонировать".

<hr>

<b3>Когда Cow полезно и почему? Приведите несколько показательных примеров.</b3>

Cow (Clone-on-Write) наиболее полезен в ситуациях, когда вы хотите избежать лишнего выделения памяти (аллокаций) в «счастливом пути» (happy path), но при этом оставляете за собой возможность владеть данными или изменять их, если это необходимо.

Ниже три показательных примера использования Cow.

1. Оптимизация обработки строк (Ленивое клонирование)

Представьте функцию, которая экранирует символы в строке. В 90% случаев входная строка может быть «чистой». С Cow вы создадите новую строку в куче только в том случае, если действительно найдете символ, требующий замены.

```rust
use std::borrow::Cow;

fn escape_string(s: &str) -> Cow<str> {
    if s.contains('<') || s.contains('>') {
        // Если нашли символы, создаем новую строку (Owned)
        let mut owned = s.to_string();
        let escaped = owned.replace("<", "&lt;").replace(">", "&gt;");
        Cow::Owned(escaped)
    } else {
        // Если символов нет, просто возвращаем входящую ссылку (Borrowed)
        Cow::Borrowed(s)
    }
}

fn main() {
    let s1 = "Hello_World";
    let res1 = escape_string(s1); // Аллокации НЕТ, используется ссылка на s1

    let s2 = "Hello <World>";
    let res2 = escape_string(s2); // Аллокация ЕСТЬ, создана новая String
}
```

2. Возврат либо константы, либо динамических данных

Cow позволяет функции возвращать либо статическую строку-заглушку, либо динамически сформированную строку, не заставляя вызывающую сторону всегда платить за аллокацию String.

```rust
use std::borrow::Cow;

fn get_error_message(code: u32) -> Cow<'static, str> {
    match code {
        404 => Cow::Borrowed("Not Found"), // Ссылка на статический литерал
        500 => Cow::Borrowed("Internal Server Error"),
        unknown => Cow::Owned(format!("Unknown error: {}", unknown)), // Динамическая строка
    }
}
```
- Почему это круто? Для известных ошибок программа работает мгновенно. String создается только для редких или уникальных случаев.

3. Парсинг данных без лишнего копирования (Zero-copy)

Представьте, что вы парсите огромный JSON или лог-файл. Многие поля в нем можно использовать «как есть» прямо из исходного буфера в памяти.

```rust
use std::borrow::Cow ;

#[derive(Debug)]
struct User<'a> {
    username: Cow<'a, str>,
}

fn main() {
    let raw_data = "aDmin";
        
    let mut user = User {
        username: Cow::Borrowed(raw_data),
    };

    if user
        .username
        .chars()
        .any(|c| c.is_uppercase()) {
            user
              .username
              .to_mut()
              .make_ascii_lowercase();
    }
            
    println!("user: {:?}", user) ;
}
```

Итого: Когда стоит использовать Cow?

- Когда аллокация дорогая: Если ваша функция вызывается миллионы раз в секунду (например, в высоконагруженном веб-сервере), замена String на Cow может значительно снизить нагрузку на сборщик мусора (в языках с GC) или просто ускорить работу в Rust.
- Для гибкости API: Если вы пишете библиотеку и не знаете, захочет ли пользователь передать вам &str или String.
- При работе с «грязными» данными: Когда данные чаще всего корректны, но иногда требуют нормализации (удаление пробелов, исправление кодировки).

Краткий вывод: Используйте Cow, когда хотите производительность ссылки, но нуждаетесь в гибкости владения.

<hr>

[`beef`]: https://docs.rs/beef
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[Rust]: https://www.rust-lang.org

[1]: https://deterministic.space/secret-life-of-cows.html
[2]: https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55
[3]: https://blog.logrocket.com/using-cow-rust-efficient-memory-utilization
[4]: https://github.com/maciejhirsz/beef#how-does-it-work
