Шаг 3.2: Декларативные и процедурные макросы
===========================================

__Estimated time__: 1 day

[Rust] предоставляет мощные и удобные встроенные возможности для генерации кода в виде [макросов][1].

> Термин macro обозначает семейство возможностей Rust: декларативные макросы с `macro_rules!` и три типа процедурных макросов:
> - Пользовательские макросы `#[derive]`, указывающие код, добавляемый с помощью атрибута `derive`, используемого для структур и перечислений.
> - Атрибуто подобные Макросы, которые определяют пользовательские атрибуты, используемые для любого элемента.
> - Функциональноподобные Макросы, которые выглядят как вызовы функций, но работают с токенами, указанными в качестве аргументов.


## Декларативные макросы

Декларативные макросы представляют собой наиболее примитивную форму макросов в [Rust]. Они довольно ограничены в своих возможностях, а их синтаксис (который представляет собой выражение `match` на основе [DSL] 
`Domain-specific language` - Предметно-ориентированный язык) может стать довольно громоздким в сложных случаях.

Они называются _декларативными_, потому что реализация макросов представляет собой объявление правил преобразования кода (вы объявляете, как будет преобразован ваш код):

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = vec![1, 2, 3];
```
Преимущество декларативных макросов в том, что они [гигиеничны][11] (и поэтому имеют гораздо лучшую поддержку в [IDE]).

Декларативные макросы используются не только для генерации кода. Довольно часто их применяют и для создания абстракций и API, поскольку они позволяют реализовать гораздо более эргономичные функции, чем обычные функции: именованные аргументы, [вариативные][17] и т. д.

Для лучшего понимания дизайна, концепций, использования и особенностей декларативных макросов, ознакомьтесь со следующей информацией:
- [Rust Book: 19.6. Macros: Declarative Macros with `macro_rules!` for General Metaprogramming][13]
- [Rust By Example: 16. macro_rules!][14]
- [The Little Book of Rust Macros][15]
- [Rust Reference: 3.1. Macros By Example][16]
- [Aurorans Solis: macros_rule!][18]

## Процедурные макросы

Процедурные макросы представляют собой гораздо более мощный инструмент генерации кода. Они называются процедурными, потому что реализация макроса представляет собой обычный код [Rust], который работает непосредственно с [AST] - (Abstract syntax tree) преобразованного кода (вы пишете процедуры, которые преобразуют ваш код). Для реализации процедурного макроса __требуется отдельный crate `proc-macro = true`__.


Процедурные макросы [негигиеничны][11], поэтому при их реализации необходимо тщательно следить за тем, чтобы макрос работал [как можно больше контекстов][22].


В настоящее время в [Rust] существует три типа процедурных макросов:

- [Функционально подобные макросы `proc_macro`][27], использование которых похоже на использование обычных декларативных макросов, но они принимают произвольные токены на входе (в отличие от декларативных) и, как правило, более мощные (могут содержать сложную логику для генерации простого кода):

    ```rust
    #[proc_macro]
    pub fn make_answer(_: TokenStream) -> TokenStream {
        "fn answer() -> u32 { 42 }".parse().unwrap()
    }
    ```
    ```rust
    make_answer!();
    ```

- [Атрибутные макросы `proc_macro_attribute`][28], которые позволяют создавать пользовательские [атрибуты Rust][25]:
    ```rust
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
       // code...
    }
    ```
    ```rust
    #[route(GET, "/")]
    fn index() {}
    ```

- [`proc_macro_derive` макросы производящие][29], которые позволяют предоставлять пользовательские реализации для атрибута `#[derive(Trait)]`:
    ```rust
    #[proc_macro_derive(AnswerFn)]
    pub fn derive_answer_fn(_: TokenStream) -> TokenStream {
        "impl Struct{ fn answer() -> u32 { 42 } }".parse().unwrap()
    }
    ```
    ```rust
    #[derive(AnswerFn)]
    struct Struct;
    ```
    В идиоматическом плане `proc_macro_derive` следует использовать _только для генерации реализаций трейтов_. Для генерации произвольных функций лучше использовать `proc_macro_attribute`.

В экосистеме [Rust] есть несколько хорошо известных библиотек, которые почти всегда используются для реализации процедурных макросов:
- [`syn`] crate представляет собой реализацию [AST] языка [Rust].
- [`quote`] crate предоставляет квази-цитирование, которое позволяет преобразовывать структуры данных синтаксического дерева [Rust] в токены исходного кода эргономичным и читаемым способом.
- [`proc-macro2`] crate предоставляет унифицированный API [`proc_macro`] для всех версий компилятора [Rust] и делает процедурные макросы пригодными для модульного тестирования.

В настоящее время они являются основой для написания реализации процедурных макросов. Хотя разработчики, как правило, избегают использования [`syn`] в тривиальных случаях (не требующих сложного анализа [AST]), поскольку это [значительно увеличивает время компиляции][30], или предпочитают использовать более простые и менее мощные крейты для анализа [AST] (например, [`venial`] или [`unsynn`]).

Кроме того, для уменьшения количества лишнего оборудования, улучшения эргономики и включения батарей в комплектацию могут использоваться дополнительные контейнеры для экосистемы. Наиболее примечательными из них являются:
- [`darling`] фреймворк, делающий декларативный анализ атрибутов более простым и эргономичным.
- [`synstructure`] — это библиотека, предоставляющая вспомогательные типы для сопоставления с вариантами перечислений и извлечения привязок к каждому из полей в производной структуре или перечислении универсальным способом.
- [`synthez`] фреймворк, предоставляющий [макросы вывода][29] для разбора [AST] (да, макросы вывода для макросов вывода!) и другие полезные «наборы» для ежедневной работы по написанию процедурных макросов.

