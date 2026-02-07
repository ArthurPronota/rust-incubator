Шаг 3.6: Сериализация и десериализация
===========================================

__Estimated time__: 1 day




## `serde`

В экосистеме [Rust] существует хорошо известный крейт [`serde`], который предоставляет общий (стандартный, де-факто) подход и набор инструментов для сериализации и десериализации.


Самое приятное то, что [`serde`] __не полагается на механизм рефлексии во время выполнения__ и использует реализацию трейтов для каждого типа, поэтому __исключает большинство затрат во время выполнения__ и в большинстве случаев __делает сериализацию такой же производительной, как и сериализатор, написанный вручную, для конкретного случая__, при этом __остается эргономичным благодаря [автоматическому выводу кода][1]__.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

[`serde`] сам по себе представляет собой лишь универсальный интерфейс сериализации, который может быть подкреплен фактической реализацией для любого формата. Уже существуют [реализованные бэкенды для наиболее часто используемых форматов][2], и вы можете [реализовать бэкенд для своего собственного формата][3], если он еще не реализован.

Чтобы лучше понять и ознакомиться с дизайном, концепциями, использованием и функциями [`serde`] (например, [десериализация с нулевым копированием][5]), прочтите следующее:
- [Official `serde` crate guide][0]
- [Official `serde` crate docs][`serde`]
- [Official `serde_json` crate docs][`serde_json`]
- [Owen Gage: Understanding Rust's serde using macro expansion][6]
- [Owen Gage: Exploring serde's data model with a toy deserializer][7]
- [Owen Gage: A look at serde-json][11]
- [Manish Goregaokar: Not a Yoking Matter (Zero-Copy #1)][12]
- [Manish Goregaokar: Zero-Copy All the Things! (Zero-Copy #2)][13]
- [Manish Goregaokar: So Zero It's ... Negative? (Zero-Copy #3)][14]


### Дополнительно

Будучи де-факто стандартом экосистемы, сам крейт [`serde`] довольно консервативен в отношении гарантий стабильности, поэтому часто может казаться, что ему не хватает очевидных функций. Следовательно, стоит рассмотреть дополнительные крейты экосистемы, которые расширяют возможности [`serde`], будучи построенными на основе его механизма:

- [`erased-serde`] ящик, предоставляющий типизированные версии признаков serde `Serialize`, `Serializer` и `Deserializer`, которые могут использоваться в качестве [trait objects][9].
- [`serde_state`] crate, расширяющий обычные трейты `Deserialize` и `Serialize`, позволяющий передавать состояние каждому значению, которое сериализуется или десериализуется.
- [`serde_repr`] crate, производный от трейтов `Serialize` и `Deserialize` объекта `serde` таким образом, чтобы делегировать вызов базовому представлению C-подобного перечисления.
- [`serde_with`] crate, предоставляющий пользовательские вспомогательные средства де/сериализации для использования в сочетании с [`serde`-аннотацией `with`][8] и с улучшенной `serde_as`-аннотацией.
- [`serde_valid`] crate, позволяющий использовать валидацию на основе [JSON Schema][10].


## `musli`

[`musli`] — это относительно новая и альтернативная структура для сериализации и десериализации, которая развивает принципы [`serde`], но также переосмысливает и преодолевает некоторые из её фундаментальных ограничений.

> Müsli разработан на принципах, схожих с [`serde`]. Он опирается на мощную систему трейтов Rust для генерации кода, который в значительной степени можно оптимизировать. В результате должен получиться код, очень похожий на написанный вручную высокооптимизированный код.

> Отличия Müsli в философии дизайна заключаются в двух моментах:
>
> Мы используем GAT для обеспечения более тесных абстракций, что должно упростить оптимизацию Rust.
>
> В некоторых случаях, когда это считается ненужным, например, [при декодировании коллекций][21], мы реже используем паттерн «Посетитель». В результате обычно получаются более чистые реализации декодирования.

Однако главной "убийственной особенностью" [`musli`] является его способность сериализовывать/десериализовывать одну и ту же модель данных в разных [режимах][22].

> Еще одно важное отличие Müsli заключается в концепции [modes][22] (обратите внимание на параметр `M` выше). Поскольку это параметр трейтов `Encode` и `Decode`, он позволяет сериализовать одну и ту же модель данных множеством различных способов.


> ```rust
> use musli::mode::{DefaultMode, Mode};
> use musli::{Decode, Encode};
> use musli_json::Encoding;
>
> enum Alt {}   // Создается пустой enum Alt - это пользовательский режим сериализации
> impl Mode for Alt {}  // Реализуется трейт Mode для этого enum
>
> #[derive(Decode, Encode)]
> #[musli(mode = Alt, packed)]  // для режима Alt использовать packed кодировку
> // packed - это компактная форма сериализации, где:
> // Поля сериализуются как массив (без имен полей)
> // Порядок полей определяется порядком в структуре
> // Более компактный вывод, но менее читаемый
> #[musli(default_field_name = "name")] // по умолчанию использовать имя поля как "name"
> // Структура с двумя полями:
> struct Word<'a> {
>     text: &'a str,
>     teineigo: bool,
> }
> // использует стандартный режим (DefaultMode) конфигурации
> let CONFIG: Encoding<DefaultMode> = Encoding::new();
> // использует пользовательский режим (Alt) конфигурации
> let ALT_CONFIG: Encoding<Alt> = Encoding::new();
>
> let word = Word {
>     text: "あります",
>     teineigo: true,
> };
> // Сериализация в DefaultMode 
> let out = CONFIG.to_string(&word)?;
> assert_eq!(out, r#"{"text":"あります","teineigo":true}"#);
> // Сериализация в Alt (пользовательский с packed)
> let out = ALT_CONFIG.to_string(&word)?;
> assert_eq!(out, r#"["あります",true]"#);
> // Разбор сырой строки:
> // r#" - начало сырой строки
> // "#  - конец сырой строки
> // ["あります",true] - содержимое сырой строки
> // # - один символ-разделитель (можно больше: ##, ###, etc.)
> ```

Чтобы лучше понять и ознакомиться с дизайном, концепциями, использованием и функциями [`musli`], прочтите следующее:
- [Official `musli` crate docs][`musli`]
- [John-John Tedro: A fresh look on incremental zero copy serialization][23]




## `rkyv`

[`rkyv`] (_archive_) — это еще одна альтернативная платформа сериализации/десериализации, __полностью ориентированная на операции [нулевого копирования][31]__.

> Подобно [serde][0], rkyv использует мощную систему трейтов Rust для сериализации данных без необходимости использования рефлексии. Несмотря на широкий спектр функций, вы платите только за то, что используете. Если ваши данные верны, процесс сериализации может быть таким же простым, как `memcpy`! Как и serde, это позволяет rkyv работать со скоростью, сравнимой с сериализаторами, написанными вручную.
>
> В отличие от serde, rkyv гарантирует отсутствие десериализации данных. Если вы записали данные на диск, вы можете просто отобразить файл в память с помощью `mmap`, преобразовать указатель, и ваши данные будут готовы к использованию. Это делает его идеальным для высокопроизводительных приложений, интенсивно использующих ввод-вывод.
> Хотя rkyv — отличный формат для итоговых данных, ему не хватает полноценной системы схем, и он плохо подходит для миграции данных и обновления схем. Если для вашего случая требуются эти возможности, вам могут понадобиться дополнительные библиотеки, которые позволят реализовать эти функции на основе rkyv. Вы можете использовать другие фреймворки сериализации, такие как serde, с теми же типами, что и в rkyv, без конфликтов.

Чтобы лучше понять и ознакомиться с дизайном, концепциями, использованием и функциями [`rkyv`], прочтите следующее:
- [Official `rkyv` crate docs][`rkyv`]
- [`rkyv` book][30]


## Task

Write a program which deserializes the [following JSON](request.json) into a static `Request` type and prints out its serialization in a YAML and TOML formats. Consider to choose correct types for data representation.

Prove your implementation correctness with tests.




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- [Как `serde` достигает своей производительности? Как он моделирует данные и разделяет обязанности?](#как-serde-достигает-своей-производительности-как-он-моделирует-данные-и-разделяет-обязанности)

- When does it have sense to prefer [`musli`] rather than [`serde`]?
- What is zero-copy deserialization? Why is it beneficial? How does it work in [`serde`]? How does it work in [`rkyv`]?

<hr>

### Как `serde` достигает своей производительности? Как он моделирует данные и разделяет обязанности?

Производительность Serde в Rust остается эталонной благодаря тому, что библиотека переносит всю тяжелую работу с этапа выполнения (_Runtime_) на этап сборки (__Compile-time__).

<hr>


[`erased-serde`]: https://docs.rs/erased-serde
[`musli`]: https://docs.rs/musli
[`rkyv`]: https://docs.rs/rkyv
[`serde`]: https://docs.rs/serde
[`serde_json`]: https://docs.rs/serde_json
[`serde_repr`]: https://docs.rs/serde_repr
[`serde_state`]: https://docs.rs/serde_state
[`serde_valid`]: https://docs.rs/serde_valid
[`serde_with`]: https://docs.rs/serde_with
[Rust]: https://www.rust-lang.org

[0]: https://serde.rs
[1]: https://serde.rs/derive.html
[2]: https://serde.rs/index.html#data-formats
[3]: https://serde.rs/data-format.html
[4]: https://serde.rs/examples.html
[5]: https://serde.rs/lifetimes.html#understanding-deserializer-lifetimes
[6]: https://owengage.com/writing/2021-07-23-serde-expand
[7]: https://owengage.com/writing/2021-08-14-exploring-serdes-data-model-with-a-toy-deserializer
[8]: https://serde.rs/field-attrs.html#with
[9]: https://doc.rust-lang.org/book/trait-objects.html
[10]: https://json-schema.org
[11]: https://owengage.com/writing/2022-07-22-a-look-at-serde-json
[12]: https://manishearth.github.io/blog/2022/08/03/zero-copy-1-not-a-yoking-matter
[13]: https://manishearth.github.io/blog/2022/08/03/zero-copy-2-zero-copy-all-the-things
[14]: https://manishearth.github.io/blog/2022/08/03/zero-copy-3-so-zero-its-dot-dot-dot-negative
[21]: https://docs.rs/serde/latest/serde/trait.Deserializer.html#tymethod.deserialize_seq
[22]: https://docs.rs/musli#modes
[23]: https://udoprog.github.io/rust/2023-10-19/musli-zerocopy.html
[30]: https://rkyv.org/rkyv.html
[31]: https://rkyv.org/zero-copy-deserialization.html
