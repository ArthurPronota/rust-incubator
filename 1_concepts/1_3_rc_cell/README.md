Шаг 1.3: Разделяемое владение и внутренняя изменчивость
==================================================

__Estimated time__: 1 day




## Shared ownership

[Rust] ownership model allows _only one owner of a value_. However, there are situations when multiple ownership is required, and it's important to understand how this can be accomplished.

The key piece is to put a value behind a smart pointer, so the pointer itself can be __cloned many times__ (thus allowing multiple owners), but is __pointing always to the same value__ (thus sharing a value). In [Rust] there is a [`Rc`] (["reference counted"][`std::rc`]) smart pointer for this purpose, and [`Arc`] ("atomic reference counted") for use in multiple threads. Both automatically destroy a value once there are no references left.

The code below won't compile as `a` is owned by `x` and moved to a heap before is passed to `y`:
```rust
struct Val(u8);

let a = Val(5);
let x = Box::new(a);
let y = Box::new(a);
```
```rust
error[E0382]: use of moved value: `a`
 --> src/main.rs:6:22
  |
5 |     let x = Box::new(a);
  |                      - value moved here
6 |     let y = Box::new(a);
  |                      ^ value used here after move
  |
  = note: move occurs because `a` has type `Val`, which does not implement the `Copy` trait
```

However, [`Rc`] allows that:
```rust
let a = Rc::new(Val(5));
let x = Rc::clone(&a);  // does not clone original value,
let y = Rc::clone(&a);  // but rather produces new reference to it
```

The [`Rc`], however, __should be used wisely__ as __won't deallocate memory on references cycle__ which is exactly what a __memory leak__ is. [Rust] is unable to prevent memory leaks at compile time (though makes hard to produce them). If it's still required to have a references cycle, you should use a [`Weak`] smart pointer ("weak reference") in combination with [`Rc`]. [`Weak`] allows to break a references cycle as can refer to a value that has been dropped already (returns `None` in such case). 

To better understand [`Rc`]/[`Weak`]'s purpose, design, limitations and use cases, read through:
- [Rust Book: 15.4. Rc, the Reference Counted Smart Pointer][1]
- [Rust Book: 15.6. Reference Cycles Can Leak Memory][2]
- [Official `std::rc` docs][`std::rc`]




## Interior mutability

[Rust] memory safety is based on the following rules (known as "borrowing rules"):

> Given an object `T`, it is only possible to have one of the following:
> - Having several immutable references (`&T`) to the object (also known as __aliasing__).
> - Having one mutable reference (`&mut T`) to the object (also known as __mutability__).

However, quite often there are situations where these rules are not flexible enough and it's required to have multiple references to a value and yet mutate it. [`Cell`] and [`RefCell`] __encapsulate mutability inside__ (thus called "interior mutability") and __provide interface which can be used through common shared references__ (`&T`). [`Mutex`] and [`RwLock`] serve the same purpose, but in a multi-threaded context.

These containers __allow to overcome [Rust] borrowing rules and track borrows at runtime__ (so called "dynamic borrowing"), which, obviously, leads to less safe code as compile-time errors become runtime panics. That's why one should __use [`Cell`]/[`RefCell`] wisely and only as a last resort__.

To better understand [`Cell`]/[`RefCell`]'s purpose, design, limitations and use cases, read through:
- [Rust Book: 15.5. RefCell and the Interior Mutability Pattern][3]
- [Official `std::cell` docs][`std::cell`]
- [Paul Dicker: Interior mutability patterns][6]
- [David Tolnay: Accurate mental model for Rust’s reference types][8]


### Advanced borrowing patterns

Notably, if the ownership over the value can be expressed separately from its data, the __interior mutability is possible while preserving compile-time borrowing checks and eliminating run-time overhead__, as proven by the [`qcell`] and [`ghost-cell`] crates.

To better understand their design, limitations and use cases, read through:
- [Official `qcell` crate docs][`qcell`]
- [Official `ghost-cell` crate docs][`ghost-cell`]
- [RustBelt: GhostCell: Separating Permissions from Data in Rust][9]




