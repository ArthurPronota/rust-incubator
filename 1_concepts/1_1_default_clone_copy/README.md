Шаг 1.1: Значения по умолчанию, клонирование и копирование
=============================================

__Estimated time__: 1 day




## Default values

[Rust] has a standard way to deal with default values of a type via the [`Default`] trait. Read through [its official docs][`Default`] to understand the design.

It can be auto-derived, but only for a `struct` whose members all have [`Default`] implementations. It is implemented for a great many types in the standard library, and also used in a surprising number of places. So if your type has a value that can be construed as being "default", it is a good idea to implement this trait.

If you're not satisfied with [std] deriving capabilities for [`Default`], consider using the [smart-default] crate. An example is quite self-explanatory:
```rust
#[derive(SmartDefault)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default = 12]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String,
    },
    Qux(i32),
}
```

A great thing is that with a [`Default`] implementation you can instantiate your <h2>`struct`</h2> with only the non-default values and have all other fields filled with default values:
```rust
let x = Foo { bar: baz, ..Default::default() };
```




## Cloning and copying

By default, all types in [Rust] follow ['move semantics'][1].

If you need a duplicate of a value, then its type should implement [`Clone`] trait (see [official docs][`Clone`]), and a duplicate is created by calling [`Clone`] methods __explicitly__. Cloning can be __either a cheap or an expensive__ operation depending on type semantics.

However, the [`Copy`] marker trait (see [official docs][`Copy`]) enables 'copy semantics' for a type, so a value is __copied implicitly__ every time it is passed. That's why copying must always perform a __simple bit-to-bit copy operation__.

[Official `Copy` docs][`Copy`] are quite explanatory about which types _should_ be [`Copy`] and which types _cannot_:

> Some types can't be copied safely. For example, copying `&mut T` would create an aliased mutable reference. Copying `String` would duplicate responsibility for managing the `String`'s buffer, leading to a double free.
> 
> Generalizing the latter case, any type implementing `Drop` can't be `Copy`, because it's managing some resource besides its own `size_of::<T>` bytes.

> Generally speaking, if your type can implement `Copy`, it should. Keep in mind, though, that implementing `Copy` is part of the public API of your type. If the type might become non-`Copy` in the future, it could be prudent to omit the `Copy` implementation now, to avoid a breaking API change.

To better understand the topic, read through:
- [Official `Clone` docs][`Clone`]
- [Official `Copy` docs][`Copy`]
- [HashRust: Moves, copies and clones in Rust][2]




## Task

- Create a `Point` type which represents a 2D point (`x` and `y` coordinates). This type has to be `Copy` and `Default`.
- Create a `Polyline` type which represents a non-empty set of `Point`s of unknown size. This type has to be `Clone` and non-`Default`.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- [Какова функция трейта `Default` в Rust?](#какова-функция-трейта-default-в-rust)
- [На что способна директива #[derive(Default)] из стандартной библиотеки? В чём её ошибка? Какие есть альтернативы?](#на-что-способна-директива-derivedefault-из-стандартной-библиотеки-в-чём-её-ошибка-какие-есть-альтернативы)
- What does [`Clone`] mean semantically?
- What does [`Copy`] mean semantically? How is it connected with [`Clone`]? Which limitations does it have and why?

<hr>

<h3>Какова функция трейта Default в Rust?</h3>

В Rust трейт Default предоставляет стандартизированный способ инициализации типа с «разумным» или «нулевым» начальным состоянием. Хотя многие типы имеют собственный конструктор new(), трейт Default позволяет использовать абстракцию в обобщенном программировании, где конкретный метод инициализации может быть неизвестен.

Основные цели:

1. Стандартизированная инициализация: Она предлагает единый интерфейс (Default::default()) для получения начального экземпляра типа без необходимости вручную указывать все поля.
2. Обобщенное программирование: Это необходимо для обобщенных функций или контейнеров, которым необходимо создавать новые экземпляры типа T, не зная его конкретного конструктора (например, Option::unwrap_or_default()).
3. Частичная инициализация: Она включает синтаксис обновления функциональной записи, где вы можете указать только те поля, которые хотите изменить, а остальные заполнить из экземпляра по умолчанию.

```rust
let options = SomeOptions { foo: 42, ..Default::default() };
```

Типичные сценарии использования:

- Структуры конфигурации: Полезно для больших структур, где пользователи могут захотеть изменить только один или два параметра.
- Инициализация коллекции: Предоставление пустой начальной точки для таких типов, как Vec, HashMap или String.
- Значения по умолчанию в стандартной библиотеке:
    - Числа: По умолчанию 0.
    - Логические значения: По умолчанию false.
    - Опция: По умолчанию None.

Default против new()

Хотя оба метода могут использоваться для инициализации, они следуют разным соглашениям:

- new(): Общее соглашение об именовании основного конструктора типа; он может принимать несколько аргументов и иметь любое имя.
- default(): Метод трейта, который не может принимать аргументы и необходим для использования типа в обобщенных контекстах, где конкретные конструкторы неизвестны.

<hr>

<h3>На что способна директива #[derive(Default)] из стандартной библиотеки? В чём её ошибка? Какие есть альтернативы?</h3>

На что способен #[derive(Default)]:

Стандартный макрос derive, предоставляемый std, автоматизирует реализацию трейта Default для структур и перечислений.

- Рекурсивная инициализация: Он устанавливает каждое поле структуры в значение Default::default() этого поля (например, 0 для целых чисел, "" для строк).
- Поддержка перечислений: Начиная с Rust 1.62, его можно использовать для перечислений, если вы пометите один из вариантов атрибутом #[default].
- Нулевая стоимость: Это преобразование на этапе компиляции; оно генерирует тот же самый код, который вы бы написали вручную.

Что в нём «не так» (ограничения):

Несмотря на стандартность, ему не хватает гибкости в трёх основных областях:
1. Отсутствие пользовательских значений

Вы не можете указать значение по умолчанию, отличное от состояния «ноль» типа.
    - Проблема: Если вы хотите, чтобы поле u32 по умолчанию имело значение 100 вместо 0, или чтобы строка по умолчанию имела значение «localhost», #[derive(Default)] вам не поможет. Вам придётся реализовать трейт вручную.

2. Ненужные обобщенные ограничения

Стандартный метод `derive` добавляет ограничение по умолчанию ко всем параметрам обобщенных типов, даже если они строго не требуются для реализации по умолчанию (например, если обобщенный тип обернут в `PhantomData` или указатель).

Проблема: это может привести к ошибкам «неудовлетворенное ограничение трейта» в сложных обобщенных структурах, где сам параметр обобщенного типа не реализует `Default`, но структура логически могла бы это сделать.

3. Сложности с `Enums`

Хотя библиотека поддерживает перечисления, это возможно только в том случае, если вариант по умолчанию является вариантом юнита (без полей) или если вас устраивает, что поля этого варианта также являются собственными значениями по умолчанию. Вы не можете определить вариант по умолчанию, требующий пользовательских данных.

<hr>

[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[std]: https://doc.rust-lang.org/stable/std
[smart-default]: https://docs.rs/smart-default
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/a/30290070/1828012
[2]: https://hashrust.com/blog/moves-copies-and-clones-in-rust
