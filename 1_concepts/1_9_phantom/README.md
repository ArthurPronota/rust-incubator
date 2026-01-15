Шаг 1.9: Фантомные типы, Ковариантность, Контравариантность, Инвариантность
=======================

__Estimated time__: 1 day

Because [Rust] has a rich type system, a programming logic and semantics are mostly expressed in types rather than in data/values, which is known as a "programming with types" concept. Often, this leads to situations where you need to express some type relations without having any values of those types. Here is where [phantom types][5] come in: they carry some semantics on type level, which invariants are checked by compiler, and are totally compiled out in runtime.

> A phantom type parameter is simply a type parameter which is never used.

However, in [Rust], this often causes the compiler to complain, and the solution is to add a "dummy" use by way of [`PhantomData`].

This is a quite common practice when you're writing a highly abstracted generics code. A real-world example (and somewhat scary) would be:
```rust
trait CommandGateway<C: Command> {
    type Result;
    
    fn command(&self, cmd: C) -> Self::Result;
}

// Here we need to abstract over some types
// to be able to use them in CommandGateway implementation.
pub struct Snapshotter<Repo, AggEv, Err> {
    repo: Repo,
    _aggregate_event: PhantomData<AggEv>,
    _error: PhantomData<Err>,
}

impl<Cmd, Repo, AggEv, Err> CommandGateway<Cmd>
    for Snapshotter<Repo, AggEv, Err>
where
    Cmd: Command + 'static,
    Cmd::Aggregate: VersionedAggregate
        + EventMessageSourced<
            AggEv,
            EventMeta<<Cmd::Aggregate as Aggregate>::Id>,
        >,
    Repo: AggregateRepository<Cmd::Aggregate>
        + EventStore<
            Cmd::Aggregate,
            Event = AggEv,
            EventMeta = EventMeta<<Cmd::Aggregate as Aggregate>::Id>,
        > + Clone
        + 'static,
    AggEv: AggregateEvent + 'static,
    Err: From<
            SnapshotterError<
                <Repo as AggregateRepository<Cmd::Aggregate>>::Error,
                <Repo as EventStore<Cmd::Aggregate>>::Error,
            >,
        > + 'static,
{
    type Result = DynFuture<Option<Cmd::Aggregate>, Err>;
    
    fn command(&self, cmd: Cmd) -> Self::Result {
        let repo = self.repo.clone();
        let fut = repo
            .load(cmd.aggregate_id().unwrap())
            .map_err(AggregateLoadingFailed)
            .map(|agg| agg.unwrap_or_else(Cmd::Aggregate::initial_state))
            .and_then(move |agg| {
                repo.read_events(
                    cmd.aggregate_id().unwrap(),
                    Some(agg.version()),
                )
                .map_err(EventsReadingFailed)
                .fold((agg, false), |(mut agg, _), ev| {
                    agg.apply_event_message(&ev);
                    Ok((agg, true))
                })
                .map(move |(agg, changed)| (agg, changed, repo))
            })
            .and_then(|(agg, has_changed, repo)| {
                if has_changed {
                    Either::A(
                        repo.store(&agg)
                            .map_err(AggregateStoringFailed)
                            .map(move |_| Some(agg)),
                    )
                } else {
                    Either::B(future::ok(None))
                }
            });
        Box::new(fut.map_err(Err::from))
    }
}
```

To better understand [`PhantomData`]'s purpose, design, limitations and use cases, read through:
- [Official `PhantomData` docs][`PhantomData`]
- [Rust By Example: 14.9. Phantom type parameters][1]
- [Rustonomicon: 3.10. PhantomData][2]
- [Reddit: Why PhantomData][3]
- [RIP Tutorial: Using PhantomData as a Type Marker][4]
- [Aayushya Vajpayee: Write Cleaner, More Maintainable Rust Code with PhantomData][11]
- [Sergey Potapov: Phantom Types in Rust][6]




## Transparency

[`PhantomData`] is transparent for [auto traits][7], which means, for example, that `PhantomData<usize>` is `Send` and `Sized`, while `PhantomData<dyn Any>` is neither `Send` nor `Sized`.

In some situations this allows us to provide the exact semantics we need for a type (like [invariance][8] for [a lifetime][9], for example). 

