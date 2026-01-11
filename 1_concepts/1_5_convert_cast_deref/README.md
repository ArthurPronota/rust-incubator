Шаг 1.5: Преобразования, приведение типов и разыменование
================================================

__Estimated time__: 1 day

As [Rust] is a [strongly typed][1] language, all type conversions must be performed explicitly in the code. As [Rust] has a rich type system (programming logic and semantics are mostly expressed in types rather than in values), type conversions are inevitable in almost every single line of code. Fortunately, [Rust] offers [well-designed type conversion capabilities][`std::convert`], which are quite ergonomic, intuitive and are pleasant to use.




## Value-to-value conversion

Value-to-value conversion in [Rust] is done with [`From`] and [`Into`] mirrored traits (implementing the first one automatically implements another one). These traits provide __non-fallible conversion__.

If your conversion may fail, then you should use [`TryFrom`]/[`TryInto`] analogues, which __allow failing in a controlled way__.

```rust
let num: u32 = 5;
let big_num: u64 = num.into();
let small_num: u16 = big_num.try_into().expect("Value is too big");
```

Note, that __all these traits consume ownership__ of a passed value. However, they [can be implemented for references too][2] if you're treating a reference as a value.

To better understand [`From`]/[`Into`]'s and [`TryFrom`]/[`TryInto`]'s purpose, design, limitations and use cases, read through:
- [Rust By Example: 6.1. From and Into][8]
- [Official `From` docs][`From`]
- [Official `Into` docs][`Into`]
- [Official `TryFrom` docs][`TryFrom`]
- [Official `TryInto` docs][`TryInto`]




## Reference-to-reference conversion

Quite often you don't want to consume ownership of a value for conversion, but rather to refer it as another type. In such case [`AsRef`]/[`AsMut`] should be used. They allow to do a __cheap non-fallible reference-to-reference conversion__.

```rust
let string: String = "some text".into();
let bytes: &[u8] = string.as_ref();
```

[`AsRef`]/[`AsMut`] are commonly implemented for smart pointers to allow referring a data behind it via regular [Rust] references.

To better understand [`AsRef`]/[`AsMut`]'s purpose, design, limitations and use cases, read through:
- [Official `AsRef` docs][`AsRef`]
- [Official `AsMut` docs][`AsMut`]
- [Ricardo Martins: Convenient and idiomatic conversions in Rust][10]


### Difference from [`Borrow`]

Novices in [Rust] are often confused with the fact that [`AsRef`]/[`AsMut`] and [`Borrow`]/[`BorrowMut`] traits have the same signatures, because it may not be clear which trait to use or implement for their needs.

See [explanation in `Borrow` trait docs][`Borrow`]:

> Further, when providing implementations for additional traits, it needs to be considered whether they should behave identical to those of the underlying type as a consequence of acting as a representation of that underlying type. Generic code typically uses `Borrow<T>` when it relies on the identical behavior of these additional trait implementations. These traits will likely appear as additional trait bounds.
> 
> In particular `Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values: `x.borrow() == y.borrow()` should give the same result as `x == y`.
> 
> If generic code merely needs to work for all types that can provide a reference to related type `T`, it is often better to use `AsRef<T>` as more types can safely implement it.

And [another one in `AsRef` trait docs][`AsRef`]:

> - Unlike `AsRef`, `Borrow` has a blanket impl for any `T`, and can be used to accept either a reference or a value.
> - `Borrow` also requires that `Hash`, `Eq` and `Ord` for a borrowed value are equivalent to those of the owned value. For this reason, if you want to borrow only a single field of a struct you can implement `AsRef`, but not `Borrow`.

So, as a conclusion:
- [`AsRef`]/[`AsMut`] means that the implementor type may be represented as a reference to the implemented type. More like one type contains another one, or is just generally reference-convertible to the one.
- [`Borrow`]/[`BorrowMut`] means that the implementor type is equivalent to the implemented type in its semantics, differing only in how its data is stored. More like one type is just a pointer to another one.

For example, it's natural for an `UserEmail` type to implement `Borrow<str>`, so it may be easily consumed in the code accepting `&str` (converted to `&str`), as they're semantically equivalent regarding `Hash`, `Eq` and `Ord`. And it's good for some execution `Context` to implement `AsRef<dyn Repository>`, so it can be extracted and used where needed, without using the whole `Context`.

To better understand [`AsRef`]/[`Borrow`]'s difference, read through:
- [Anup Jadhav: AsRef vs Borrow trait (ft. ChatGPT)][12]


### Inner-to-outer conversion

[`AsRef`]/[`AsMut`] are able to do only outer-to-inner reference conversion, but obviously not the opposite.

```rust
struct Id(u8);

