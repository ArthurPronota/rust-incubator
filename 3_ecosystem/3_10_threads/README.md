Шаг 3.10: Многопоточность и параллелизм
=========================================

__Estimated time__: 1 day

Одна из главных целей проектирования [Rust] — это [параллелизм][1]. [Rust] имеет [сильную позицию][2] по этому поводу, в то время как позволяет сосуществовать различным моделям параллельного выполнения.


## Threads

[Rust] имеет встроенную поддержку [собственных потоков][3] в виде модуля [`std::thread`] своей стандартной библиотеки.


Традиционно [threads][3] используются для решения задач, ограниченных [CPU-bound], поскольку они позволяют выполнять задачи параллельно. Однако на практике потоки часто используются и для решения задач, ограниченных [I/O-bound], особенно когда [asynchronous I/O][4] плохо поддерживается (что в настоящее время справедливо для `std` библиотеки [Rust]).


Крейт [`crossbeam`] также предоставляет реализацию [scoped threads][5], которые позволяют заимствовать значения из стека. Они также доступны в виде [`std::thread::scope`], начиная с [Rust] 1.63.

Чтобы лучше понять проектирование потоков в [Rust], концепции, использование и особенности (особенно важна и широко используется [TLS][4]), прочтите следующее:
- [Rust Book: 16.1. Using Threads to Run Code Simultaneously][6]
- [Rust By Example: 20.1. Threads][7]
- [Official `std::thread` docs][`std::thread`]
- [Nicky Meuleman: Multithreading in Rust][29]




## Synchronization

[Синхронизация потоков][11] — это обширная тема, но обычно она осуществляется посредством [атомарных операций][12], общего состояния с [эксклюзивным доступом][13] или посредством [обмена данными между потоками][14]. В [Rust] есть встроенная поддержка всех этих способов.


[Атомарные операции][12] представлены модулем [`std::sync::atomic`] стандартной библиотеки [Rust] (а также, дополнительно, крейтом [`atomic`]).


[Эксклюзивный доступ][13] может контролироваться с помощью примитивов модуля [`std::sync`] стандартной библиотеки [Rust].

Взаимодействие потоков обычно представляется через [каналы][14] и реализовано в модуле [`std::sync::mpsc`] стандартной библиотеки [Rust].


Несмотря на это, существует также крейт [`crossbeam`], предоставляющий более функциональные и оптимизированные примитивы параллелизма и синхронизации. Наиболее примечательным является [`crossbeam-channel`] как [улучшение][15] реализаций канала `std`.


Чтобы лучше понять и ознакомиться с принципами работы, концепциями, использованием и особенностями примитивов синхронизации в [Rust], прочтите следующие материалы:
- [Rust Book: 16.2. Using Message Passing to Transfer Data Between Threads][16]
- [Rust Book: 16.3. Shared-State Concurrency][13]
- [Rust Blog: Fearless Concurrency with Rust][2]
- [Official `std::sync` docs][`std::sync`]
- [Official `std::sync::atomic` docs][`std::sync::atomic`]
- [Official `std::sync::mpsc` docs][`std::sync::mpsc`]
- [Official `atomic` crate docs][`atomic`]
- [Official `crossbeam-channel` crate docs][`crossbeam-channel`]
- [Nicky Meuleman: Multithreading in Rust][29]
- [Carl Fredrik Samson: Explaining Atomics in Rust][26]
- [Gray Olson: The plight of the misunderstood memory ordering][37]
- [Aleksey Kladov: Mutexes Are Faster Than Spinlocks][27]
- [Mara Bos: Comparing Rust's and C++'s Concurrency Library][31]
- [Mahmoud Al-Qudsi: Implementing truly safe semaphores in rust][32]
- [Michael Snoyman: My Best and Worst Deadlock in Rust][35]




## Parallelism

Важно понимать концепцию [различия между параллельным и многопоточным выполнением][21].


Экосистема [Rust] поддерживает параллелизм в виде библиотек [`rayon`], [`dpc-pariter`] и [`fork_union`], которые упрощают преобразование последовательного итератора для _выполнения в параллельных потоках_.

Еще один способ параллельной обработки данных _без использования [потоков][3]_ — это использование инструкций [SIMD]. Если алгоритм достаточно распараллеливаем, применение инструкций [SIMD] может [значительно повысить производительность][24]. Экосистема [Rust] обеспечивает базовую поддержку инструкций [SIMD] в виде крейта [`packed_simd`].