## Shared mutability

The most spread case is a combination of two previous: `Rc<RefCell<T>>` (or `Arc<Mutex<T>>`). This allows to mutate a value by multiple owners.

A real-world example would be a database client object: it _must be mutable_, as mutates its state under-the-hood (opens network connections, manages database sessions, etc), yet _we need to own it in multiple places_ of our code, not a single one.

The following articles may explain you this concept better:
- [Manish Goregaokar: Wrapper Types in Rust: Choosing Your Guarantees][4]
- [Alexandre Beslic: Rust, Builder Pattern, Trait Objects, `Box<T>` and `Rc<T>`][5]




## Avoiding panics and deadlocks

There is a simple rule for omitting deadlocks with [`Mutex`]/[`RwLock`] (applicable for panics with [`Cell`]/[`RefCell`] types too):

> Locking scopes must not intersect in any way.

The following example explains why deadlocks happen:
```rust
let owner1 = Arc::new(Mutex::new("string"));
let owner2 = owner1.clone();

let value = owner1.lock.unwrap();

// owner2 locking scope intersects with owner1 lock's scope.
let value = owner2.lock.unwrap(); 
```

Let's remove the intersection:
```rust
let owner1 = Arc::new(Mutex::new("string"));
let owner2 = owner1.clone();
{
    let value = owner1.lock.unwrap();
    // No intersection as owner1 locking scope ends here.
}
{
    let value = owner2.lock.unwrap();
}
```

That's why, usually, you should __omit to expose `Rc<RefCell<T>>`__ (or `Arc<Mutex<T>>`) __in API__'s, but rather __make them an inner implementation detail__. Doing this way you have full control over all locking scopes inside your methods (no scope can expand to outside), so __ensure that no intersection will happen__, and __expose a totally safe API__.

```rust
#[derive(Clone)]
struct SharedString(Arc<Mutex<String>>);

impl SharedString {
    fn mutate_somehow(&self) {
        let mut val = self.lock.unwrap();
        *val = "another string"
    }
}

let owner1 = SharedString(Arc::new(Mutex::new("string")));
let owner2 = owner1.clone();

// We are mutating the same value here,
// but no locking scopes intersection may happen by design.
// Such API will never deadlock or panic 
// due to runtime violation of borrowing rules.
owner1.mutate_somehow();
owner2.mutate_somehow();
```

And even when there is no possibility to hide lock guards behind API boundary, it may be feasible to try encoding the described property via type system, using zero-sized wrapper types on guards. See the following articles for examples and design insights:
- [Adrian Taylor: Can the Rust type system prevent deadlocks?][7]




## Task

Напишите коллекцию `GlobalStack<T>`, которая представляет собой тривиальный стек без указания размера (может расти бесконечно) и имеет следующую семантику:
- может изменяться через несколько разделяемых ссылок (`&GlobalStack<T>`);
- клонирование не клонирует данные, а только создает указатель, поэтому несколько владельцев могут изменять одни и те же данные.




## Questions

