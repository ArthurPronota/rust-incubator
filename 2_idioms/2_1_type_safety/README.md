Шаг 2.1: Расширенные типы обеспечивают корректность
=======================================

__Estimated time__: 1 day

[Rust] has a rich type system which allows to express our program primitives, entities, notions, logic and semantics mostly in types, rather than in data/values, which is known as a "programming with types" concept. The benefits of this are obvious: the more compiler knows about our problem - the more false programs it will decline. Or, rephrased: __the more we describe about the program in types - the more we reduce the probability for the program to be incorrect__.

"Programming with types" inevitably implies its own idioms and patterns. The most common are described below.




## Newtype

Рассмотрим следующий пример, демонстрирующий возможную ошибку:
```rust
#[derive(Clone)]
struct Post {
    id: u64,
    user_id: u64,
    title: String,
    body: String,
}

fn repost(post: &Post, new_author_id: u64) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Oops!
    new_post
}
```

Проблема возникает потому, что наши сущности выражены в значениях, поэтому компилятор не различает `Post::id` и `Post::user_id`, поскольку они имеют один и тот же тип.

Давайте представим эти сущности в виде типов:
```rust
mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);
}

#[derive(Clone)]
struct Post {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

fn repost(post: &Post, new_author_id: user::Id) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Does not compile!
    new_post
}
```
Теперь компилятор способен полностью исключить подобные ошибки на этапе компиляции и предоставлять достаточно информативные сообщения об ошибках:
```rust
error[E0308]: mismatched types
  --> src/main.rs:27:19
   |
27 |     new_post.id = new_author_id;
   |                   ^^^^^^^^^^^^^ expected struct `post::Id`, found struct `user::Id`
   |
   = note: expected type `post::Id`
              found type `user::Id`
```

This is what is called ["newtype pattern"][1]. [Newtypes][1] are a zero-cost abstraction - __there is no runtime overhead__. Additionally, you may __enforce desired invariants on values of the type__ (for example, `Email` type may allow only valid email address strings to be its values, and another good example is [`uom`] crate). Also, [newtype pattern][1] __makes code more understandable for developers__, as domain knowledge is reflected in types, so is described and documented more explicitly.

The downside of using [newtype pattern][1] is a necessity of writing _more boilerplate code_, because you should provide common traits implementations by yourself (like `Clone`, `Copy`, `From`/`Into`/`AsRef`/`AsMut`), as without them the type won't be ergonomic in use. However, most of them can be _derived automatically_ with `std` capabilities or third-party derive-crates (like [`derive_more`]), so the cost is acceptable in most cases. Furthermore, the excellent [`nutype`] crate pushes this idea even further, aiming to provide the best ergonomics for [newtype pattern][1] without compromising any guarantees it gives.

To better understand [newtype pattern][1], read through:
- [Rust Design Patterns: Newtype][1]
- [Rust By Example: 14.7. New Type Idiom][2]
- [Alexis King: Parse, don’t validate][7] ([ru][7_ru])
- [Stefan Baumgartner: Refactoring in Rust: Abstraction with the Newtype Pattern][10]
- [Official `nutype` crate docs][`nutype`]
- [Angus Morrison: The ultimate guide to Rust newtypes][11]




## Typestates

[Newtype pattern][1] prevents us from invalid use of data. But what about behavior? Can we _enforce some behavioral invariants at compile time_, so compiler is able to _cut off incorrect behavior totally_?

Not always, but _yes_ in some cases. One possible way is to use [typestates][3] to represent (in types) a _sequence of states_ our type is able to be in, and to declare transitions (via functions) between these states. Doing so will allow compiler to __cut off incorrect state transitions at compile time__.

A real-world example of applying this idiom in [Rust] would be the awesome [`state_machine_future`] crate.

To better understand [typestates][3], read through:
- [David Teller: Typestates in Rust][3]
- [Cliff L. Biffle: The Typestate Pattern in Rust][5]
- [Ana Hobden: Pretty State Machine Patterns in Rust][4]
- [Will Crichton: Type-level Programming in Rust][6]
- [Sergey Potapov: Builder with typestate in Rust][8]
- [Azriel Hoh: Compile Time Correctness: Type State][9]
- [Oleksandr Prokhorenko: From 'It Might Work' to 'It Will Work': Typestate in Rust][12]




