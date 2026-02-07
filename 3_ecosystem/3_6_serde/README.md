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

[`musli`] is a relatively fresh and alternative framework for serialization and deserialization, which succeeds the principles of [`serde`], but also rethinks and overcomes some of its fundamental limitations.

> Müsli is designed on similar principles as [`serde`]. Relying on Rust’s powerful trait system to generate code which can largely be optimized away. The end result should be very similar to handwritten highly optimized code.

> Where Müsli differs in design philosophy is twofold:
>
> We make use of GATs to provide tighter abstractions, which should be easier for Rust to optimize.
>
> We make less use of the Visitor pattern in certain instances where it’s deemed unnecessary, such as [when decoding collections][21]. The result is usually cleaner decode implementations

However, the __main "killer feature"__ of [`musli`] is its __ability to serialize/deserialize the same data model in different [modes][22]__. 

> Another major aspect where Müsli differs is in the concept of [modes][22] (note the `M` parameter above). Since this is a parameter of the `Encode` and `Decode` traits it allows for the same data model to be serialized in many different ways.

> ```rust
> use musli::mode::{DefaultMode, Mode};
> use musli::{Decode, Encode};
> use musli_json::Encoding;
>
> enum Alt {}
> impl Mode for Alt {}
>
> #[derive(Decode, Encode)]
> #[musli(mode = Alt, packed)]
> #[musli(default_field_name = "name")]
> struct Word<'a> {
>     text: &'a str,
>     teineigo: bool,
> }
>
> let CONFIG: Encoding<DefaultMode> = Encoding::new();
> let ALT_CONFIG: Encoding<Alt> = Encoding::new();
>
> let word = Word {
>     text: "あります",
>     teineigo: true,
> };
>
> let out = CONFIG.to_string(&word)?;
> assert_eq!(out, r#"{"text":"あります","teineigo":true}"#);
>
> let out = ALT_CONFIG.to_string(&word)?;
> assert_eq!(out, r#"["あります",true]"#);
> ```

To better understand and be familiar with [`musli`]'s design, concepts, usage and features, read through:
- [Official `musli` crate docs][`musli`]
- [John-John Tedro: A fresh look on incremental zero copy serialization][23]




## `rkyv`

[`rkyv`] (_archive_) is an another alternative serialization/deserialization framework, __fully focused on [zero-copy][31] operations__.

> Like [serde][0], rkyv uses Rust’s powerful trait system to serialize data without the need for reflection. Despite having a wide array of features, you also only pay for what you use. If your data checks out, the serialization process can be as simple as a `memcpy`! Like serde, this allows rkyv to perform at speeds similar to handwritten serializers.
>
> Unlike serde, rkyv produces data that is guaranteed deserialization free. If you wrote your data to disk, you can just `mmap` your file into memory, cast a pointer, and your data is ready to use. This makes it ideal for high-performance and IO-bound applications.

> While rkyv is a great format for final data, it lacks a full schema system and isn’t well equipped for data migration and schema upgrades. If your use case requires these capabilities, you may need additional libraries the build these features on top of rkyv. You can use other serialization frameworks like serde with the same types as rkyv conflict-free.

To better understand and be familiar with [`rkyv`]'s design, concepts, usage and features, read through:
- [Official `rkyv` crate docs][`rkyv`]
- [`rkyv` book][30]




## Task

Write a program which deserializes the [following JSON](request.json) into a static `Request` type and prints out its serialization in a YAML and TOML formats. Consider to choose correct types for data representation.

Prove your implementation correctness with tests.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- How does [`serde`] achieve its performance? How does it model data and decouple responsibilities?
- When does it have sense to prefer [`musli`] rather than [`serde`]?
- What is zero-copy deserialization? Why is it beneficial? How does it work in [`serde`]? How does it work in [`rkyv`]?




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