После выполнения всех вышеперечисленных действий вы должны быть в состоянии ответить (и понять, почему) на следующие вопросы.:
- [`Что такое разделяемое? Какую проблему она решает? Какие у неё недостатки?`](#что-такое-разделяемое-какую-проблему-она-решает-какие-у-неё-недостатки)

- [`Что такое внутренняя изменяемость? Зачем она нужна в Rust? Какова её цена?`](#что-такое-внутренняя-изменяемость-зачем-она-нужна-в-rust-какова-её-цена)


- [`Можно ли написать собственный тип с возможностью внутренней изменчивости без использования std? Почему?`](#можно-ли-написать-собственный-тип-с-возможностью-внутренней-изменчивости-без-использования-std-почему)


- [`Что такое разделяемая изменяемость? В каких случаях она наиболее распространена?`](#что-такое-разделяемая-изменяемость-в-каких-случаях-она-наиболее-распространена)


- [`Как обеспечить пользователям доступ к API без возникновения паники/взаимной блокировки при использовании внутренней изменяемости?`](#как-обеспечить-пользователям-доступ-к-api-без-возникновения-паникивзаимной-блокировки-при-использовании-внутренней-изменяемости)

- [`Atomic Types in Rust`](#atomic-types-in-rust)

<hr>

<h3>Что такое разделяемое? Какую проблему она решает? Какие у неё недостатки?</h3>

Совместное владение — это шаблон управления памятью, при котором несколько переменных (владельцев) несут равную ответственность за время жизни одного фрагмента данных. Данные остаются активными до тех пор, пока существует хотя бы один владелец.

Какую проблему это решает?

Совместное владение устраняет ограничения строгого правила единственного владельца в Rust. Оно необходимо, когда:

- Графоподобные структуры данных: В графе или двусвязном списке на один узел могут указывать несколько других узлов. Невозможно определить на этапе компиляции, какой узел должен быть «основным» владельцем.
- Параллелизм: Ситуация, когда нескольким потокам необходимо получить доступ к одним и тем же данным только для чтения, и неясно, какой поток завершится последним.
- Циклические зависимости: Ситуации, когда объекты ссылаются друг на друга таким образом, что создается цикл.

Как это реализовано?

Реализовано с помощью подсчета ссылок:

- Rc<T>: Для однопоточного совместного владения.
- Arc<T>: Для многопоточного (атомарного) совместного владения.

При «клонировании» `Rc` или `Arc` вы не копируете данные; вы создаете новый указатель и увеличиваете внутренний счетчик. Когда указатель удаляется, счетчик уменьшается. Когда счетчик достигает нуля, данные освобождаются.

Какие из последствий это влечет за собой?

Несмотря на свою мощь, совместное владение влечет за собой три основных недостатка:

1. Накладные расходы на производительность
- Память: Каждое выделение памяти для `Rc` или `Arc` требует дополнительного места в куче для хранения счетчиков «сильных» и «слабых» ссылок.
- CPU: Увеличение и уменьшение счетчика занимает время. Для `Arc<T>` это атомарные операции, которые значительно медленнее стандартных операций увеличения, поскольку они должны синхронизироваться между ядрами ЦП.

2. Неизменяемость по умолчанию

Данные внутри `Arc` или `Rc` неизменяемы. Для изменения общих данных необходимо обернуть их в дополнительный слой «внутренней изменяемости» (например, `RefCell<T>`, `Mutex<T>` или `RwLock<T>`), что добавляет дополнительные проверки во время выполнения и потенциально может привести к снижению производительности.

<hr>

<h3>Что такое внутренняя изменяемость? Зачем она нужна в Rust? Какова её цена?</h3>

Внутренняя изменяемость — это шаблон проектирования в Rust, который позволяет изменять данные, даже если у вас есть только неизменяемая ссылка (&T) на них. Он фактически переносит обеспечение соблюдения правил заимствования Rust со времени компиляции на время выполнения.

Почему это необходимо?

Это необходимо, потому что строгий механизм проверки заимствований во время компиляции Rust следует правилу: «Либо одна изменяемая ссылка, либо много неизменяемых ссылок». Хотя это безопасно, иногда это слишком ограничительно для корректной логики:

1. Совместное владение: При использовании `Arc<T>` или `Rc<T>` у вас несколько владельцев. Поскольку данные являются общими, Rust позволяет иметь только неизменяемые ссылки на внутреннее значение. Без внутренней изменяемости общие данные никогда не могли бы быть изменены.
2. Детали реализации: Иногда объекту необходимо обновить внутреннее состояние (например, кэш или журнал) во время операции, которая логически доступна только для чтения извне.
3. Циклические структуры данных: Создание таких структур, как графы или двусвязные списки, часто требует, чтобы узлы указывали друг на друга и обновляли эти указатели после создания узлов.

Как это реализовано?

Это достигается с помощью типов-оберток, которые используют примитив `UnsafeCell<T>` внутри себя:

- `Cell<T>`: Для небольших типов копирования в одном потоке. Нет снижения производительности, но ограниченный API (Типы ограниченные Copy).
- `RefCell<T>`: Для однопоточного использования. Отслеживает заимствования во время выполнения и вызывает панику, если вы нарушаете правила (например, пытаетесь заимствовать изменяемый тип, пока кто-то читает).
- `Mutex<T>` / `RwLock<T>`: Для многопоточного использования. Обеспечивают потокобезопасность, заставляя текущий поток ждать (блокироваться), пока он не сможет безопасно получить «изменяемую» блокировку.

Цена, которую вы платите

Внутренняя изменчивость не «бесплатна»; она предполагает ряд компромиссов:

1. Потеря гарантий компиляции

Вы теряете гарантию «если скомпилируется, значит, безопасно» в отношении заимствований. С `RefCell` ваш код может скомпилироваться идеально, но завершиться с ошибкой (паникой) во время выполнения, если ваша логика случайно попытается заимствовать данные дважды.

2. Снижение производительности во время выполнения

- `RefCell`: Необходимо увеличивать и уменьшать внутренний счетчик каждый раз при заимствовании данных для обеспечения безопасности.
- `Mutex/RwLock`: Значительные накладные расходы из-за синхронизации потоков, переключения контекста и потенциальной «конкуренции за блокировки», когда потоки простаивают, ожидая доступа.

3. Сложность мышления

Это требует от разработчика большей осторожности. Необходимо вручную управлять «временем жизни» заимствований во время выполнения (например, следить за тем, чтобы защитный механизм RefMut был отменен), чтобы избежать взаимоблокировок или паники.

4. Накладные расходы на память

Каждая внутренняя оболочка изменяемости добавляет небольшой объем дополнительной памяти для отслеживания состояния заимствования (целое число для RefCell) или примитива уровня ОС (для Mutex).

Сводная таблица

|Тип|Многопотоковость|Механизм|Штраф|
|----|---------|---------|-------|
|Cell|Single|Bitwise overwrite|Minimal (Copy only)|
|RefCell|Single|Runtime counter|Small (Check + potential Panic)|
|Mutex|Multi|Thread blocking|High (Atomic locks + Waiting)|
|RwLock|Multi|Thread blocking|High (Atomic locks + Waiting)|

Короче говоря, внутренняя изменяемость — это «лазейка», позволяющая использовать гибкие шаблоны, такие как совместно изменяемое состояние, но при этом она жертвует гарантированной стабильностью на этапе компиляции Rust ради проверок во время выполнения и снижения производительности.

<hr>

<h3>Можно ли написать собственный тип с возможностью внутренней изменчивости без использования std? Почему?</h3>

Да, вполне возможно написать собственный тип с внутренней изменяемостью без использования std. Фактически, основные строительные блоки для внутренней изменяемости находятся в основной библиотеке, которая доступна в средах `#![no_std]`.

Почему это возможно

Внутренняя изменяемость в Rust основана на одном примитиве: `core::cell::UnsafeCell<T>`.

- `UnsafeCell` — это языковой элемент: это особый тип, известный компилятору («языковой элемент»). Это единственный допустимый способ сообщить компилятору Rust, что данные, находящиеся за разделяемой ссылкой (`&T`), могут быть изменены.
- Отключает оптимизации: обычно компилятор оптимизирует код, предполагая, что данные за `&T` неизменяемы и никогда не меняются. `UnsafeCell` указывает компилятору отключить эти оптимизации для конкретного места в памяти.
- Доступно в ядре: поскольку `UnsafeCell` и другие базовые ячейки (такие как `Cell` и `RefCell`) определены в `core::cell`, они не требуют стандартной библиотеки.

Как это реализовать

Чтобы создать собственный тип изменяемости внутри ячейки в среде `no_std`, вы оборачиваете свои данные в `UnsafeCell` и предоставляете для них безопасный API.

1. Оберните данные: используйте `UnsafeCell<T>` в качестве поля в вашей структуре.
2. Получите указатель: используйте метод `.get()` для `UnsafeCell`, чтобы получить необработанный указатель (`*mut T`) на внутренние данные.
3. Обеспечьте соблюдение инвариантов (ручная безопасность): поскольку вы используете небезопасный код для изменения данных через разделяемую ссылку, вы должны вручную убедиться, что соблюдаете правила заимствования Rust:
- Однопоточный режим: Гарантируется, что одновременно не существует двух изменяемых ссылок (например, как `RefCell` использует счетчик).
- Многопоточный режим: Используйте атомарные операции (из `core::sync::atomic`) или аппаратные блокировки для предотвращения состояний гонки данных.

Краткое описание альтернатив в `no_std`
|Type|Location|Use Case|
|----|--------|--------|
|UnsafeCell<T>|core::cell|Базовый примитив для всей внутренней изменчивости.|
|Cell<T>|core::cell|Для малых Copy типов; безопасно и просто.|
|RefCell<T>|core::cell|Предоставляет заимствования, проверяемые во время выполнения.|
|Atomic* types|core::sync::atomic|Потокобезопасная внутренняя изменяемость примитивов.|

<hr>

<h3>Что такое разделяемая изменяемость? В каких случаях она наиболее распространена?</h3>

В Rust разделяемая изменяемость — это шаблон, позволяющий нескольким указателям или владельцам одновременно изменять один и тот же фрагмент данных.

По умолчанию система владения Rust применяет строгое правило «изменяемости XOR с псевдонимами»: у вас может быть много ссылок только для чтения (`&T`) или ровно одна изменяемая ссылка (`&mut T`), но никогда не оба одновременно. Разделяемая изменяемость использует внутреннюю изменяемость для безопасного обхода этих ограничений времени компиляции во время выполнения.

Типичные сценарии использования совместно изменяемых данных

Совместно изменяемые данные необходимы всякий раз, когда данные не имеют единого, четко определенного владельца или должны быть изменены из нескольких независимых частей программы:

- <b>Графовые и циклические структуры данных</b>: В графе несколько узлов могут указывать на одного соседа и нуждаться в его обновлении. Поскольку ни один узел не «владеет» соседом, необходимо использовать совместно изменяемую структуру (часто `Rc<RefCell<T>>`), чтобы позволить этим нескольким указателям изменять общий узел.
- <b>Глобальные или общие кэши</b>: Приложения часто используют центральный кэш, доступный для многих потоков или функций. Совместно изменяемая структура позволяет нескольким частям приложения одновременно считывать данные из этого кэша и обновлять его, используя потокобезопасные примитивы, такие как `Arc<Mutex<T>>` или `Arc<RwLock<T>>`.
- <b>Шаблоны наблюдателя и уведомления</b>: Несколько «слушателей» или «наблюдателей» могут нуждаться в обновлении общего состояния или регистрации в общем генераторе событий.
- <b>Фреймворки пользовательского интерфейса и управление состоянием</b>: Компоненты пользовательского интерфейса часто используют общее состояние приложения. Когда один компонент (например, кнопка) обновляет состояние, другие компоненты должны немедленно увидеть это изменение.
- <b>Физические движки и симуляции</b>: В симуляциях, где на один и тот же набор частиц действуют несколько сил или ограничений, общая изменчивость позволяет различным системам симуляции одновременно обновлять свойства частиц (положение, скорость).

Реализация на Rust

Rust предоставляет специальные типы «ячеек» для обработки общей изменяемости в зависимости от среды:

|Context|Implementation Types|Key Characteristics|
|-------|--------------------|-------------------|
|Single-Threaded|Cell<T>, RefCell<T>|Обеспечивает соблюдение правил заимствования во время выполнения; вызовет ошибку, если правила будут нарушены (например, два одновременных заимствования изменяемых объектов).|
|Multi-Threaded|Mutex<T>, RwLock<T>, Atomic*|Использует аппаратные или системные блокировки, чтобы гарантировать, что данные изменяют только один поток одновременно.|

Штрафы и риски

- <b>Накладные расходы во время выполнения</b>: Проверка количества заимствований (в `RefCell`) или получение блокировок (в `Mutex`) добавляют циклы CPU и накладные расходы на память по сравнению со стандартными ссылками.
- <b>Риск паники или взаимоблокировки</b>: В однопоточном коде некорректная логика может вызвать панику во время выполнения. В многопоточном коде неправильный порядок блокировок может привести к постоянной взаимоблокировке.
- <b>Сложность</b>: Часто сложнее понять, «кто» и когда изменяет данные, что может привести к незаметным логическим ошибкам.

<hr>

<h3>Как обеспечить пользователям доступ к API без возникновения паники/взаимной блокировки при использовании внутренней изменяемости?</h3>

Чтобы обеспечить свободные от паники и отсутствие взаимоблокировок при использовании внутренней изменяемости в Rust, необходимо отказаться от «оптимистичных» методов, таких как `borrow()` или lock(), и вместо этого использовать уязвимые шаблоны, ограниченный доступ или атомарное состояние.
Вот лучшие стратегии для обеспечения надежного API:

1. Используйте методы доступа с возможностью ошибок (`try_methods`)

Вместо того чтобы вызывать панику при занятости ресурса, возвращайте `Result` или `Option`. Это возлагает на пользователя ответственность за корректную обработку состояния «занятости».

- Для `RefCell`: используйте `try_borrow_mut()` вместо `borrow_mut()`.
- Для `Mutex`: используйте `try_lock()` вместо `lock()`.

```rust
pub fn update_data(&self, val: i32) -> Result<(), MyError> {
    let mut guard = self.inner.try_lock().map_err(|_| MyError::Busy)?;
    *guard = val;
    Ok(())
}
```

2. Инкапсулируйте мутации (функциональные обновления)

Не раскрывайте пользователю механизмы защиты (`MutexGuard` или `RefMut`). Если пользователь слишком долго удерживает механизм защиты, это приводит к взаимоблокировкам. Вместо этого принимайте замыкание. Это гарантирует, что блокировка будет получена и снята в рамках одного контролируемого вызова функции.

```rust
pub fn with_data<F, R>(&self, f: F) -> R 
where F: FnOnce(&mut T) -> R 
{
    let mut guard = self.inner.lock().expect("Lock poisoned");
    f(&mut *guard) 
    // Guard is automatically dropped here, preventing deadlocks elsewhere
}
```

3. Предпочитайте `Cell` вместо `RefCell`

Если ваш тип данных — `Copy` (например, целые числа или небольшие структуры), используйте `Cell<T>`.
- Почему: `Cell` не использует счетчик заимствований; он выполняет побитовое копирование.
- Гарантия: Физически невозможно, чтобы `Cell` вызвал панику, поскольку он не отслеживает заимствования.

4. Используйте атомарные типы для примитивов

Для простых счетчиков или флагов используйте `std::sync::atomic`.

- <b>Почему</b>: Атомарные типы управляются аппаратным обеспечением CPU. Они не блокируют, не могут вызывать взаимоблокировку и не могут привести к панике. Это наиболее производительная и безопасная форма совместного изменения.

5. Предотвращение взаимоблокировок с помощью порядка блокировки или одиночных мьютексов

Взаимоблокировки возникают, когда поток A ожидает сначала блокировку 1, а затем 2, в то время как поток B ожидает сначала блокировку 2, а затем 1.

- Стратегия «мегаструктуры»: вместо пяти полей, защищенных мьютексом, поместите всю структуру внутрь одного мьютекса. Взаимоблокировка невозможна, если нужно получить только одну блокировку.
- Иерархическая блокировка: если вам необходимо использовать несколько блокировок, строго задокументируйте и соблюдайте порядок (например, «Всегда блокируйте A перед B»).

6. Проектируйте с учетом "Total Functions"

Убедитесь, что ваш API обрабатывает все возможные состояния. Если объект находится в состоянии, в котором изменение невозможно, API должен отражать это через систему типов (например, возвращая None), а не предполагать, что состояние допустимо, и выдавать ошибку, когда это не так.

Сводная таблица
|Риск|Решение|Механизм|
|----|-------|--------|
|Panic|Использовать try_ варианты|Возвращать Result вместо краха.|
|Panic|Использовать Cell|Полностью исключает необходимость проверки заемных средств.|
|Deadlock|Closure-based API|Гарантируется немедленное снятие блокировки.|
|Deadlock|Atomic Types|Синхронизация на аппаратном уровне без блокировок.|
|Deadlock|Крупнозернистая блокировка|Один мьютекс для всей структуры предотвращает циклическое ожидание.|

<hr>

<h3>Atomic Types in Rust</h3>

В Rust атомарные типы обеспечивают низкоуровневый, потокобезопасный доступ к общей памяти без накладных расходов, связанных с тяжелыми механизмами блокировки, такими как мьютекс. Они находятся в модуле `std::sync::atomic` и гарантируют «неделимость» операций — либо значение полностью обновляется, либо остается неизменным.

Основные атомарные типы

Rust определяет атомарные версии большинства примитивных типов. Обратите внимание, что доступность может варьироваться в зависимости от архитектуры процессора (например, на некоторых 32-битных платформах может отсутствовать `AtomicU64`).

- Boolean значение: AtomicBool
- Integers: AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize
- Unsigned Integers: AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize
- Pointers: AtomicPtr<T> (для атомарных операций над необработанными указателями)

Основные операции

В отличие от обычных переменных, атомарные значения нельзя изменять с помощью стандартных операторов (например, +=). Вместо этого они используют специализированные методы, требующие параметра порядка доступа к памяти для определения того, как процессор и компилятор должны обрабатывать синхронизацию:

- <b>load/store</b>: Чтение или запись значения.
- <b>swap</b>: Замена значения и возврат предыдущего.
- <b>fetch_add/fetch_sub</b>: Атомарное добавление или вычитание (возвращает предыдущее значение).
- <b>compare_exchange</b>: Самый мощный примитив; он обновляет значение только в том случае, если оно в данный момент соответствует ожидаемому значению, что является основой для создания пользовательских структур без блокировок.

Порядок использования памяти

Каждая атомарная операция должна указывать порядок использования памяти, чтобы сообщить оборудованию, как синхронизировать память между потоками:

- <b>Relaxed</b>: Отсутствие синхронизации; атомарна только сама операция.
- <b>Acquire / Release</b>: Синхронизация памяти между потоками, которые записывают (Release) и читают (Acquire) одну и ту же переменную.
- <b>SeqCst (Sequentially Consistent)</b>: Самый высокий уровень; гарантирует, что все потоки видят все операции в одном и том же глобальном порядке.

Основные характеристики

- <b>Внутренняя изменяемость</b>: Атомарные типы позволяют изменять данные через общую ссылку (&T), что делает их совместимыми с Arc для совместного использования между потоками.
- <b>Отсутствие блокировок</b>: Обычно они напрямую сопоставляются с инструкциями ЦП, что делает их намного быстрее, чем мьютексы, для простых состояний, таких как счетчики или флаги.
- <b>Спецификация синхронизации</b>: Все атомарные типы реализуют спецификацию синхронизации, что означает, что их безопасно использовать совместно между потоками.

<hr>

[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[`ghost-cell`]: https://docs.rs/ghost-cell
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`qcell`]: https://docs.rs/qcell
[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`Weak`]: https://doc.rust-lang.org/std/rc/struct.Weak.html
[stack]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
[`std::cell`]: https://doc.rust-lang.org/std/cell
[`std::rc`]: https://doc.rust-lang.org/std/rc
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/ch15-04-rc.html
[2]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
[3]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
[4]: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees
[5]: https://abronan.com/rust-trait-objects-box-and-rc
[6]: https://pitdicker.github.io/Interior-mutability-patterns
[7]: https://medium.com/@adetaylor/can-the-rust-type-system-prevent-deadlocks-9ae6e4123037
[8]: https://docs.rs/dtolnay/latest/dtolnay/macro._02__reference_types.html
[9]: https://plv.mpi-sws.org/rustbelt/ghostcell
