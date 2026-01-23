Шаг 2.6: Запечатывание
=================

__Estimated time__: 1 day

В программировании «запечатывание» обычно означает, что некоторый API (в основном публичный) не может быть унаследован, расширен или реализован за пределами места его определения. Например, [запечатанный класс или интерфейс в Kotlin][1] не может быть унаследован или реализован за пределами библиотеки, где он определен. В [Rust] этот принцип может применяться к [трейтам][2].


## Traits

__Запечатанный trait__ — это __общедоступный__ trait, который __не может быть реализован вне места его определения__ (__module или crate__, в зависимости от видимости этого признака).
> ```rust
> /// Этот trait является запечатанный и не может быть реализована для типов, находящихся за пределами этого crate.
> pub trait TheTrait: private::Sealed {
>     // Ноль или более методов, которые пользователю разрешено вызывать.
>     fn ...();
>
>     // Ноль или более закрытых методов, вызов которых пользователю запрещен.
>     #[doc(hidden)]
>     fn ...();
> }
>
> // Реализуйте для некоторых типов.
> impl TheTrait for usize {
>     /* ... */
> }
>
> mod private {
>     pub trait Sealed {}
>
>     // Implement for those same types, but no others.
>     // Реализуйте для тех же типов, но не для других.
>     impl Sealed for usize {}
> }
> ```
> The empty private `Sealed` supertrait cannot be named by downstream crates, so we are guaranteed that implementations of `Sealed` (and therefore `TheTrait`) only exist in the current crate.
> Пустой private супертрейт `Sealed` не может быть назван нижестоящими крейтами, поэтому мы гарантируем, что реализации `Sealed` (и, следовательно, `TheTrait`) существуют только в текущем крейте.



This is the most common way to seal a trait. The boilerplate could be completely cut off by using a [`sealed`] crate, providing a convenient macro to generate the one:
Это наиболее распространенный способ запечатать trait. Стандартный текст можно полностью убрать, используя крейт [`sealed`], который предоставит удобный макрос для его генерации:

```rust
use sealed::sealed;

#[sealed]
pub trait TheTrait {}

#[sealed]
impl TheTrait for usize {}
```

Однако существуют альтернативные способы запечатать признак [с помощью сигнатуры его метода][5] или даже [запечатать его частично][6].

Главная цель защиты признака, конечно же, заключается в [обеспечении защиты от будущих угроз][7] для [API].

> Мы можем добавлять методы в `TheTrait` в релизе, не нарушающем обратную совместимость, даже если это обычно приводит к изменению совместимости для трейтов, которые не запечатаны. Также мы можем изменять сигнатуру методов, которые не задокументированы публично.

Важно отметить, что __запечатывание трейта полностью основано на__ обмане с правилами видимости (__использовании публичного [супертрейта][8]__ или типа, имя которого не является публично экспортируемым__), и поэтому не влияет на семантику системы типов (запечатанный публичный трейт — это просто обычный публичный трейт с точки зрения системы типов). Теоретически, запечатывание трейта должно влиять на его [когерентность][9], [ослабляя его строгость для вариантов использования, которые никогда не могут произойти с запечатанным трейтом][10]. Однако это потребовало бы специальной поддержки со стороны компилятора, что, похоже, [не произойдет в ближайшем будущем][11].


Для лучшего понимания свойств запечатывания, её конструкции и областей применения, ознакомьтесь со следующей информацией:
- [Rust API Guidelines: 10. Future proofing: Sealed traits protect against downstream implementations (C-SEALED)][3]
- [Predrag Gruevski: A definitive guide to sealed traits in Rust][4]
- [Jack Wrenn: Private Methods on a Public Trait][13]
- [Official `sealed` crate docs][`sealed`]



## Task

Seal the traits defined in [this step's crate](src/lib.rs) in the following way:
- Make the [`MyIteratorExt` trait](src/my_iterator_ext.rs) fully sealed. Do it manually, using the [`sealed`] crate or a similar one is __not allowed__.
- Make the [`MyError` trait](src/my_error.rs) partially sealed. Only seal the method marked with `#[doc(hidden)]` attribute.
- Sealing should work on both module level (disallowing to implement the sealed trait or the sealed method in the root module of the crate or any other module outside the one where the traits are defined, prove it by providing commented implementations in the root module of the crate, which doesn't compile due to the seal, if uncommented) and crate level (prove it by creating [documentation tests which doesn't compile][12] due to the seal).




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- What does sealing mean in programming in a broad sense?
- [`Что означает «запечатывание» в программировании в широком смысле?`]()

