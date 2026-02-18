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


To better understand and be familiar with [Rust]'s synchronization primitives design, concepts, usage, and features, read through:
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

The important concept to understand is [how concurrency and parallelism differ][21].

[Rust] ecosystem has support for parallelism in form of [`rayon`], [`dpc-pariter`] and [`fork_union`] crates, which make it easy to convert a sequential iterator to _execute in parallel threads_.

Another way to perform parallel data processing _without using [threads][3]_ is [SIMD] instructions usage. If an algorithm is parallelizable enough, applying [SIMD] instructions may [increase performance drastically][24]. [Rust] ecosystem provides basic support for [SIMD] instructions in a form of [`packed_simd`] crate.

To better understand and be familiar with parallelism in [Rust], read through:
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

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is concurrency? What is parallelism? How do they relate to each other and how do they differ?
- How parallelism is represented in [Rust]? Which are common crates for using it?
- What are the main ways of threads synchronization in [Rust]? Which advantages and disadvantages does each one have? What are the use-cases for each one?




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
