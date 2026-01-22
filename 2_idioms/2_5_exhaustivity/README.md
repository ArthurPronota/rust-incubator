Шаг 2.5: Исчерпывание
======================

__Estimated time__: 1 day


Проверка полноты в [сопоставлении с образцом][1] — очень полезный инструмент, позволяющий выявлять определенные ошибки на этапе компиляции, проверяя, были ли учтены и рассмотрены все комбинации значений в исходном коде. При правильном применении она повышает качество [бесстрашного рефакторинга][2] исходного кода, исключая возможность незаметного проникновения ошибок типа «забыл изменить» в кодовую базу при ее расширении.


## Enums

Наиболее каноническим и знаковым примером проверки исчерпываемости является использование перечисления в выражении `match`. Суть здесь в том, чтобы __[опустить][5] использование [`_` (шаблон подстановки)][4] или привязок «соответствует чему угодно»__, поскольку такие выражения `match` не сломаются во время компиляции при добавлении чего-либо нового.

Например, это очень плохой код:
```rust
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        _ => Permissions::All, // anybody else is administrator 
    }
}
```
Если по какой-либо причине будет добавлен новый объект `Role::Guest`, то с очень высокой вероятностью этот код не будет соответствующим образом изменен, что приведет к возникновению уязвимости безопасности, поскольку любому гостю будет предоставлено разрешение `Permissions::All`. Это происходит главным образом потому, что сам код никак не сигнализирует о необходимости его пересмотра.


Благодаря использованию принципа исчерпывающего поиска, код можно изменить таким образом, __чтобы он ломался на этапе компиляции при добавлении нового варианта `Role`__:
```rust
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        Role::Admin => Permissions::All, 
    }
}
```
```
error[E0004]: non-exhaustive patterns: `&Role::Guest` not covered
  --> src/lib.rs:16:11
   |
16 |     match role {
   |           ^^^^ pattern `&Role::Guest` not covered
   |
note: `Role` defined here
  --> src/lib.rs:2:5
   |
1  | enum Role {
   |      ----
2  |     Guest,
   |     ^^^^^ not covered
```

## Structs

While enums exhaustiveness is quite an obvious idea, due to extensive usage of `match` expressions in a regular code, the structs exhaustiveness, on the other hand, is not, while being as much useful. Exhaustivity for structs is achieved by __using [destructuring][6] without [`..` syntax (multiple fields ignoring)][7]__.

В то время как исчерпывающая детализация перечислений является довольно очевидной идеей из-за широкого использования выражений `match` в обычном коде, исчерпывающая детализация структур, с другой стороны, не является таковой, хотя и столь же полезна. Исчерпывающая детализация структур достигается путем __использования [деструктуризации][6] без [`..` синтаксиса (игнорирование нескольких полей)][7]__.

Например, имея следующий код:
```rust
struct Address {
    country: Country,
    city: City,
    street: Street,
    zip: Zip,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.country)?;
        writeln!(f, "{}", self.city)?;
        writeln!(f, "{}", self.street)?;
        write!(f, "{}", self.zip)
    }
}
```

Очень легко забыть изменить реализацию `Display` при добавлении нового поля `state`.


Таким образом, изменение кода с помощью __исчерпывающей деструктуризации позволяет избежать такой незаметной ошибки, возникающей на этапе компиляции__:

```rust
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            country,
            city,
            street,
            zip,
        } = self;
        writeln!(f, "{country}")?;
        writeln!(f, "{city}")?;
        writeln!(f, "{street}")?;
        write!(f, "{zip}")
    }
}
```
```
error[E0027]: pattern does not mention field `state`
  --> src/lib.rs:19:13
   |
19 |           let Self {
   |  _____________^
20 | |             country,
21 | |             city,
22 | |             street,
23 | |             zip,
24 | |         } = self;
   | |_________^ missing field `state`
   |
help: include the missing field in the pattern
   |
23 |             zip, state } = self;
   |                ~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
23 |             zip, .. } = self;
   |    
```