Для лучшего понимания дизайна, концепций, использования и особенностей процедурных макросов, ознакомьтесь со следующей информацией:
- [Rust Book: 19.6. Macros: Procedural Macros for Generating Code from Attributes][23]
- [Rust Reference: 3.2. Procedural Macros][26]
- [Official `syn` crate docs][`syn`]
- [Official `venial` crate docs][`venial`]
- [Official `unsynn` crate docs][`unsynn`]
- [Official `quote` crate docs][`quote`]
- [Official `proc-macro2` crate docs][`proc-macro2`]
- [Nazmul Idris: Guide to Rust procedural macros][32]
- [Vitaly Bragilevsky: What Every Rust Developer Should Know About Macro Support in IDEs][31]
- [Arthur Cohen: Looking at Rust builtin derives][33]




## Task

Implement a `btreemap!` macro, which allows to create [`BTreeMap`] in an ergonomic and declarative way (similarly to `vec!`).

Provide two implementations: one via declarative macro and other one via procedural macro.




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- [`Что такое макросы? Какую проблему они решают?`](#что-такое-макросы-какую-проблему-они-решают)


- Which benefits do declarative macros have in [Rust] comparing to procedural ones? Which downsides and limitations?
- Which kinds of procedural macros do exist in [Rust]?
- What are common crates for implementing procedural macros in [Rust]? What responsibilities does each one have? Which are mandatory, which are not?
- What are good practices for implementing procedural macros in [Rust]?

<hr>

<h3>Что такое макросы? Какую проблему они решают?</h3>

В Rust макросы — это способ написания кода, который генерирует другой код. Это мощный инструмент метапрограммирования, который работает на этапе компиляции.

Существует два основных типа:

1. Декларативные (`Declarative`): `macro_rules!` — работают через сопоставление с шаблоном (как `vec![]` или `println!()`).
2. Процедурные (`Procedural`): функции на Rust, которые принимают код как входные данные и возвращают модифицированный код (например, `#[derive(Serialize)]`).

__Какую проблему они решают?__

1. _Избавление от дублирования_ (Boilerplate)
Если вам нужно реализовать один и тот же трейт для 20 разных структур, писать это вручную утомительно и чревато ошибками. Макрос сделает это за вас автоматически.
2. _Создание предметно-ориентированных языков (DSL)_
Макросы позволяют расширять синтаксис Rust. Например, макрос `html!` в некоторых библиотеках позволяет писать HTML-подобный код прямо внутри Rust, который затем превращается в оптимизированные структуры данных.
3. _Вариативность аргументов_
Обычные функции в Rust имеют фиксированное количество аргументов. Макросы же (как `println!`) могут принимать любое количество параметров разных типов.
4. _Выполнение вычислений во время компиляции_
Макросы позволяют проверить корректность данных (например, строк формата или SQL-запросов) еще до того, как программа будет запущена. Если в строке `println!("{:?}", x)` допущена ошибка, компилятор сообщит об этом сразу.

__В чем разница с функциями?__

|Характеристика|Функции|Макросы|
|Когда работают|Во время выполнения (Runtime)|Во время сборки (Compile-time)|
|Аргументы|Фиксированное кол-во и типы|Переменное кол-во (Variadic)|
|Сложность|Легко читать и отлаживать|Труднее писать и отлаживать|
|Гибкость|Ограничена сигнатурой|Почти безгранична (манипуляция токенами)|

__Важное правило__: Всегда отдавайте предпочтение функциям. Используйте макросы только тогда, когда задача не может быть решена через систему типов или дженерики.

<hr>

[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[`darling`]: https://docs.rs/darling
[`proc_macro`]: https://doc.rust-lang.org/proc_macro
[`proc-macro2`]: https://docs.rs/proc-macro2
[`quote`]: https://docs.rs/quote
[`syn`]: https://docs.rs/syn
[`synstructure`]: https://docs.rs/synstructure
[`synthez`]: https://docs.rs/synthez
[`unsynn`]: https://docs.rs/unsynn
[`venial`]: https://docs.rs/venial
[AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[DSL]: https://en.wikipedia.org/wiki/Domain-specific_language
[IDE]: https://en.wikipedia.org/wiki/Integrated_development_environment
[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Macro_(computer_science)
[11]: https://en.wikipedia.org/wiki/Hygienic_macro
[13]: https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
[14]: https://doc.rust-lang.org/rust-by-example/macros.html
[15]: https://danielkeep.github.io/tlborm/book/README.html
[16]: https://doc.rust-lang.org/reference/macros-by-example.html
[17]: https://doc.rust-lang.org/rust-by-example/macros/variadics.html
[18]: https://auroranssolis.github.io/rust/2024/02/14/macros-rule.html
[22]: https://rust-lang.github.io/api-guidelines/macros.html#item-macros-work-anywhere-that-items-are-allowed-c-anywhere
[23]: https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes
[25]: https://doc.rust-lang.org/reference/attributes.html
[26]: https://doc.rust-lang.org/reference/procedural-macros.html
[27]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
[28]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
[29]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
[30]: https://hackmd.io/mxdn4U58Su-UQXwzOHpHag?view#round-13-cargo-timing-opt-j8
[31]: https://blog.jetbrains.com/rust/2022/12/05/what-every-rust-developer-should-know-about-macro-support-in-ides
[32]: https://developerlife.com/2022/03/30/rust-proc-macro
[33]: https://cohenarthur.github.io/2023/06/05/rust-derives.html