In other situations we don't actually care about semantics of the phantom type parameter at all. Moreover, we don't want the substituted type to change [auto traits][7] implementations of the whole type in any way, preserving only the semantics of the actual contained data, as this may impose ergonomic problems to us:
```rust
struct Nonce<Of>(PhantomData<Of>, usize);

// This compiles OK, as `Nonce<()>` is `Send`.
let nonce: Nonce<()> = Nonce(PhantomData, 1);
thread::spawn(move || {
    println!("{nonce:?}");
});

// This doesn't compile, as `Nonce<Rc<()>>` is not `Send`.
let nonce: Nonce<Rc<()>> = Nonce(PhantomData, 2);
thread::spawn(move || {
    println!("{nonce:?}");
});

// This doesn't compile, as `dyn Any` is not `Sized`.
let nonce: Nonce<dyn Any> = Nonce(PhantomData, 3);
```

To omit such problems, let's just form the correct type inside [`PhantomData`], so we always have the desired [auto traits][7] implementations despite the substituted type:
```rust
struct Nonce<Of: ?Sized>(PhantomData<AtomicPtr<Box<Of>>>, usize);

// This compiles OK now, despite `Rc<()>` is not `Send`.
let nonce: Nonce<Rc<()>> = Nonce(PhantomData, 2);
thread::spawn(move || {
    println!("{nonce:?}");
});

// This compiles OK now, as any `?Sized` type is allowed.
let nonce: Nonce<dyn Any> = Nonce(PhantomData, 3);
```




## Custom phantom type

Interesting enough, despite the [`PhantomData`] being a [lang item][10], it's still possible to define a custom type without using the original [`PhantomData`], but behaving like the one. This is demonstrated quite fairly by the [`ghost`] crate.

```rust
use ghost::phantom;

#[phantom]
#[derive(Copy, Clone, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Crazy<'a, V: 'a, T> where &'a V: IntoIterator<Item = T>;

fn main() {
    let _ = Crazy::<'static, Vec<String>, &'static String>;

    // Lifetime elision.
    let crazy = Crazy::<Vec<String>, &String>;
    println!("{:?}", crazy);
}
```

For more detailed explanation, read through:
- [Official `ghost` crate docs][`ghost`]




## Task

Реализуйте тип `Fact<T>`, который возвращает случайный факт о типе `T`, для которого реализован `Fact<T>`.

```rust
let f: Fact<Vec<T>> = Fact::new();
println!("Fact about Vec: {}", f.fact());
println!("Fact about Vec: {}", f.fact());
```

```
Fact about Vec: Vec is heap-allocated.
Fact about Vec: Vec may re-allocate on growing.
```

## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [Ковариантность, Контравариантность, Инвариантность в Rust](#ковариантность-контравариантность-инвариантность-в-rust)