## Task


Для `Post` типа описанного выше предположим следующее поведение в нашем приложении:
```
+-----+              +-------------+            +-----------+
| New |--publish()-->| Unmoderated |--allow()-->| Published |
+-----+              +-------------+            +-----------+
                           |                          |
                         deny()                    delete()
                           |       +---------+        |
                           +------>| Deleted |<-------+
                                   +---------+
```

Implement this behavior using [typestates idiom][3], so that calling `delete()` on `New` post (or calling `deny()` on `Deleted` post) will be a compile-time error.

Реализуйте это поведение, используя [typestates idiom][3], так чтобы вызов `delete()` для записи `New` (или вызов `deny()` для записи `Deleted`) приводил к ошибке компиляции.



## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [`Почему выражение семантики в типах — это хорошо? Каковы преимущества и недостатки?`](#почему-выражение-семантики-в-типах--это-хорошо-каковы-преимущества-и-недостатки)

- [`Что такое шаблон NewType? Как он работает? Какие гарантии он предоставляет?`](#что-такое-шаблон-newtype-как-он-работает-какие-гарантии-он-предоставляет)

- What is typestates pattern? How does it work? Which guarantees does it give?

<hr>

<h3>Почему выражение семантики в типах — это хорошо? Каковы преимущества и недостатки?</h3>

Выражение семантики через систему типов (часто называемое `Type-Driven Design` или `Strong Typing`) в Rust года является золотым стандартом разработки. Это подход, при котором бизнес-логика и правила безопасности кодируются непосредственно в типах данных, а не в комментариях или проверках во время выполнения.

<h4>Почему это хорошо?</h4>

В Rust типы — это не просто разметка памяти (как в C), а математическое доказательство корректности вашей программы. Компилятор выступает в роли автоматизированного аудитора, который проверяет соблюдение правил до запуска кода.

<h4>Преимущества</h4>

- Исключение логических ошибок (`Compile-time Safety`):

     Используя паттерн `Newtype` (например, `struct UserId(u64)` вместо простого `u64`), вы гарантируете, что никогда случайно не передадите идентификатор пользователя там, где ожидается идентификатор заказа. Это исключает целые классы багов «перепутанных аргументов».

- Самодокументируемый код:

    Сигнатура функции `fn activate_user(id: UnactivatedUserId) -> ActivatedUser` говорит о бизнес-процессе больше, чем страница документации. Вы сразу видите, что на вход нельзя подать уже активированного пользователя.

- Безопасный рефакторинг:

    Если вы меняете логику работы типа, компилятор мгновенно подсветит все места в коде, которые перестали соответствовать новой семантике. Это позволяет менять архитектуру огромных систем с минимальным риском.

- Zero-Cost Abstractions (Нулевая стоимость):

    Такие механизмы, как `PhantomData` или типизированные состояния (`Typestate`), существуют только при компиляции. В итоговом бинарном файле они исчезают, не добавляя ни одного лишнего такта процессору.

- Управление ресурсами через `RAII`:

    Семантика владения гарантирует, что файл будет закрыт, а память освобождена ровно тогда, когда объект выйдет из области видимости, что исключает утечки.

<h4>Недостатки</h4>

- Крутая кривая обучения:

    Разработчику нужно не просто «написать код», а спроектировать систему типов. Это требует понимания `generics`, времен жизни (`lifetimes`) и `trait`, что на начальных этапах замедляет работу.

- «Борьба с компилятором»:

    Иногда строгая семантика типов мешает быстрому прототипированию. Написание кода, который «просто работает», в Rust требует больше усилий, чем в динамических языках (Python/JS).

- Раздувание кода (`Code Bloat`):

    Чрезмерное использование статической диспетчеризации (`generics`) для выражения семантики может увеличить время компиляции и размер бинарного файла из-за мономорфизации.

- Сложность API:

    Для внешних пользователей API может выглядеть перегруженным из-за обилия мелких типов-оберток и параметров времени жизни.

<h4>Резюме: Когда это важно?</h4>

Выражение семантики в типах наиболее выгодно в долгоживущих и критически важных проектах.

- Это стандарт для системного программирования, блокчейн-разработки, облачных сервисов и встроенных систем.
- Это позволяет переложить ответственность за проверку «глупых ошибок» на машину, освобождая человеческий мозг для решения реальных архитектурных задач.

Дополнительно о проектировании типов можно почитать в [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html).

<hr>

<h3>Что такое шаблон NewType? Как он работает? Какие гарантии он предоставляет?</h3>

Шаблон NewType в Rust — это идиома проектирования, при которой вы создаете новую структуру с единственным полем, чтобы «обернуть» существующий тип. Это позволяет создать новый именованный тип, который для компилятора будет полностью отличаться от исходного, хотя в памяти они выглядят идентично.

Этот шаблон остается краеугольным камнем архитектуры надежных систем на Rust.

<h4>Как он работает?</h4>

Обычно NewType реализуется как кортежная структура (`tuple struct`):

```rust
// Обертываем стандартный String в наш собственный тип
struct Email(String);

// Обертываем u32 для разных идентификаторов
struct UserId(u32);
struct ProjectId(u32);

fn main() {
    let user_id = UserId(10);
    let project_id = ProjectId(10);

    // ОШИБКА КОМПИЛЯЦИИ: типы UserId и ProjectId несовместимы, 
    // хотя оба внутри содержат u32.
    // if user_id == project_id { ... } 
}
```
<h4>Какие гарантии он предоставляет?</h4>

1. Типобезопасность (`Type Safety`):
    Самая важная гарантия — защита от логических ошибок. Вы не сможете случайно передать `OrderId` там, где функция ожидает `UserId`. Без `NewType` (используя просто `u64`) такая ошибка была бы обнаружена только в рантайме или через баг-репорты.

2. Инкапсуляция и абстракция:
    Вы можете скрыть детали реализации. Например, если вы решите изменить `UserId` с `u32` на `u64` или на `Uuid`, вам нужно будет изменить только определение структуры и методы её создания. Весь остальной код, работающий с типом `UserId`, останется прежним.

3. Реализация внешних трейтов (`Orphan Rules`):
    В Rust нельзя реализовать «чужой» трейт для «чужого» типа (например, вы не можете реализовать свой трейт для `Vec<T>`).

    - Решение: Оберните `Vec<T>` в NewType. Теперь этот тип — «ваш», и вы можете реализовывать для него любые трейты.

4. Разграничение разрешенных операций: 

    Стандартный `f64` позволяет делить одно число на другое. Но если вы обернете его в struct `Weight(f64)`, вы можете сознательно не реализовывать трейт `Div` для `Weight`. Это гарантирует, что никто в программе не сможет случайно «разделить один вес на другой», если в вашей бизнес-логике это не имеет смысла.



<hr>

[`derive_more`]: https://docs.rs/derive_more
[`nutype`]: https://docs.rs/nutype
[`state_machine_future`]: https://docs.rs/state_machine_future
[`uom`]: https://docs.rs/uom
[Rust]: https://www.rust-lang.org

[1]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[2]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[3]: https://yoric.github.io/post/rust-typestate
[4]: https://hoverbear.org/2016/10/12/rust-state-machine-pattern
[5]: https://cliffle.com/blog/rust-typestate
[6]: https://willcrichton.net/notes/type-level-programming
[7]: https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate
[7_ru]: https://habr.com/ru/post/498042
[8]: https://www.greyblake.com/blog/builder-with-typestate-in-rust
[9]: https://peace.mk/blog/compile-time-correctness-type-state
[10]: https://fettblog.eu/refactoring-rust-abstraction-newtype
[11]: https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes#write-ergonomic-newtype-constructors-with-from-and-tryfrom
[12]: https://minikin.me/blog/typestate-in-rust