Чтобы лучше понять и освоить параллелизм в [Rust], прочтите следующее:
- [Nicky Meuleman: Concurrent vs parallel][28]
- [Official `rayon` crate docs][`rayon`]
- [`rayon` crate FAQ][22]
- [`rayon` crate demos][23]
- [Kofi Otuo: Implementing data parallelism with Rayon Rust][34]
- [Dawid Ciężarkiewicz: Adding parallelism to your Rust iterators with `dpc-pariter`][30]
- [Official `dpc-pariter` crate docs][`dpc-pariter`]
- [Ash Vardanian: Fork Union: Beyond OpenMP in C++ and Rust?][36]
- [Official `fork_union` crate docs][`fork_union`]
- [Rust Edition Guide: 3.9. SIMD for faster computing][25]
- [Official `packed_simd` crate docs][`packed_simd`]
- [vgatherps: Parsing numbers into base-10 decimals with SIMD][33]




## Task

Write a program with the following workflow:
- `Producer` is a separate thread, which continuously generates square matrixes of random `u8` elements and size `4096`.
- `Consumer` is a separate thread, which takes a generated matrix, counts sum of all its elements and prints the sum to STDOUT.
- There are only 1 `Producer` and 2 `Consumer`s.
- Counting sum of matrix elements should be parallelized.




## Questions