impl AsRef<u8> for Id {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

impl AsRef<Id> for u8 {
    fn as_ref(&self) -> &Id {
        &Id(*self)
    }
}
```
```
error[E0515]: cannot return reference to temporary value
  --> src/lib.rs:11:9
   |
11 |         &Id(*self)
   |         ^---------
   |         ||
   |         |temporary value created here
   |         returns a reference to data owned by the current function
```

However, there is nothing wrong with such conversion as long as memory layout of the inner type is the same for the outer type.

```rust
#[repr(transparent)]
struct Id(u8);

impl AsRef<Id> for u8 {
    fn as_ref(&self) -> &Id {
        unsafe { mem::transmute(self) }
    }
}
```

That's exactly what [`ref-cast`] crate checks and does, without necessity of writing `unsafe` explicitly. See [crate's documentation][`ref-cast`] for more explanations.




## Dereferencing

[`Deref`]/[`DerefMut`] standard library trait __allows to implicitly coerce from a custom type to a reference__ when dereferencing (operator `*v`) is used. The most common example of this is using [`Box<T>`][`Box`] where `&T` is expected.

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

let m = Box::new(String::from("Rust"));
hello(&m);
```

To better understand [`Deref`]'s purpose, design, limitations and use cases, read through:
- [Rust Book: 15.2. Treating Smart Pointers Like Regular References with the Deref Trait][3]
- [Official `Deref` docs][`Deref`]
- [Tim McNamara: Explaining Rust’s Deref trait][13]


### Incorrect usage

The implicit coercion that [Rust] implements for [`Deref`] is a sweet honey pot which may lead you to misuse of this feature.

The common temptation is to use [`Deref`] in a combination with [newtype pattern][4], so you can use your inner type via outer type without any explicit requirements. However, this is considered to be a bad practice, and [official `Deref` docs][`Deref`] clearly states:

> __`Deref` should only be implemented for smart pointers.__

The wider explanation of this bad practice is given in [this SO answer][5] and [`Deref` polymorphism anti-pattern][6] description.




## Casting

For casting between types the [`as` keyword][`as`] is used in [Rust].

```rust
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}
```

However, it supports only a [small, fixed set of transformations][7], and __is [not idiomatic][11] to use when other conversion possibilities are available__ (like [`From`], [`TryFrom`], [`AsRef`]).

See also:
- [Rust By Example: 5.1. Casting][9]
- [Rust Reference: 8.2.4. Type cast expressions][7]




## Task

Реализуйте следующие типы:

1. EmailString — тип, значением которого может быть только допустимая строка адреса электронной почты.
2. Random<T> — интеллектуальный указатель, который при создании принимает 3 значения типа, на который он указывает, и каждый раз случайным образом указывает на одно из них.

Предоставьте реализации преобразования и разыменования для этих типов по вашему выбору, чтобы упростить и сделать более удобным их использование и взаимодействие со стандартными типами.


## Questions


После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [`Как в Rust представлено преобразование значений? Какова связь между ошибочным и безошибочным преобразованием?`](#как-в-rust-представлено-преобразование-значений-какова-связь-между-ошибочным-и-безошибочным-преобразованием)

- How reference-to-reference conversion is represented in [Rust]? How its traits differ? When and which one should be used?
- [`Как в Rust представлено преобразование ссылок? Чем отличаются его трейты? Когда и какой из них следует использовать?`](#как-в-rust-представлено-преобразование-ссылок-чем-отличаются-его-трейты-когда-и-какой-из-них-следует-использовать)


- [`Как в Rust можно осуществить преобразование внутренних ссылок во внешние? Какие для этого необходимы условия?`](#как-в-rust-можно-осуществить-преобразование-внутренних-ссылок-во-внешние-какие-для-этого-необходимы-условия)

- [`Что такое разыменование в Rust? Как его можно использовать не по назначению? Почему его не следует использовать не по назначению?`](#что-такое-разыменование-в-rust-как-его-можно-использовать-не-по-назначению-почему-его-не-следует-использовать-не-по-назначению)

- Why using [`as`] keyword is not a good practice in [Rust]? Why do we still use it?

<hr>

<h3>Как в Rust представлено преобразование значений? Какова связь между ошибочным и безошибочным преобразованием?</h3>

В Rust преобразование значений реализовано через систему типажных соглашений (trait-based conventions), находящихся в модуле std::convert. Это делает код предсказуемым и идиоматичным.

1. Безошибочное преобразование: From и Into

Эти трейты используются, когда преобразование гарантированно завершится успешно.

- From: Позволяет типу определить, как создать себя из другого типа.
    - Пример: `String::from("hello")`.
- Into: Автоматически реализуется для любого типа, который реализовал   From. Это «обратная сторона» медали.
    - Зачем: Используется в аргументах функций, чтобы принимать любые типы, которые можно превратить в нужный (например, `fn open<P: Into<PathBuf>>(path: P)`).

```rust
// 1. From/Into - безошибочные преобразования
let s: String = String::from("hello");  // From - создание из
let s2: String = "hello".into();        // Into - преобразование в

// Реализация обычно выглядит так:
impl From<&str> for String {
    fn from(s: &str) -> String {
        String::from(s)
    }
}
```

2. Ошибочное преобразование: TryFrom и TryInto

Эти трейты (введены в Rust 1.34) используются для ненадежных преобразований, которые могут вернуть ошибку.

- TryFrom: Возвращает Result<Self, Self::Error>.
    - Пример: Преобразование i64 в u32. Если число отрицательное, вернется ошибка, так как u32 не может его хранить.
- TryInto: Как и в случае с Into, реализуется автоматически, если реализован TryFrom.

```rust
use std::convert::TryFrom;
use std::num::TryFromIntError;

// Пример: преобразование числа с проверкой переполнения
impl TryFrom<i32> for u8 {
    type Error = TryFromIntError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value <= 255 {
            Ok(value as u8)
        } else {
            Err(TryFromIntError)
        }
    }
}

let x: i32 = 256;
match u8::try_from(x) {
    Ok(v) => println!("Успех: {}", v),
    Err(e) => println!("Ошибка: {}", e),  // Сработает здесь
}
```

3. Связь между ними

Связь между этими парами трейтов иерархическая и логическая:

1. Сходство структуры: TryFrom является "ошибочным" аналогом From. Если From гарантирует результат, то TryFrom заставляет программиста обработать возможный сбой.
2. Эволюция: Часто разработка начинается с TryFrom (когда мы не уверены в данных), но если область допустимых значений типа (инвариант) сужается до такой степени, что ошибка невозможна, тип переписывается на From.

3. Автоматическая реализация (Blanket Implementation):

    - Rust автоматически реализует Into для всех, кто реализовал From.
    - Rust автоматически реализует TryInto для всех, кто реализовал TryFrom.
    - Важный нюанс: Любой тип, реализующий From, автоматически получает реализацию TryFrom (где ошибка — это Infallible, то есть ошибка, которая никогда не случится).

4. Другие виды преобразований

    - AsRef / AsMut: Дешевое преобразование ссылки в ссылку. Например, &String в &str. Это не меняет тип данных, а лишь дает другой «взгляд» на них.
    - Deref Coercion: Неявное преобразование умных указателей (например, Box<T> в &T), которое происходит автоматически при вызове методов.

Резюме: что выбрать?

|Трейт|Тип преобразования|Результат|Потребление данных|
|-----|------------------|---------|------------------|
|From / Into|Безопасное|T (значение)|Да (By value)|
|TryFrom / TryInto|Опасное|Result<T, E>|Да (By value)|
|AsRef / AsMut|Ссылка|&T (ссылка)|Нет (By reference)|

Если вы пишете библиотеку, всегда предпочитайте реализацию From, так как она дает Into бесплатно и считается признаком хорошего тона в Rust.

<hr>

<h3>Как в Rust представлено преобразование ссылок? Чем отличаются его трейты? Когда и какой из них следует использовать?</h3>

Преобразование ссылок в Rust представлено трейтами, которые позволяют получить доступ к данным под другим «углом» (типом), не потребляя (не перемещая) исходное значение. В отличие от From/Into, эти преобразования дешевы, так как работают только с указателями.

Основные трейты: AsRef, AsMut, Borrow и BorrowMut.

1. AsRef и AsMut

Это самые распространенные трейты для преобразования ссылок.

- `AsRef<T>`: Позволяет рассматривать тип как ссылку на T.
- `AsMut<T>`: То же самое, но для изменяемых ссылок.

Когда использовать:

Обычно используется в обобщениях (generics), чтобы сделать функцию гибкой к входным типам. Например, если функция принимает путь к файлу, она может принимать всё, что превращается в &Path.

```rust
use std::path::Path;

// Функция примет &str, String, PathBuf и &Path
fn print_path<P: AsRef<Path>>(path: P) {
    let p = path.as_ref();
    println!("{:?}", p);
}
```

2. Borrow и BorrowMut

Эти трейты похожи на AsRef, но накладывают более строгие ограничения.

- Borrow<T>: Позволяет заимствовать значение как тип T.

Ключевое отличие от AsRef:

Borrow требует, чтобы реализации Eq, Ord и Hash для заимствованного типа совпадали с реализациями исходного типа.

Когда использовать:

Почти исключительно в контексте хеш-таблиц (HashMap) или ассоциативных массивов.

- Если у вас `HashMap<String, i32>`, вы хотите иметь возможность искать значение по `&str`, не создавая каждый раз новую String. Для этого String реализует `Borrow<str>`.

3. Сравнение трейтов

|Трейт|Основная цель|Ограничения|Пример|
|-----|-------------|-----------|------|
|AsRef|Гибкость аргументов|Нет (любое преобразование ссылки)|String -> &[u8]|
|Borrow|Ключи в коллекциях|Хеш и Сравнение должны совпадать|String -> &str|
|Deref|Умные указатели|Неявное преобразование|Box<T> -> &T|

4. Когда и какой выбрать?

- Используйте AsRef, если вы пишете библиотечную функцию и хотите, чтобы пользователю было удобно передавать разные типы (например, &str вместо &String). Это стандарт для «generic» кода.
- Используйте Borrow, если вы создаете свою коллекцию или структуру данных, где важно, чтобы заимствованный ключ вел себя точно так же, как исходный (например, при расчете хеша).
- Используйте Deref, если вы пишете умный указатель (как ваш Random<T> ранее). Deref позволяет компилятору делать неявные преобразования (deref coercion), когда вы вызываете методы.

Краткое правило:

- Нужна гибкость в аргументах функции? → AsRef.
- Нужен поиск по ключу в HashMap? → Borrow.
- Создаете обертку (smart pointer)? → Deref.

Подробности можно найти в официальной документации std::convert и std::borrow.

<hr>

<h3>Как в Rust можно осуществить преобразование внутренних ссылок во внешние? Какие для этого необходимы условия?</h3>

Преобразование «внутренних» ссылок (тех, что указывают на данные внутри структуры) во «внешние» (те, что живут независимо) в Rust напрямую связано с концепцией владения (ownership) и временем жизни (lifetimes).

Поскольку Rust гарантирует безопасность памяти, вы не можете просто «вытащить» ссылку на внутреннее поле, если сама структура будет уничтожена.

Вот основные способы и условия для такого преобразования:

1. Использование механизма Deref (Неявное преобразование)

Если ваша структура является «умным указателем» или оберткой, реализация трейта Deref позволяет компилятору автоматически превращать ссылку на структуру (&Wrapper) в ссылку на её внутреннее содержимое (&Inner).

- Условие: Структура должна владеть данными, на которые выдается ссылка.
- Результат: Внешняя ссылка будет иметь то же время жизни, что и исходная ссылка на структуру.

```rust
use std::ops::Deref;

struct Container(String);

impl Deref for Container {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0 // Возвращаем внутреннюю ссылку как внешнюю
    }
}
```

2. Привязка времени жизни (Lifetime Elision)

Для того чтобы метод структуры мог вернуть ссылку на её внутреннее поле, время жизни возвращаемой ссылки должно быть явно или неявно связано с временем жизни self.

- Условие: Возвращаемая ссылка не может пережить структуру.
- Синтаксис:

```rust
struct Data {
    value: String,
}

impl Data {
    // Внешняя ссылка &'a str привязана к времени жизни &'a self
    fn get_value<'a>(&'a self) -> &'a str {
        &self.value
    }
}
```

3. Преобразование через Cow (Clone-on-Write)

Если вам нужно вернуть ссылку на внутренние данные, но в некоторых случаях этих данных может не быть (и их нужно создать), используется Cow. Это позволяет вернуть либо заимствование (ссылку), либо владение.

4. Легальное «удлинение» времени жизни (Leak)

В исключительных случаях, если вам нужно превратить внутреннюю ссылку во внешнюю со временем жизни 'static (которая никогда не умрет), вы можете использовать Box::leak.

- Условие: Данные перемещаются в кучу и остаются там навсегда (память не будет освобождена).
- Зачем: Полезно для глобальных конфигураций, создаваемых во время выполнения.

```rust
fn make_static(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
```

5. Опасный путь: Self-Referential Structs

Rust по умолчанию запрещает создавать структуры, в которых одно поле хранит ссылку на другое поле той же структуры. Это вызвано тем, что при перемещении структуры в памяти адрес внутреннего поля изменится, и ссылка станет невалидной (dangling pointer).
- Для этого используются специальные библиотеки (например, ouroboros или pin-project) или механизм Pin.

Краткий чек-лист условий:

- Инвариант безопасности: Внешняя ссылка не должна существовать дольше, чем владелец данных.
- Заимствование (Borrow Checker): Пока существует внешняя ссылка на внутреннее поле, вы не можете изменять исходную структуру или перемещать её.
- Стабильность адреса: Если вы планируете перемещать структуру, используйте `Box<T>` или `Pin<T>`, чтобы адрес внутренних данных в куче не менялся.

Если ваша задача — передать данные из одного места в другое без жесткой привязки к родителю, рассмотрите возможность использования `Arc<T>` или `String::clone()`. Иногда копирование данных дешевле и безопаснее, чем сложная борьба с временами жизни.

<hr>

<h3>Что такое разыменование в Rust? Как его можно использовать не по назначению? Почему его не следует использовать не по назначению?</h3>

В Rust разыменование (`dereferencing`) — это операция доступа к данным, на которые указывает ссылка или умный указатель. Она выполняется с помощью оператора `*` или происходит автоматически благодаря механизму `Deref Coercion`.

Однако наличие трейтов `Deref` и `DerefMut` открывает возможности для использования этого механизма «не по назначению».

Что такое разыменование?

Когда у вас есть ссылка &T, вы не работаете с самими данными напрямую, вы работаете с адресом. Операция *my_ref говорит процессору: «перейди по этому адресу и возьми значение».

```rust
let value = 42;
let reference = &value;

// Явное разыменование
let dereferenced = *reference; // 42

fn takes_number(num: i32) {}
takes_number(*reference); // Явно
```

Для умных указателей (например, `Box<T>`, `String`, `Vec<T>`) разыменование работает через трейт std::ops::Deref. Он позволяет обращаться к содержимому обертки так, будто это исходный тип.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

let my_box = MyBox(42);
println!("{}", *my_box); // Разыменование через Deref
```

Как его можно использовать не по назначению?

Использование «не по назначению» — это реализация Deref для типов, которые не являются умными указателями. Это называют «Deref антипаттерном».

Примеры злоупотребления:

1. Эмуляция наследования:

Вы создаете структуру Admin и реализуете Deref, чтобы она возвращала &User. Теперь вы можете вызывать методы User прямо на объекте Admin. Это кажется удобным, но Admin — это не указатель на пользователя, это отдельная сущность.

2. Сокращение пути к полям (Shortcut):

Реализация Deref для сложной структуры, чтобы она возвращала её главное поле (например, Settings возвращает &HashMap). Это делается только ради того, чтобы не писать settings.data.get(), а писать settings.get().

3. Скрытие сложности:

Когда тип A разыменовывается в B, а B в C. Создается цепочка, в которой программист вообще перестает понимать, с каким типом данных он работает в данный момент.


<hr>


[`as`]: https://doc.rust-lang.org/std/keyword.as.html
[`AsMut`]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`Borrow`]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[`BorrowMut`]: https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html
[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
[Rust]: https://www.rust-lang.org
[`ref-cast`]: https://docs.rs/ref-cast
[`std::convert`]: https://doc.rust-lang.org/std/convert/index.html
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html

[1]: https://en.wikipedia.org/wiki/Strong_and_weak_typing
[2]: https://doc.rust-lang.org/std/string/struct.String.html#impl-From%3C%26%27_%20str%3E
[3]: https://doc.rust-lang.org/book/ch15-02-deref.html
[4]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[5]: https://stackoverflow.com/questions/45086595/is-it-considered-a-bad-practice-to-implement-deref-for-newtypes
[6]: https://rust-unofficial.github.io/patterns/anti_patterns/deref.html
[7]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
[8]: https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
[9]: https://doc.rust-lang.org/rust-by-example/types/cast.html
[10]: https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust
[11]: https://rust-lang.github.io/rust-clippy/master/index.html#as_conversions
[12]: https://web.archive.org/web/20240220233335/https://rusty-ferris.pages.dev/blog/asref-vs-borrow-trait
[13]: https://timclicks.dev/article/explaining-rusts-deref-trait
