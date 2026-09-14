## Создание процедурного и декларативного макросов 

макрос `btreemap!`, который собирает `BTreeMap` так же, как `vec!` собирает `Vec`. В проекте есть **две реализации**: декларативная в `src/main.rs` и процедурная (function-like) в `src/lib.rs`.

## Задача

Синтаксис вызова один и тот же:

```rust
btreemap!(
    "one" => 1,
    "two" => 2,
    "three" => 3,
);
```

Ожидаемый результат — `BTreeMap` с этими парами. Крейт помечен как `proc-macro = true`, поэтому процедурный макрос живёт в библиотеке, а декларативный — локально в `main`.

---

## Декларативный макрос (`macro_rules!`)

Он объявлен внутри блока в `main.rs`, чтобы не конфликтовать с процедурным `step_3_2::btreemap`:

```7:17:C:\Users\user\work\MyWorks\Rust\rust-incubator\3_ecosystem\3_2_macro\src\main.rs
        macro_rules! btreemap {
            ($($key:expr => $value:expr),* $(,)?) => {
                {
                    let mut map = BTreeMap::new() ;
                    $(
                        map.insert($key, $value) ;
                    )*
                    map
                }
            };
        }
```

Как это работает:

1. **Одно правило** — шаблон на весь вызов.
2. `$key:expr => $value:expr` — пара «выражение → выражение», не только литералы (`1 + 2` тоже ок).
3. `$( ... ),*` — ноль или больше пар, разделённых запятыми.
4. `$(,)?` — необязательная завершающая запятая (как у `vec!`).
5. В теле создаётся `BTreeMap`, для каждой пары вызывается `insert`, карта возвращается из блока.

Это классический «макрос как `vec!`»: гигиеничный pattern matching, без отдельного крейта и без AST. Ограничение — нельзя разобрать произвольный синтаксис вне шаблона и нельзя дать свои сообщения об ошибках.

`BTreeMap` берётся из `use` в том же блоке. Для учебника это нормально; в публичном API обычно пишут `::std::collections::BTreeMap`, чтобы имя не зависело от окружения.

---

## Процедурный макрос (`#[proc_macro]`)

В `Cargo.toml`: `proc-macro = true`, зависимости `syn` (feature `full`) и `quote`.

Точка входа:

```66:67:C:\Users\user\work\MyWorks\Rust\rust-incubator\3_ecosystem\3_2_macro\src\lib.rs
#[proc_macro]
pub fn btreemap(input: TokenStream) -> TokenStream {
```

Это **function-like** макрос: `btreemap!(...)`, не `#[derive]` и не атрибут.

### Парсинг

Одна запись карты:

```rust
struct MapEntry {
    key: Expr,
    value: Expr,
}
```

`Parse` для неё:

1. `input.parse()?` → ключ как `syn::Expr`
2. `input.parse::<Token![=>]>()?` → токен `=>`
3. снова `Expr` → значение

Весь вход:

```rust
parse_macro_input!(input with Punctuated::<MapEntry, Token![,]>::parse_terminated);
```

`Punctuated` + `parse_terminated` даёт список пар через запятую и допускает trailing comma — то же поведение, что у декларативного варианта.

### Генерация кода

Ключи и значения вынимаются итераторами, затем `quote!`:

```89:97:C:\Users\user\work\MyWorks\Rust\rust-incubator\3_ecosystem\3_2_macro\src\lib.rs
    let expanded = quote! {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            #(
                _map.insert(#keys, #values);
            )*
            _map
        }
    };
```

`#(#keys, #values)*` повторяет `insert` для каждой пары. Путь `::std::collections::BTreeMap` абсолютный — макрос не ломается, если пользователь не импортировал `BTreeMap`. Имя `_map` снижает шанс столкновения с локальной переменной (полная гигиена у proc-macro всё равно слабее, чем у `macro_rules!`).

`TokenStream::from(expanded)` отдаёт `proc_macro2::TokenStream` обратно компилятору.

Вызов из `main`: `use step_3_2::btreemap;` — имя крейта из `Cargo.toml` (`step_3_2`).

---

## Сравнение двух реализаций

| | Декларативный | Процедурный |
|---|---|---|
| Где | любой модуль | отдельный крейт `proc-macro` |
| Механика | шаблон `macro_rules!` | `syn` → AST → `quote` |
| Вызов | `btreemap!(k => v, ...)` | тот же |
| Trailing comma | `$(,)?` | `parse_terminated` |
| Путь к `BTreeMap` | локальный `use` | `::std::collections::...` |
| Гигиена | есть | нужно следить вручную |
| Сложность / ошибки | просто; «no rules expected…» | можно свои `syn::Error` (здесь не сделано) |

Семантика одинаковая: блок `{ let mut map = BTreeMap::new(); insert…; map }`. Процедурный вариант здесь учебный: для такого DSL достаточно `macro_rules!`. Процедурный код показывает стек `TokenStream` → `Parse`/`Punctuated` → `quote!` → `TokenStream`.
