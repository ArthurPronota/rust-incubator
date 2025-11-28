Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [the Rust Book][Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, the memory model, and the type and module systems.

Polish your familiarity by completing [Rust By Example] and [Rustlings][rustlings].

Read through [the Cargo Book][Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:

- What memory model does [Rust] have?

Модель памяти rust определяется строгими правилами за соблюдением которых отвечает компилятор.
1. Владение: у каждого фрагмента данных есть один владелец. Когда владелец выходит из области действия, память освобождается (RAII - (Resource Acquisition Is Initialization)).
2. Правила заимствования: в любой момент времени вы можете иметь либо одну изменяемую ссылку, либо любое количество неизменяемых ссылок на фрагмент данных.
3. Время жизни: компилятор отслеживает длительность действия ссылок, чтобы предотвратить появление висячих указателей.

Эта модель предотвращает такие распространенные ошибки, как гонки данных, ошибки использования памяти после освобождения и разыменование нулевого указателя.

- Is it single-threaded or multiple-threaded?

[Rust] поддерживает как однопоточную так и многопоточную моделию.

1. Однопоточная: вы можете написать однопоточное приложение, используя стандартные типы, такие как Rc<T> и RefCell<T>.
2. Многопоточная: [Rust] славится своим «бесстрашным многопоточным выполнением». Компилятор использует свою систему типов для предотвращения гонок данных во время компиляции. Для доступа к общей памяти между потоками требуются специальные потокобезопасные интеллектуальные указатели, такие как Arc<T> (атомарный счётчик ссылок) и Mutex<T> (взаимное исключение). Компилятор предотвратит случайное совместное использование потоков небезопасных типов.

- Is it synchronous or asynchronous?

[Rust] предоставляет стандартные библиотеки и фреймворки, поддерживающие оба стиля программирования:

1. Синхронный: Стандартная библиотека (std::fs, std::net) по умолчанию предоставляет блокирующие функции ввода-вывода. При чтении файла текущий поток ожидает завершения операции.
2. Асинхронный: [Rust] обладает первоклассной поддержкой синтаксиса async/await. Используя внешнюю среду выполнения, такую ​​как Tokio или async-std, вы можете создавать высокоэффективные неблокирующие сетевые сервисы. Это позволяет одному потоку эффективно управлять тысячами одновременных операций.

<hr>

- What runtime does [Rust] have? 

Подход [Rust] к «среде выполнения» отличается высокой гибкостью и контекстно-зависим:
1. Минимальная среда выполнения для основных задач: для базовых инструментов командной строки или системных программ стандартный исполняемый файл [Rust] запускается непосредственно в ОС (используя стандартные службы ОС, такие как выделение/освобождение памяти через malloc и свободные аналоги). Основная «среда выполнения» — это всего несколько строк установочного кода для инициализации стека и вызова функции main().
2. Отсутствие виртуальной машины: в отличие от таких языков, как Java или Python, [Rust] не работает на виртуальной машине (ВМ), которая управляет выполнением или сборкой памяти.
3. Гибкие среды выполнения для параллельной работы: если вам нужны расширенные функции, такие как асинхронный ввод-вывод (async/await), необходимо явно подключить стороннюю асинхронную библиотеку среды выполнения, например, Tokio или async-std. Эти библиотеки управляют планированием задач и циклами событий, но они скомпилированы в ваш исполняемый файл, а не являются отдельной языковой средой выполнения.

- Does it use a GC (garbage collector)?

Отсутствие сборщика мусора (GC)
1. Управление памятью во время компиляции: система контроля владения и механизм проверки заимствований [Rust] анализируют использование памяти во время компиляции. Эта система точно определяет, где необходимо выделить память и, что особенно важно, где её необходимо освободить.
2. Детерминированное уничтожение: память автоматически освобождается точно в тот момент, когда её владелец выходит из области видимости (через трейт Drop), что представляет собой детерминированный процесс, называемый RAII (Resource Acquisition Is Initialization).

<hr>

- What does static typing mean? 

В Rust статическая типизация означает, что тип каждой переменной, параметра функции и выражения определяется и проверяется компилятором перед запуском программы.
Rust изначально является языком со статической типизацией. Хотя он часто использует вывод типов, чтобы позволить опускать явные аннотации типов (например, let x = 5; компилятор знает, что x — это i32), базовые типы всегда фиксируются во время компиляции.

- What is a benefit of using it?

Ключевые преимущества статической типизации в Rust

Преимущества статической типизации в Rust усиливаются благодаря уникальной модели владения, которая обеспечивает превосходные гарантии безопасности, производительности и параллелизма по сравнению с другими языками:

1. Гарантирует безопасность памяти и потоков во время компиляции
Это самое значительное преимущество Rust. Компилятор использует информацию о типах для обеспечения соблюдения правил владения и заимствования.

- Преимущество: Rust может гарантировать, что вы не будете обращаться к памяти после её освобождения (ошибки использования после освобождения) или что несколько потоков одновременно изменят одни и те же данные (гонки данных). Эти ошибки просто приводят к сбою компиляции программы, не давая пользователю достичь её в рабочей среде.

2. Абстракции с нулевой стоимостью и высокая производительность
Поскольку все типы известны во время компиляции, Rust избегает накладных расходов во время выполнения.

- Преимущество: Скомпилированный код высоко оптимизирован, поскольку программе не требуется среда выполнения или виртуальная машина для определения типа обрабатываемых данных. Это позволяет Rust работать со скоростью, сравнимой с C и C++.

3. Беспрепятственный параллелизм
Строгая система статических типов Rust обеспечивает беспрецедентный параллелизм. Типы Send и Sync — это маркерные признаки, автоматически применяемые к типам, чтобы указать, можно ли их безопасно использовать совместно или передавать между потоками.

- Преимущество: Компилятор откажется компилировать код, который пытается передать потоконебезопасный тип (например, Rc<T>) через границы потоков. Это статически предотвращает подавляющее большинство ошибок параллельного выполнения — гарантия, которую могут предложить немногие другие языки.

4. Надёжное понимание кода и надёжный рефакторинг
Явная природа типов служит отличной, проверенной машиной документацией.

- Преимущество: Это упрощает поддержку крупномасштабных проектов. При изменении сигнатуры функции компилятор мгновенно подсвечивает каждое место в кодовой базе, требующее обновления, гарантируя, что ничего не будет упущено.


<hr>

- What are generics and parametric polymorphism? 

В Rust дженерики являются основным механизмом, используемым для достижения параметрического полиморфизма.

Эти концепции позволяют разработчикам писать код, работающий с различными типами данных, без необходимости повторять код для каждого конкретного типа.

Что такое дженерики в Rust?

Дженериками называют плейсхолдеры (обычно это отдельные заглавные буквы, например, T для Type или U), представляющие абстрактные типы или поведения. Вы определяете функции, структуры, перечисления или трейты для работы с этим плейсхолдером. При компиляции кода компилятор заменяет плейсхолдер конкретными типами, с которыми код фактически используется.
Что такое параметрический полиморфизм?

Параметрический полиморфизм — это формальный термин в информатике для обозначения этой возможности. Это означает, что функция или структура данных написана в дженерике таким образом, что её поведение параметризуется типами, с которыми она работает. Код ведёт себя одинаково независимо от типа, пока тип соответствует определённым ограничениям (например, реализует требуемый трейт).

Пример обобщений

Основным примером является перечисление Option<T> стандартной библиотеки:
```rust
enum Option<T> {
    Some(T),
    None,
}
```
Здесь T — параметр универсального типа. Option<T> может содержать любой тип (i32, String, пользовательскую структуру и т. д.) внутри варианта Some.

Другой распространённый пример — написание функции, которая находит наибольший элемент в списке:

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        // This comparison works because we constrained T to require the 'PartialOrd' trait
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This works for integers:
let number_list = vec![34, 50, 25, 100, 65];
let result = largest(&number_list); // T becomes i32

// And it works for characters:
let char_list = vec!['y', 'm', 'a', 'q'];
let result = largest(&char_list); // T becomes char
```

- Which problems do they solve?

Дженерик решает проблему дублирования кода, сохраняя при этом типобезопасность и производительность.

1. Предотвращает дублирование кода (принцип DRY - Don't Repeat Yourself, DRY)
Без дженериков пришлось бы писать отдельные функции largest_i32(), largest_f64(), largest_String() и так далее, повторяя каждый раз одну и ту же логику.
Дженерик придерживается принципа «Не повторяйся» (Don't Repeat Yourself, DRY), уменьшая размер кодовой базы и упрощая её поддержку.

2. Поддерживает строгую статическую типизацию и безопасность
В языке с динамической типизацией (например, Python) можно написать одну функцию, принимающую любые данные, но выяснится, являются ли типы несовместимыми, только во время выполнения кода.
В Rust дженерики поддерживают статическую типизацию: компилятор проверяет ограничения (например, PartialOrd + Copy) во время компиляции. Это гарантирует, что код является одновременно универсальным и безопасным, выявляя ошибки типов на ранних этапах.

3. Нулевые затраты времени выполнения (статическая диспетчеризация)
Во время компиляции Rust использует процесс, называемый мономорфизацией. Компилятор, по сути, создаёт специализированные копии универсальной функции для каждого конкретного используемого типа.

```rust
// The compiler internally generates these behind the scenes:
fn largest_i32(list: &[i32]) -> i32 { /* ... */ }
fn largest_char(list: &[char]) -> char { /* ... */ }
```
a) Преимущество: такой подход означает, что использование дженериков не требует никаких затрат во время выполнения. Ваш финальный исполняемый файл выполняется так же быстро, как если бы вы написали каждую функцию вручную, в отличие от языков с динамической диспетчеризацией или объектными типами, которые иногда приводят к снижению производительности при использовании дженериков.

<hr>

- What are traits? How are they used? How do they compare to interfaces? What are auto traits and blanket impls? What is a marker trait?
- What are static and dynamic dispatch? Which should you use, and when?
- What is a crate and what is a module in [Rust]? How do they differ? How are they used?
- What are move semantics? What are borrowing rules? What is the benefit of using them?
- What is immutability? What is the benefit of using it?
- What is cloning? What is copying? How do they compare?
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
- What is an iterator? What is a collection? How do they differ? How are they used?
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macros?
- How is code tested in [Rust]? Where should you put tests and why?
- Why does [Rust] have `&str` and `String` types? How do they differ? When should you use them?
- What are lifetimes? Which problems do they solve? Which benefits do they give?
- Is [Rust] an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance?

_Additional_ articles, which may help to understand the above topic better:
- [George He: Thinking in Rust: Ownership, Access, and Memory Safety][19]
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][12]
- [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker][15]
- [Sabrina Jewson: Why the “Null” Lifetime Does Not Exist][16]
- [HashRust: A guide to closures in Rust][13]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][14]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [Lukasz Uszko: Safe and Secure Coding in Rust: A Comparative Analysis of Rust and C/C++][18]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][10]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][11]
- [Clayton Ramsey: I built a garbage collector for a language that doesn't need one][17]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[11]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[12]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[13]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[14]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[15]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad
[16]: https://sabrinajewson.org/blog/null-lifetime
[17]: https://claytonwramsey.github.io/2023/08/14/dumpster.html
[18]: https://luk6xff.github.io/other/safe_secure_rust_book/intro/index.html
[19]: https://cocoindex.io/blogs/rust-ownership-access