- [`Зачем в Rust существует PhantomData? Какие проблемы он решает?`](#зачем-в-rust-существует-phantomdata-какие-проблемы-он-решает)

- [Как на практике работает прозрачность PhantomData?](#как-на-практике-работает-прозрачность-phantomdata)

- [`Какие существуют альтернативы PhantomData? Когда их целесообразно использовать?`](#какие-существуют-альтернативы-phantomdata-когда-их-целесообразно-использовать)

<hr>

<h3>Ковариантность, Контравариантность, Инвариантность в Rust<h3>

<h5>Вариантность для времён жизни:</h5>

1. Читайте вслух: 'a: 'b = "'a outlives 'b" = "'a живёт не меньше 'b"
2. Визуализируйте: Представляйте отрезки времени
3. Начинайте с простого: Используйте одно время жизни, добавляйте второе только когда нужно
4. Полагайтесь на компилятор: Он подскажет, когда нужно добавить ограничение

<h5>Итог</h5>

'long: 'short — это гарантия системы типов Rust, что:
- 'long будет валидным как минимум так же долго, как 'short
- Можно безопасно преобразовать `&'long T в &'short T`
- Защищает от висячих указателей и неопределённого поведения

Это ключевой механизм, который позволяет Rust гарантировать безопасность памяти без сборщика мусора!


<h5>Контравариантность (Contravariance) в Rust</h5>

Что такое контравариантность (замена: подтип -> супертип)?

Контравариантность — это свойство типа, которое позволяет использовать более общий тип (супертип) там, где ожидается более конкретный (подтип) (в противоположность ковариантности).

Говоря проще: если Fruit — общий тип, а Apple — конкретный (Apple является Fruit), то:

- Ковариантность: `Container<Apple>` → `Container<Fruit>` (можно использовать контейнер яблок как контейнер фруктов)
- Контравариантность: ``Container<Fruit>` → `Container<Apple>` (можно использовать контейнер фруктов как контейнер яблок)

Где встречается контравариантность в Rust?

Контравариантность в Rust встречается только в одном месте: аргументы функций.

Тип функции: fn(T) -> U

Для типа функции fn(T) -> U:

- Контравариантен по T (аргумент)
- Ковариантен по U (возвращаемое значение)

```text
Иерархия типов:
   Fruit (базовый)
    / \
Apple Orange (производные)

Контравариантность:
  Если нужна функция:   fn(Apple) -> ()
  Можно передать:       fn(Fruit) -> ()   ← более общий тип
```

<h5>Ковариантность (Covariance) в Rust</h5>

Что такое ковариантность (зпмена: супертип -> подтип)?

Ковариантность — это свойство типа, которое позволяет использовать производный (подтип) тип вместо базового (супертипа). В Rust это в основном относится к времени жизни ('a) и отношениям между типами.

Основная идея

Если Dog является подтипом Animal (все собаки — животные), то:

- Ковариантность: `Container<Dog>` можно использовать как `Container<Animal>`
- Говоря о времени жизни: если 'long живет дольше 'short, то &'long T можно использовать как &'short T

<h5>Инвариантность (Invariance) в Rust</h5>

Что такое инвариантность?

Инвариантность — это свойство типа, которое запрещает любые замены через подтипирование. Тип должен совпадать точно, без преобразований.

Если сказать проще: никаких замен — только точное совпадение.

<h5>Таблица вариантности типов в Rust</h5>

|Тип|Вариантность по 'a|Вариантность по T|Объяснение|
|---|------------------|-----------------|----------|
|&'a T|Ковариантен|Ковариантен|Можно использовать более длинное время жизни вместо более короткого и более конкретный тип|
|&'a mut T|Ковариантен|Инвариантен|Время жизни можно удлинить, но тип должен точно совпадать|
|*const T|-|Ковариантен|Как обычная ссылка|
|*mut T|-|Инвариантен|Как изменяемая ссылка|
|`Vec<T>`|-|Ковариантен|Можно использовать `Vec<Derived>` как `Vec<Base>`|
|`Cell<T>`|-|Инвариантен|Внутренняя изменяемость требует точности|
|`UnsafeCell<T>`|-|Инвариантен|Основа внутренней изменяемости|
|`Box<T>`|-|Ковариантен|Умный указатель ведет себя как T|
|fn(T) -> U|-|Контравариантен по T, Ковариантен по U|Функции обратно вариантны по аргументам|
|Rc<T>|-|Ковариантен|Как `Box<T>`|

<hr>

<h3>Зачем в Rust существует PhantomData? Какие проблемы он решает?</h3>

В Rust `PhantomData<T>` — это «нулевой» тип (маркер), который не занимает места в памяти во время выполнения, но сообщает компилятору важную информацию о типах и временах жизни на этапе проверки кода.

Он остается ключевым инструментом для написания безопасных библиотек и работы с unsafe.

<h4>Основные проблемы, которые решает PhantomData:</h4>

1. Неиспользуемые параметры типов (Unused Type Parameters)

Rust запрещает определять структуры с обобщенными типами (`<T>`) или временами жизни (`<'a>`), если они никак не используются в полях структуры. Это необходимо, чтобы компилятор точно знал, как ведет себя тип.

Проблема:
```rust
struct MyStruct<T> {
    id: u32,
    // Ошибка: параметр T не используется
}
```

Решение:

PhantomData «притворяется» полем, использующим этот тип.
```rust
use std::marker::PhantomData;

struct MyStruct<T> {
    id: u32,
    _marker: PhantomData<T>, // Теперь компилятор доволен
}
```

2. Управление вариантностью (Variance)

Вариантность определяет, можно ли заменить один тип на другой (например, подтип на супертип). Это критично для времен жизни.


- Если ваша структура хранит сырой указатель `*const T`, Rust по умолчанию считает его ковариантным (можно заменить один тип на другой).
- Если вы используете `PhantomData<fn(T) -> T>`, вы можете сделать тип инвариантным, что предотвратит опасные преобразования времен жизни в unsafe коде.

3. Управление правилами Drop (Drop Check)

Когда структура удаляется, Rust должен знать, может ли её деструктор безопасно обращаться к данным типа T.

- Если `PhantomData<T>` присутствует в структуре, компилятор считает, что структура «владеет» типом T.
- Это заставляет Drop Checker проверять, что T все еще валиден, когда вызывается деструктор вашей структуры. Это предотвращает использование данных после их освобождения (use-after-free).

4. Маркировка потокобезопасности (`Send / Sync`)

Как мы обсуждали ранее, вы можете добавить PhantomData, чтобы управлять тем, будет ли ваша структура считаться Send или Sync.

- Добавление `PhantomData<Rc<()>>` сделает структуру `!Send`.
- Добавление `PhantomData<*mut u8>` сделает структуру `!Sync`.

<h4>Практический пример: Обертка над сырым указателем</h4>

Допустим, вы пишете обертку над объектом из C, который представлен указателем. Вам нужно, чтобы Rust знал, что эта обертка «привязана» к времени жизни какого-то другого объекта.
```rust
struct RawRef<'a, T> {
    ptr: *const T,
    _lifecycle: PhantomData<&'a T>, // Сообщаем компилятору: "Мы заимствуем T на время 'a"
}
```

Без PhantomData компилятор позволил бы удалить исходный объект T, пока RawRef еще существует. С ним — он выдаст ошибку заимствования.

<h4>Резюме</h4>

PhantomData нужен, чтобы «объяснить» компилятору ваши намерения, которые не выражены явно через обычные поля. Это мост между высокоуровневой системой типов Rust и низкоуровневой реальностью указателей и сырой памяти.

<hr>

<h3>Как на практике работает прозрачность PhantomData?</h3>

На практике «прозрачность» PhantomData означает, что этот тип существует только в <b>воображении компилятора</b>. В рантайме его нет: он не занимает места, не влияет на выравнивание данных и не генерирует никакого машинного кода.

Вот как эта прозрачность работает в различных аспектах:

<h4>1. Нулевой размер (Zero-Sized Type)</h4>

`PhantomData<T>` — это тип с размером 0 байт. Это позволяет добавлять его в любую структуру, не увеличивая её размер в памяти.

```rust
use std::marker::PhantomData;
use std::mem::size_of;

struct Container {
    value: u32,
    _marker: PhantomData<String>, 
}

fn main() {
    // Размер структуры будет равен 4 байтам (только u32)
    // PhantomData полностью "испарился"
    println!("Размер: {} байт", size_of::<Container>()); 
}
```

<h4>2. Влияние на время жизни (Lifetimes)</h4>

Хотя поля физически нет, компилятор ведет себя так, будто оно там есть. Это позволяет «привязать» структуру к данным, которыми она не владеет напрямую.

Практический пример: Обертка над сырым указателем

Представьте, что вы пишете обертку для C-библиотеки. У вас есть только `*const u8`, но вы хотите, чтобы Rust следил за тем, чтобы исходный буфер не был удален.

```rust
struct MySlice<'a> {
    ptr: *const u8,
    len: usize,
    // "Притворяемся", что храним ссылку, чтобы задействовать Borrow Checker
    _marker: PhantomData<&'a u8>, 
}

fn main() {
    let data = vec![1, 2, 3];
    let slice = MySlice {
        ptr: data.as_ptr(),
        len: data.len(),
        _marker: PhantomData,
    };

    // drop(data); // Если раскомментировать, компилятор выдаст ошибку!
    // Благодаря PhantomData он "видит", что slice всё еще заимствует data.
    println!("Длина: {}", slice.len);
}
```

<h4>3. Управление авто-трейтами (Send, Sync)</h4>

PhantomData позволяет «прозрачно» менять свойства структуры в многопоточности.

- Если вы добавите `PhantomData<Rc<u8>>`, ваша структура автоматически станет `!Send` и `!Sync`, потому что `Rc` не является потокобезопасным.
- Это полезно, когда ваша структура работает с локальными для потока ресурсами (например, `OpenGL` контекстом).

<h4>4. Drop Check (Проверка удаления)</h4>

Когда Rust удаляет (drop) структуру, он должен знать, безопасно ли это. Если ваша структура использует unsafe для управления памятью типа T, Rust по умолчанию не знает, что T все еще должен быть "жив" в момент удаления.

Добавляя `PhantomData<T>`, вы явно говорите: «Моя структура логически владеет T». Это заставляет компилятор гарантировать, что T не будет удален раньше, чем деструктор вашей структуры закончит работу.

<h4>5. Состояние API (State Machines)</h4>

PhantomData часто используется для создания типизированных состояний, которые не тратят память.

```rust
struct Open;
struct Closed;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Closed> {
    fn open(self) -> Door<Open> { 
        Door { _state: PhantomData } 
    }
}

impl Door<Open> {
    fn close(self) -> Door<Closed> { 
        Door { _state: PhantomData } 
    }
}
```

Здесь PhantomData позволяет реализовать логику двери на уровне типов. Вы не сможете вызвать close() у уже закрытой двери, и при этом в памяти вообще не будет переменной state.

<h4>Итог</h4>

Прозрачность PhantomData — это способ взломать логику компилятора, не платя за это производительностью. Вы добавляете "фантомное" поле, чтобы Rust учитывал владение, времена жизни или потокобезопасность, но в итоговом бинарном файле этого поля никогда не будет. Документация std::marker::PhantomData.

<hr>

<h3>Какие существуют альтернативы PhantomData? Когда их целесообразно использовать?</h3>

Прямого функционального эквивалента PhantomData не существует, так как это специальное указание для компилятора. Однако, в зависимости от того, какую именно задачу вы решаете, есть альтернативные архитектурные подходы.

1. Пустые кортежные варианты перечислений (Enums)

Если вам нужно пометить тип состоянием (State Pattern), вместо структур с PhantomData можно использовать пустые перечисления.

- Когда использовать: Для реализации «типизированных состояний» (Typestate Pattern), где важно только различие типов на этапе компиляции.
- Плюс: Не нужно создавать экземпляр PhantomData.

```rust
pub struct Open;
pub struct Closed;

pub struct Door<S> {
    // Вместо PhantomData используем само состояние, если оно ZST (Zero Size Type)
    state: S, 
}

impl Door<Closed> {
    pub fn new() -> Door<Closed> { Door { state: Closed } }
}
```

2. Использование `Unique<T>` или `NonNull<T>` (для указателей)

Если вы пишете низкоуровневую обертку над сырыми указателями, вместо `*mut T + PhantomData<T>` часто лучше использовать специализированные типы из `std::ptr`.

- Когда использовать: При написании собственных структур данных (как Vec или Box).
- Тип `NonNull<T>`: Сообщает компилятору, что указатель никогда не равен null и является ковариантным.
- Плюс: Включает оптимизацию ниши (например, `Option<NonNull<T>>` занимает столько же места, сколько и обычный указатель).

3. Поля с нулевым размером (ZST)

Вы можете использовать любой тип нулевого размера (Zero-Sized Type) вместо PhantomData, если вам просто нужно занять место в определении.

- Когда использовать: Если вам нужно, чтобы тип просто присутствовал в структуре, но не влиял на вариантность или Drop Check.
- Пример: Использование пустых массивов `[T; 0]`.

4. Авто-трейты и отрицательные реализации (для Send/Sync)

Если PhantomData использовался только для того, чтобы сделать тип !Send или !Sync, в современном Rust (в ночных сборках или через специальные хаки) можно использовать прямые указания.

- Когда использовать: Когда цель — управление потокобезопасностью.
- Альтернатива: Добавление поля `*mut T` (которое по умолчанию `!Sync`).

Сводная таблица альтернатив

|Задача|Альтернатива|Почему это лучше|
|------|------------|----------------|
|Хранение состояния|Пустые структуры напрямую|Проще синтаксис, не нужно импортировать маркер.|
|Сырые указатели|`std::ptr::NonNull<T>`|Поддерживает оптимизацию Option|
|Вариантность|`fn(T) -> T` (внутри PhantomData)|Единственный надежный способ управлять вариантностью.|
|Drop Check|#[may_dangle] (unsafe)|Более тонкий контроль над проверкой удаления.|

Резюме: когда НЕ использовать альтернативы?

Не стоит заменять PhantomData, если вам нужно управлять вариантностью или Drop Check. Ни один другой механизм в Rust не делает это так же явно и надежно. PhantomData — это стандартный «сигнал» для других разработчиков о том, что в структуре происходит работа с временами жизни или небезопасным кодом.

<hr>


[`ghost`]: https://docs.rs/ghost
[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/rust-by-example/generics/phantom.html
[2]: https://doc.rust-lang.org/nomicon/phantom-data.html
[3]: https://www.reddit.com/r/rust/comments/8oqj14/why_phantomdata
[4]: https://riptutorial.com/rust/example/24109/using-phantomdata-as-a-type-marker
[5]: https://stackoverflow.com/questions/28247543/motivation-behind-phantom-types
[6]: https://www.greyblake.com/blog/phantom-types-in-rust
[7]: https://doc.rust-lang.org/stable/reference/special-types-and-traits.html#auto-traits
[8]: https://docs.rs/variance/0.1.3/src/variance/lib.rs.html#16
[9]: https://docs.rs/variance/0.1.3/src/variance/lib.rs.html#92
[10]: https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item
[11]: https://aayushyavajpayee.substack.com/p/coming-soon