- What is trait sealing in [Rust]? When is it useful?
- What limitations does trait sealing in [Rust] have? What could it be able to provide if supported by compiler?

<hr>

<h3>Что означает «запечатывание» в программировании в широком смысле?</h3>

В программировании термин «запечатывание» (`sealing`) означает ограничение возможности расширения или изменения программного компонента внешним кодом.

В широком смысле это механизм контроля иерархии, который запрещает другим программистам наследоваться от вашего класса или реализовывать ваш интерфейс за пределами строго определенной области (обычно — текущего модуля или библиотеки).

Вот основные аспекты «запечатывания» в разных парадигмах:

<h4>1. Запечатывание в объектно-ориентированном программировании (ООП)</h4>

В таких языках, как C#, Kotlin или Java, запечатывание используется для остановки цепочки наследования.

- Механизм: Ключевые слова sealed (Kotlin/Java) или sealed / final (C#).
- Цель: Гарантировать, что поведение класса не будет переопределено. Это упрощает поддержку кода, так как автор точно знает все возможные подтипы своего класса.
- Пример: Если вы создаете класс CreditCardPayment, вы можете «запечатать» его, чтобы никто не мог создать «поддельный» подкласс, который меняет логику списания средств.

<h4>2. Запечатывание типов (Алгебраические типы данных)</h4>

В функциональных и современных системных языках (Swift, Scala, а также Java 17+) «запечатанный» интерфейс или класс позволяет компилятору знать все возможные варианты реализации.

- Преимущество: Это включает проверку полноты (`exhaustiveness checking`). Если вы используете `match` или `switch` по запечатанному типу, компилятор выдаст ошибку, если вы забыли обработать один из вариантов. В Rust эту роль выполняют `enum`.

<h4>3. Запечатывание в контексте Rust (`Sealed Traits`)</h4>

Хотя в Rust нет ключевого слова sealed, программисты используют этот шаблон (паттерн), чтобы запретить пользователям библиотеки реализовывать определенный трейт для своих типов.

- Как это работает: Трейт делается зависимым от другого (скрытого в приватном модуле) трейта.
- Зачем: Это позволяет автору библиотеки добавлять новые методы в трейт, не ломая код пользователей (так как пользователи всё равно не могли его реализовать сами).

<h4>Почему «запечатывание» — это хорошо?</h4>

- Безопасность (`Security`): Гарантия того, что критически важный код не будет подменен или расширен непредсказуемым образом.
- Оптимизация (`Performance`): Компилятор может применять более агрессивные оптимизации (например, `devirtualization`), так как он точно знает, что новых наследников у типа не появится.
- Предсказуемость API: Пользователь библиотеки получает четкие границы использования инструмента, что снижает риск ошибок.

<h4>Когда НЕ стоит запечатывать?</h4>

Если вы разрабатываете фреймворк, который по своей сути должен расширяться пользователями (например, система плагинов или базовые UI-компоненты), запечатывание будет излишним и вредным ограничением.

<h4>Итог:</h4>
Запечатывание — это способ сказать: «Этот компонент завершен и должен использоваться именно так, как я задумал, без вмешательства в его структуру».



<hr>

[`sealed`]: https://docs.rs/sealed
[API]: https://en.wikipedia.org/wiki/API
[Rust]: https://www.rust-lang.org

[1]: https://kotlinlang.org/docs/sealed-classes.html
[2]: https://doc.rust-lang.org/book/ch10-02-traits.html
[3]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
[4]: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust
[5]: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust#sealing-traits-via-method-signatures
[6]: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust#partially-sealed-traits
[7]: https://en.wikipedia.org/wiki/Future-proof
[8]: https://doc.rust-lang.org/reference/items/traits.html#supertraits
[9]: https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
[10]: https://stackoverflow.com/questions/50012745/is-there-a-way-to-tell-the-compiler-that-nobody-will-implement-a-trait-for-a-ref
[11]: https://internals.rust-lang.org/t/sealed-traits/16797
[12]: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes
[13]: https://jack.wrenn.fyi/blog/private-trait-methods
