Step 1.2: Boxing and pinning
============================

__Estimated time__: 1 day




## Boxing

[`Box`] is a pointer that owns heap-allocated data. This is the most common and simplest form of [heap] allocation in [Rust].

It's more idiomatic to use references (`&T`/`&mut T`) for pointing to the data, however they often come with lifetime complexity. [`Box`] allows to avoid this complexity at the cost of heap allocation.

[`Box`] is also a way to go if an owned [slice] is needed, but is not intended to be resized. For example, `Box<str>`/`Box<[T]>` are often used instead of `String`/`Vec<T>` in such cases.

To better understand [`Box`]'s purpose, design, limitations, and use cases, read through:
- [Rust Book: 15.1. Using Box to Point to Data on the Heap][1]
- [Official `std::boxed` docs][`std::boxed`]
- [Amos: What's in the box?][3]
- [Mahdi Dibaiee: What is `Box<str>` and how is it different from `String` in Rust?][8]




## Pinning

It is sometimes useful to have objects that are guaranteed to not move, in the sense that their placement in memory does not change, and can thus be relied upon. A prime example of such a scenario would be building self-referential structs, since moving an object with pointers to itself would invalidate them, which could cause undefined behavior.

[`Pin<P>`][`Pin`] ensures that the pointee of any pointer type `P` has a stable location in memory, meaning it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped. We say that the pointee is "pinned".

However, many types are always freely movable, even when pinned, because they do not rely on having a stable address. This includes all the basic types (like `bool`, `i32`, references) as well as types consisting solely of these types. Types that do not care about pinning implement the [`Unpin`] marker trait, which cancels the effect of [`Pin`]. For `T: Unpin`, `Pin<Box<T>>` and `Box<T>` function identically, as do `Pin<&mut T>` and `&mut T`.

Note, that pinning and [`Unpin`] only affect the pointed-to type `P::Target`, not the pointer type `P` itself that got wrapped in `Pin<P>`. For example, whether or not `Box<T>` is `Unpin` has no effect on the behavior of `Pin<Box<T>>` (here, `T` is the pointed-to type).

To better understand [`Pin`]'s purpose, design, limitations, and use cases, read through:
- [Official `std::pin` docs][`std::pin`]
- [Reddit: Pinned objects ELI5?][2]
- [SoByte: Pin and Unpin in Rust][10]
- [Adam Chalmers: Pin, Unpin, and why Rust needs them][4]
- [Tamme Schichler: Pinning in plain English][5]
- [Yoshua Wuyts: Safe Pin Projections Through View Types][6]
- [Official `#[pin_project]` docs][7]
- [Alice Ryhl answers on "Pin tutorial are confusing me"][9]
- [Rust Forum: Why is it unsafe to pin a shared reference?][11]
- [Ohad Ravid: Put a Pin on That][12]
- [Razieh Behjati: Leaky Abstractions and a Rusty Pin][13]
- [Saoirse Shipwreckt: Pin][14]




## Task

1. Для следующих типов: `Box<T>`, `Rc<T>`, `Vec<T>`, `String`, `&[u8]`, `T`.  
   Реализуйте следующие traits.:
   ```rust
   trait SayHi: fmt::Debug {
       fn say_hi(self: Pin<&Self>) {
           println!("Hi from {:?}", self)
       }
   }
   ```
   ```rust
   trait MutMeSomehow {
       fn mut_me_somehow(self: Pin<&mut Self>) {
        // Реализация должна быть осмысленной и
        // очевидно вызывать что-то, требующее `&mut self`.
        // Цель здесь — попрактиковаться в обработке
        // преобразования `Pin<&mut Self>` -> `&mut self`
        // в разных контекстах, без введения
        // каких-либо ограничений трейта `Unpin`.
       }
   }
   ```

2. Для следующей структуры:
   ```rust
   struct MeasurableFuture<Fut> {
       inner_future: Fut,
       started_at: Option<std::time::Instant>,
   }
   ```

   Предоставьте реализацию трейта [`Future`], которая будет прозрачно опрашивать `inner_future` и выводить время её выполнения в наносекундах, как только она будет готова. Использование привязки трейта `Fut: Unpin` (или аналогичной) не допускается.



## Questions

После выполнения всех вышеперечисленных действий вы должны быть в состоянии ответить (и понять, почему) на следующие вопросы:
- [`Что означает "boxing" в [Rust]? В чём её польза? Когда и зачем она необходима?`](#что-означает-boxing-в-rust-в-чём-её-польза-когда-и-зачем-она-необходима)


- [`Что такое [Pin] и зачем он нужен? Какие гарантии он предоставляет? Как он их обеспечивает?`](#что-такое-pin-и-зачем-он-нужен-какие-гарантии-он-предоставляет-как-он-их-обеспечивает)


- [`Как [Unpin] влияет на [Pin]? Что это означает?`](#как-unpin-влияет-на-pin-что-это-означает)

- Is it allowed to move pinned data after the [`Pin`] dies? Why?
- [Разрешено ли перемещать закрепленные данные после того, как [Pin] перестает работать? Почему?]()

- What is structural pinning? When should it be used and why?
- What is [`Pin`] projection? Why does it exist? How is it used?

<hr>

<h3>Что означает "boxing" в [Rust]? В чём её польза? Когда и зачем она необходима?</h3>

В Rust «упаковка» (boxing) относится к действию обертывания значения в интеллектуальный указатель Box<T>.
Это действие немедленно перемещает обернутые данные из стека в память кучи. Сам Box<T> представляет собой указатель фиксированного размера, который остается в стеке, управляя расположением данных в куче.

Как это полезно

Упаковка (boxing) — это фундаментальный способ управления памятью, которая либо слишком велика для стека, либо требует динамического времени жизни.

- Динамический размер: Box<T> — это указатель фиксированного размера, независимо от размера T. Это решает основное требование Rust: компилятор должен знать размер всех элементов в стеке во время компиляции.
- Управление выделением памяти: Он явно перемещает данные в кучу, что полезно при работе с очень большими объектами, которые могут вызвать переполнение стека, если их хранить в стеке.
- Владение и безопасность: Box<T> строго соблюдает правила владения Rust. Когда Box выходит из области видимости, память в куче детерминированно освобождается (с помощью трейта Drop), что делает ее безопасной и защищенной от утечек без сборщика мусора.

Когда и почему это необходимо

Boxing необходим в нескольких конкретных ситуациях:

1. Когда компилятору требуется гарантированный фиксированный размер

Это наиболее часто встречается с объектами-трейтами и рекурсивными структурами данных.

- Объекты-трейты (`Box<dyn Trait>`): Функция, принимающая любой тип, реализующий интерфейс Drive, требует фиксированного размера для выделения указателя на него. Нельзя использовать `Vec<dyn Drive>`, потому что разные автомобили могут иметь разные размеры, но можно использовать `Vec<Box<dyn Drive>>`, потому что все блоки имеют одинаковый размер.
- Рекурсивные типы: Структуры, которые ссылаются сами на себя (например, связанные списки или бинарные деревья), должны использовать `Box<T>`, чтобы избежать вычисления бесконечного размера во время компиляции.
```rust
// The compiler can't figure out the size of a Node internally
// struct Node { next: Node } // ❌ Error!

struct Node {
    value: i32,
    next: Box<Option<Node>> // ✅ Works: the Box has a known pointer size
}
```

2. Во избежание переполнения стека при работе с большими объемами данных

Если вы работаете с массивом или структурой большого размера, вы можете использовать Box, чтобы предотвратить превышение стандартного размера стека, предоставляемого операционной системой.

<hr>

<h3>Что такое [`Pin`] и зачем он нужен? Какие гарантии он предоставляет? Как он их обеспечивает?</h3>

В Rust Pin — это обертка для указателей, которая гарантирует, что данные, на которые они указывают (объект, на который они указывают), не будут перемещены в памяти.

Почему необходим Pin?

По умолчанию все типы в Rust являются перемещаемыми; компилятор может в любой момент переместить данные по новому адресу в памяти. Это проблематично для структур данных, чувствительных к адресу:

- Самореферентные структуры: Если структура содержит указатель на собственное поле, перемещение структуры по новому адресу приведет к тому, что этот внутренний указатель будет указывать на старое (теперь недействительное) местоположение. [Закрепление](https://doc.rust-lang.ru/async-book/04_pinning/01_chapter.html)
- Async/Await: Компилятор Rust преобразует асинхронные функции в конечные автоматы (Futures). Эти конечные автоматы часто хранят ссылки на собственное внутреннее состояние (подобно локальному буферу). Перемещение Future после начала его выполнения сделает эти внутренние указатели недействительными.

Гарантии, предоставляемые функцией закрепления (Pin)

Когда значение типа T закрепляется, Rust предоставляет гарантию стабильности памяти:

- Неизменяемость: Значение останется по тому же адресу в памяти с момента закрепления до момента его удаления.
- Действительный адрес: Любые прямые указатели на закрепленное значение останутся действительными на протяжении всего его существования.
- Безопасное уничтожение: Закрепленное значение не может быть перемещено из своего места в памяти даже при его уничтожении; оно должно быть удалено на месте.

Как Pin выполняет эти гарантии

Pin выполняет свой контракт, ограничивая способы взаимодействия разработчиков с обернутым указателем в безопасном коде:

- Ограничения API: Для типов, которые являются !Unpin (типы, которые нельзя безопасно перемещать), Pin<&mut T> делает невозможным получение стандартной ссылки на &mut T с помощью безопасного кода. Без изменяемой ссылки вы не можете использовать такие методы, как std::mem::replace или std::mem::swap, которые являются основными способами перемещения данных из указателя.
- Трейт маркера Unpin: Большинство типов в Rust являются Unpin, что означает, что они не чувствительны к адресу и могут быть безопасно перемещены даже при наличии закрепления. Если тип реализует Unpin, Pin фактически ничего не делает и предоставляет полный доступ к базовым данным.
- Небезопасные требования: Чтобы закрепить значение, которое является !Unpin, необходимо использовать небезопасный код. Используя небезопасный код, разработчик вручную обещает компилятору, что он будет соблюдать контракт закрепления — в частности, что данные не будут перемещены или повторно использованы для чего-либо еще, пока они не будут удалены.

<hr>

<h3>Как [Unpin] влияет на [Pin]? Что это означает?</h3>

В Rust Unpin — это трейт-маркер, который сообщает компилятору, что перемещение значения безопасно, даже если оно в данный момент «закреплено». Он фактически отключает ограничения на неподвижность, обычно накладываемые оберткой Pin.

Значение Unpin

- Безопасное перемещение: Тип, реализующий интерфейс Unpin, может быть перемещен по желанию, поскольку он не содержит внутренних самоссылок, которые были бы аннулированы при изменении адреса в памяти.
- Автоматический трейт: Большинство типов в Rust, таких как i32, String и Vec, автоматически реализуют интерфейс Unpin.
- Противоположность (!Unpin): Типы, не реализующие интерфейс Unpin (обозначенные как !Unpin), считаются «чувствительными к адресу». Эти типы обычно включают самоссылочные структуры, подобные тем, которые генерируются блоками async/await.

Как Unpin влияет на Pin

Pin предоставляет свою уникальную гарантию неподвижности только для типов, которые не являются Unpin. Если тип реализует Unpin, обертка Pin ведет себя как обычный указатель.


| Feature |	T: Unpin (Normal Types)	| T: !Unpin (Self-referential)|
|---------|-------------------------|-----------------------------|
|Ограничение по Pin|Снимает ограничения. Pin<&mut T> можно безопасно преобразовать обратно в &mut T.|Вводит ограничения. Нельзя безопасно получить &mut T из Pin<&mut T>.|
|Перемещение|Можно перемещать даже в закрепленном состоянии.|Нельзя перемещать после закрепления.|
|Основное назначение|Pin по сути ничего не делает.|Pin необходим для безопасного использования данного типа.|

По сути, Unpin — это «предохранительный клапан», позволяющий Pin существовать как библиотечная концепция без наложения ненужных затрат на производительность или ограничений API на типы, которые на самом деле не нуждаются в закреплении в памяти.

<hr>

<h3>Разрешено ли перемещать закрепленные данные после того, как [Pin] перестает работать? Почему?</h3>

Нет, перемещать закрепленные данные запрещено даже после того, как сам указатель Pin будет удален.

Контракт закрепления распространяется на базовые данные, а не только на указатель. После того, как значение, не реализующее интерфейс Unpin, было закреплено, оно должно оставаться в том же месте памяти до тех пор, пока не будет полностью удалено.

Почему нельзя перемещать данные

- Срок действия контракта: «Гарантия закрепления» или «Гарантия удаления» начинается в момент создания указателя Pin и действует до тех пор, пока деструктор данных (drop) не завершит возврат.
- Самоссылки: Закрепленные данные (например, асинхронный Future) часто содержат внутренние указатели на свои собственные поля. Удаление указателя Pin не удаляет эти внутренние самоссылки. Если вы переместите данные после того, как Pin перестанет работать, но до того, как сами данные будут удалены, эти внутренние указатели будут указывать на недопустимую память, что приведет к неопределенному поведению.
- Ответственность за безопасность: Если вы закрепили данные в стеке с помощью небезопасного метода (например, через Pin::new_unchecked), вы подписали контракт, обещающий, что данные не будут перемещаться до конца своего жизненного цикла. Компилятор не может автоматически обеспечить это после удаления обертки Pin, поэтому перемещение исходной переменной является нарушением этого небезопасного контракта.

<hr>

[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`Pin`]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[`std::boxed`]: https://doc.rust-lang.org/std/boxed/index.html
[`std::pin`]: https://doc.rust-lang.org/std/pin/index.html
[`Unpin`]: https://doc.rust-lang.org/std/marker/trait.Unpin.html
[heap]: https://en.wikipedia.org/wiki/Memory_management#HEAP
[Rust]: https://www.rust-lang.org
[slice]: https://doc.rust-lang.org/std/primitive.slice.html

[1]: https://doc.rust-lang.org/book/ch15-01-box.html
[2]: https://www.reddit.com/r/rust/comments/9akmqv/pinned_objects_eli5
[3]: https://fasterthanli.me/articles/whats-in-the-box
[4]: https://blog.adamchalmers.com/pin-unpin
[5]: https://blog.schichler.dev/pinning-in-plain-english-ckwdq3pd0065zwks10raohh85
[6]: https://blog.yoshuawuyts.com/safe-pin-projections-through-view-types
[7]: https://docs.rs/pin-project/latest/pin_project/attr.pin_project.html
[8]: https://web.archive.org/web/20230605135444/https://mahdi.blog/rust-box-str-vs-string
[9]: https://users.rust-lang.org/t/pin-tutorial-are-confusing-me/91003/18
[10]: https://www.sobyte.net/post/2022-07/rust-pin-unpin
[11]: https://users.rust-lang.org/t/why-is-it-unsafe-to-pin-a-shared-reference/40309
[12]: https://ohadravid.github.io/posts/2023-07-put-a-pin-on-that
[13]: https://itnext.io/leaky-abstractions-and-a-rusty-pin-fbf3b84eea1f
[14]: https://without.boats/blog/pin
