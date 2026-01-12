Шаг 1.7: `Sized` и `?Sized` типы
====================================

__Estimated time__: 1 day

Most types in [Rust] have a particular size, in bytes, that is knowable at compile time. For example, an `i32` is 32 bits big, or 4 bytes. However, there are some types which are useful to express, but do not have a defined size (called "unsized" or "dynamically sized" types). One example is `[T]`: it represents a certain number of `T` in a sequence, but we don’t know how many there are, so the size is not known.

All types with a constant size known at compile time in [Rust] implement [`Sized`] marker trait. And all type parameters (except `Self` in traits) have always an implicit bound of [`Sized`]. So, you should not bother about specifying [`Sized`] marker trait in code, usually.

To better understand [`Sized`]'s and `?Sized`'s purpose, design, limitations and use cases, read through:
- [Official `Sized` docs][`Sized`]
- [Old Rust Book: 3.31. Unsized Types][4]
- [Rust Forum: Trait Objects and the Sized Trait][5]
- [pretzelhammer: Sizedness in Rust][6]
- [Christian Visintin: Don't you dare to sort your struct fields when using ?Sized][7]




## Using `?Sized` to accept more types

The more important concept to understand for day-to-day routine is a `?Sized` trait bound, which __lifts the implicit [`Sized`] bound allowing to use more types__ in generic code (so provide better API and ergonomics).

A real-world example would be:
```rust
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}
```
which allows to use "unsized" types like [trait objects][3]
```rust
impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;
    
    fn handle_command(&self, cmd: &CreateUser, user_repo: &Self::Context) -> Self::Result {
        // Here we operate with the `UserRepository`
        // via its trait object `&dyn UserRepository`
    }
}
```




## Task

Given the [`User` and `UserRepository` implementations from the previous task](../1_6_dispatch#task), write the actual code for `CommandHandler<CreateUser>` implementation described above.

Provide tests for `CommandHandler<CreateUser>` implementation where `dyn UserRepository` is mocked with another hand-written type for testing purposes (you will need to transform the `UserRepository` type into a trait).




## Questions


После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- What is [`Sized`] trait about? When [Rust] implies it? And when not?
- [`Что означает свойство Sized? Когда Rust его подразумевает? А когда нет?`]()
- Why `?Sized` trait bound is important? When should we use it and why?

<hr>

<h3>Что означает свойство Sized? Когда Rust его подразумевает? А когда нет?</h3>

Свойство Sized является фундаментальным маркером того, как данные располагаются в памяти.

<h4>Что означает Sized?</h4>

Тип считается `Sized` (имеющим фиксированный размер), если количество байтов, которые он занимает в памяти, известно на этапе компиляции.

- Зачем это нужно: Только для Sized типов компилятор может заранее знать, сколько места выделить в стеке, как вычислить смещение полей в структуре и как передавать значения напрямую.
- Примеры: u32 (4 байта), f64 (8 байт), [i32; 10] (40 байт), а также все обычные struct и enum.

Противоположность — DST (Dynamically Sized Types), типы с динамическим размером. К ним относятся срезы ([`T`]) и объекты трейтов (`dyn Trait`). Их размер известен только во время выполнения.

<h4>Когда Rust подразумевает Sized?</h4>

В Rust работает принцип «Sized по умолчанию». Это сделано для удобства, так как 99% типов в коде имеют фиксированный размер.

1. <b>Обобщения (Generics)</b>: Когда вы пишете `<T>`, Rust неявно добавляет ограничение `: Sized`.

```rust
// Вы пишете так:
fn func<T>(item: T) {}

// Компилятор видит так:
fn func<T: Sized>(item: T) {}
```

2. <b>Структуры</b>: По умолчанию все поля структуры должны быть `Sized`.

3. <b>Локальные переменные</b>: Вы не можете создать переменную типа `[u8]` (без ссылки) в стеке, так как компилятор не знает, сколько места зарезервировать.

<h4>Когда Rust НЕ подразумевает Sized?</h4>

Существует несколько исключений, где ограничение Sized снимается или его нужно снимать вручную.

1. Специальный синтаксис `?Sized`:

Если вы хотите, чтобы ваше обобщение могло принимать и типы с неизвестным размером (например, `str` или `dyn Trait`), вы должны явно «отключить» `Sized`.

```rust
// T может быть как фиксированного размера, так и динамического
fn process<T: ?Sized>(item: &T) { 
    // Здесь мы можем работать только через ссылку, 
    // так как размер T неизвестен.
}
```
Символ ? читается как «возможно, не Sized».


<hr>


[Rust]: https://www.rust-lang.org
[`Sized`]: https://doc.rust-lang.org/std/marker/trait.Sized.html

[3]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
[4]: https://doc.rust-lang.org/1.26.0/book/first-edition/unsized-types.html
[5]: https://users.rust-lang.org/t/trait-objects-and-the-sized-trait/14410
[6]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md
[7]: https://blog.veeso.dev/blog/en/dont-you-dare-to-sort-your-struct-fields-when-using-sized