Другие примеры практического применения поддержания инвариантов, охватывающих все поля структуры, посредством проверки исчерпываемости, иллюстрируются в следующих статьях:

- [Ashley Mannix: How we organize a complex Rust codebase][8]




## `#[non_exhaustive]`

До настоящего времени было показано, __как проверка на исчерпывающий характер кода может обеспечить перспективность пользовательского кода__ (того, который использует API определенного типа, а не объявляет его), __заставляя его ломаться всякий раз, когда используемый [API] расширяется__ и требует пересмотра.


__Интересно, что атрибут `#[non_exhaustive]`__, __служит той же самой цели [обеспечения защиты от устаревания][12]__ исходного кода, но совершенно противоположным образом: __он используется в библиотечном коде__ (том, который объявляет [API] какого-либо типа для использования) __для сохранения обратной совместимости, чтобы избежать нарушения работы пользовательского кода при расширении используемого [API]__.


> Внутри определяющего крейта параметр `non_exhaustive` не оказывает никакого эффекта.

> За пределами определяющей библиотеки типы, аннотированные `non_exhaustive`, имеют ограничения, обеспечивающие обратную совместимость при добавлении новых полей или вариантов.

> 
> Non-exhaustive типы не могут быть сконструированы вне определяющего крейта:

> - Non-exhaustive варианты (варианты `struct` или `enum`) нельзя создать с помощью `StructExpression` (в том числе с использованием синтаксиса функционального обновления).

> - могут быть созданы экземпляры `enum`.

> При сопоставлении с non-exhaustive типов, выходящих за рамки определяющей библиотеки, существуют ограничения:
> - При сопоставлении с шаблоном для non-exhaustive варианта (`struct` или `enum`) необходимо использовать `StructPattern`, который должен включать `...`. Видимость конструктора варианта кортежа снижается до `min($vis, pub(crate))`.

> - When pattern matching on a non-exhaustive `enum`, matching on a variant does not contribute towards the exhaustiveness of the arms.

> It's also not allowed to cast non-exhaustive types from foreign crates.

> Non-exhaustive types are always considered inhabited in downstream crates.

Despite being opposite qualities, both exhaustivity and non-exhaustivity are intended for [future-proofing][12] a codebase, thus cannot be applied blindly everywhere, but rather wisely, where it may really has sense. That's why it's __very important__ to understand their __use-cases and implicability__ very well.

To better understand `#[non_exhaustive]` attribute's purpose, design, limitations and use cases, read through:
- [Rust Reference: 7.6. The `non_exhaustive` attribute][9]
- [Rust RFC 2008: `non_exhaustive`][10]
- [Turreta: Using `#[non_exhaustive]` for Non-exhaustive Rust Structs][11]




## Task

Refactor the code contained in [this step's crate](src/lib.rs), so the bugs introduced there will be uncovered at compile-time, and fix them appropriately.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- How can exhaustiveness checking be useful in [Rust] code for enums and structs? When should we use it, when not?
- How does `#[non_exhaustive]` attribute work in [Rust]? What are its use-cases? When should it be used, when not?




[API]: https://en.wikipedia.org/wiki/API
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/ch18-00-patterns.html
[2]: https://news.ycombinator.com/item?id=27553775
[3]: https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms
[4]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
[5]: https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_enum_match_arm
[6]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values
[7]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-remaining-parts-of-a-value-with-
[8]: https://blog.datalust.co/rust-at-datalust-how-we-organize-a-complex-rust-codebase#maintaininginvariantsthatcoverallstructfields
[9]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
[10]: https://rust-lang.github.io/rfcs/2008-non-exhaustive.html
[11]: https://web.archive.org/web/20250120122453/https://turreta.com/blog/2019/12/21/using-non_exhaustive-for-non-exhaustive-rust-structs
[12]: https://en.wikipedia.org/wiki/Future-proof