После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [Что такое параллелизм? Что такое конкуренция? Как они связаны друг с другом и чем отличаются?](#что-такое-параллелизм-что-такое-конкуренция-как-они-связаны-друг-с-другом-и-чем-отличаются)

- [Как в Rust реализован параллелизм? Какие крейты обычно используются для его применения?](#как-в-rust-реализован-параллелизм-какие-крейты-обычно-используются-для-его-применения)

- What are the main ways of threads synchronization in [Rust]? Which advantages and disadvantages does each one have? What are the use-cases for each one?

<hr>

### Что такое параллелизм? Что такое конкуренция? Как они связаны друг с другом и чем отличаются?

В Rust разделение этих понятий критично для правильного выбора между __Tokio__ (асинхронность) и __Rayon__ (вычисления).

1. #### Конкуренция (Concurrency) — «Многозадачность»

Это способность программы работать над несколькими задачами одновременно, __переключаясь__ между ними.

- Аналогия: Один повар готовит суп и салат одновременно. Пока суп закипает, он режет овощи. Он не делает два дела в одну секунду, но обе задачи продвигаются.
- В Rust: Реализуется через асинхронность (async/await) и переключение задач (tasks) в одном или нескольких потоках. Основная цель — не блокировать программу, пока она ждет ответа от сети или диска.

2. #### Параллелизм (Parallelism) — «Одновременность»

Это физическое выполнение нескольких операций в один и тот же момент времени на разных ядрах процессора.

- Аналогия: Два повара. Один только варит суп, второй только режет салат. Они работают абсолютно независимо и одновременно.
- В Rust: Реализуется через многопоточность (std::thread) или библиотеки для параллельных вычислений (например, Rayon). Основная цель — ускорить тяжелые вычисления.

3. #### Как они связаны и чем отличаются?

|Характеристика|Конкуренция (Concurrency)|Параллелизм (Parallelism)|
|Суть|Управление множеством задач.|Одновременное выполнение задач.|
|Ресурсы|Может работать на одном ядре.|Требует несколько ядер/процессоров.|
|Типичная задача|Ожидание ввода-вывода (I/O), сеть.|Математика, обработка видео, рендеринг.|
|Инструмент в Rust|tokio, async-std, futures.|std::thread, rayon.|

__Связь__: Параллелизм — это частный случай конкуренции, доступный на многоядерных системах. Вы можете писать конкурентный код (асинхронный), который будет выполняться параллельно (на пуле потоков).

4. #### Роль Rust: Fearless Concurrency

Rust уникален тем, что благодаря системе владения (Ownership) и трейтам Send и Sync, он предотвращает «состояние гонки» (data races) еще на этапе компиляции.

- __Send__: Позволяет передать владение объектом в другой поток.
- __Sync__: Позволяет безопасно обращаться к объекту из нескольких потоков одновременно через ссылки.

#### Когда что использовать?

1. Если ваша программа ждет (базу данных, пользователя, API) — используйте __Конкуренцию__ (async).
2. Если ваша программа считает (матрицы, поиск, сжатие) — используйте __Параллелизм__ (threads/rayon).

<hr>

### Как в Rust реализован параллелизм? Какие крейты обычно используются для его применения?

В Rust параллелизм (Parallelism) реализован через разделение задач между физическими ядрами процессора. В отличие от асинхронности (Concurrency), где задачи «ждут» ввода-вывода, параллелизм в Rust сфокусирован на __максимальной загрузке CPU__.

1. #### Как это реализовано в языке?

Основа параллелизма в Rust — это система владения и два маркера-трейта:

- __Send__: Гарантирует, что владение типом можно безопасно передать в другой поток.
- __Sync__: Гарантирует, что к типу можно безопасно обращаться из нескольких потоков одновременно через ссылки.

Благодаря этим трейтам компилятор Rust запрещает состояние гонки (data races) на этапе сборки. Если вы попытаетесь передать небезопасный объект (например, Rc) в другой поток, код просто не скомпилируется.

2. #### Основные инструменты и крейты

А. Стандартная библиотека (__std::thread__)

Низкоуровневый способ создания потоков ОС.

- Использование: thread::spawn(|| { ... }).
- Когда: Для долгоживущих фоновых задач, которые не требуют частого создания/удаления.
- Минус: Создание потока ОС — дорогая операция.

Б. __Rayon__ (Золотой стандарт для вычислений)

Самый популярный крейт для параллельной обработки данных. Он превращает обычные итераторы в параллельные.

- Как работает: Использует алгоритм work-stealing (кража задач) для равномерной загрузки всех ядер.
- Пример: 
```rust
my_vec.par_iter().map(|x| x * 2).collect() ;
```
- Когда: Обработка массивов, математика, поиск в больших коллекциях [2.1].


В. [Crossbeam](https://docs.rs/crossbeam/latest/crossbeam/) (Низкоуровневые инструменты)

Предоставляет структуры данных и примитивы для эффективного параллельного программирования.

- Фишка: __Scoped threads__. Позволяет запускать потоки, которые могут заимствовать локальные переменные из родительского потока без использования Arc.
- Инструменты: Быстрые каналы (MPMC), Epoch-based GC для lock-free структур.

Г. [DashMap](https://docs.rs/dashmap/latest/dashmap/)

Высокопроизводительная параллельная HashMap. Позволяет множеству потоков читать и писать в одну таблицу без блокировки всей коллекции целиком (через шардирование).

3. #### Сводная таблица выбора

|Задача|Крейт|Почему?|
|------|-----|-------|
|Параллельный цикл / итератор|Rayon|Максимальная простота и авто-балансировка.|
|Обмен данными между потоками|Crossbeam|Самые быстрые и гибкие каналы.|
|Заимствование локальных данных|Crossbeam|Scoped threads решают проблемы лайфтаймов.|
|Сложные вычисления на CPU|Rayon|Эффективное использование пула потоков.|

__Итог__: Для 90% задач параллелизма в Rust достаточно Rayon. Если вам нужно тонкое управление потоками или общая память — используйте Crossbeam и DashMap.

#### Пример как с помощью Rayon ускорить обработку списка в 10 раз всего одной строчкой кода

В Cargo.toml
```Cargo.toml
[dependencies]
rayon = "1.10"
```

```rust
use rayon::prelude::*; // 1. Импортируем трейты Rayon
use std::time::Instant;

fn main() {
    let mut data: Vec<u64> = (0..1_000_000).collect();

    // ПОСЛЕДОВАТЕЛЬНО
    let start = Instant::now();
    let res1: Vec<u64> = data.iter()
        .map(|&x| x * x)
        .collect();
    println!("Обычный итератор: {:?}", start.elapsed());

    // ПАРАЛЛЕЛЬНО (Rayon)
    let start = Instant::now();
    let res2: Vec<u64> = data.par_iter() // 2. Всего четыре символа 'par_' меняют всё!
        .map(|&x| x * x)
        .collect();
    println!("Параллельный итератор: {:?}", start.elapsed());

    assert_eq!(res1, res2);
}
```

Примечание:

1. Расшифровка названия Cargo.toml
    - Cargo: Это имя официального менеджера пакетов и системы сборки Rust. Оно переводится как «груз». Метафора заключается в том, что Rust — это корабль, а библиотеки (crates), которые он везет, — это его груз.
    - TOML: Это расширение формата файла. Оно расшифровывается как __Tom's Obvious Minimal Language__ (Очевидный минималистичный язык Тома — в честь Тома Престона-Вернера, создателя GitHub).

2. Почему именно TOML?

В Rust выбрали этот формат вместо JSON или YAML по трем причинам:

- Читаемость: Он похож на старые .ini файлы, но строго типизирован.
- Комментарии: В отличие от JSON, TOML позволяет писать комментарии через #, что критично для конфигурации.
- Порядок: TOML сохраняет структуру данных в человекопонятном виде, что удобно для Git-диффов.


#### Почему это работает так быстро?

- Work-Stealing (Кража задач): Rayon создает пул потоков (обычно по количеству ядер вашего CPU). Если один поток закончил свою часть работы раньше других, он «крадет» задачи у тех, кто еще занят. Это гарантирует, что все ядра загружены на 100%.
- Zero-Cost Abstraction: Rayon настолько оптимизирован, что накладные расходы на разделение данных между потоками практически незаметны для больших коллекций.
- Безопасность: Если внутри __map()__ вы попытаетесь изменить общую переменную без мьютекса, Rust не скомпилирует этот код, защищая вас от Race Condition.


<hr>

[`atomic`]: https://docs.rs/atomic
[`crossbeam`]: https://docs.rs/crossbeam
[`crossbeam-channel`]: https://docs.rs/crossbeam-channel
[`dpc-pariter`]: https://docs.rs/dpc-pariter
[`fork_union`]: https://docs.rs/fork_union
[`packed_simd`]: https://docs.rs/packed_simd
[`rayon`]: https://docs.rs/rayon
[`std::sync`]: https://doc.rust-lang.org/std/sync/index.html
[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/index.html
[`std::sync::mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`std::thread`]: https://doc.rust-lang.org/std/thread/index.html
[`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[Rust]: https://www.rust-lang.org
[SIMD]: https://en.wikipedia.org/wiki/SIMD

[1]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[2]: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[3]: https://en.wikipedia.org/wiki/Thread_(computing)
[4]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[5]: https://docs.rs/crossbeam/0.7.1/crossbeam/thread/index.html
[6]: https://doc.rust-lang.org/book/ch16-01-threads.html
[7]: https://doc.rust-lang.org/rust-by-example/std_misc/threads.html
[8]: https://doc.rust-lang.org/std/thread/index.html#thread-local-storage
[11]: https://en.wikipedia.org/wiki/Synchronization_(computer_science)#Thread_or_process_synchronization
[12]: https://en.wikipedia.org/wiki/Linearizability
[13]: https://doc.rust-lang.org/book/ch16-03-shared-state.html
[14]: https://en.wikipedia.org/wiki/Channel_(programming)
[15]: ../../archive/Stjepan_Glavina_Designing_a_channel.md
[16]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
[21]: https://stackoverflow.com/a/1050257/1828012
[22]: https://github.com/rayon-rs/rayon/blob/master/FAQ.md
[23]: https://github.com/rayon-rs/rayon/tree/master/rayon-demo
[23]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[24]: https://branchfree.org/2019/02/25/paper-parsing-gigabytes-of-json-per-second
[25]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[26]: https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust
[27]: https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html
[28]: https://nickymeuleman.netlify.app/garden/concurrent-vs-parallel
[29]: https://nickymeuleman.netlify.app/blog/multithreading-rust
[30]: https://dpc.pw/adding-parallelism-to-your-rust-iterators
[31]: https://blog.m-ou.se/rust-cpp-concurrency
[32]: https://neosmart.net/blog/implementing-truly-safe-semaphores-in-rust/
[33]: https://vgatherps.github.io/2022-11-28-dec
[34]: https://blog.logrocket.com/implementing-data-parallelism-rayon-rust
[35]: https://www.snoyman.com/blog/2024/01/best-worst-deadlock-rust
[36]: https://ashvardanian.com/posts/beyond-openmp-in-cpp-rust
[37]: https://www.grayolson.me/blog/posts/misunderstood-memory-ordering
