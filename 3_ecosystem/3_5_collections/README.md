Шаг 3.5: Коллекции и итераторы
===================================

__Estimated time__: 1 day




## `std` коллекции

[Rust] предоставляет [реализации для часто используемых коллекций][`std::collections`] в своей библиотеке `std`. Они имеют [различные гарантии][2] и предназначены для [различных целей][1], и обычно применимы для 90% случаев использования.


Чтобы лучше понять назначение, дизайн, ограничения и варианты использования [`std::collections`], ознакомьтесь со следующими материалами:
- [Rust Book: 8. Common Collections][5]
- [Rust By Example: 19.2. Vectors][3]
- [Rust By Example: 19.7. HashMap][4]
- [Official `std::collections` docs][`std::collections`]




## Итераторы

> Итераторы широко используются в идиоматическом коде Rust, поэтому стоит с ними ознакомиться.

В то время как коллекция представляет собой полный набор данных, итератор — это способ итерации по её элементам.

> Итератор имеет метод [`next`][7], который при вызове возвращает `Option<Item>`. [`next`][7] будет возвращать `Some(Item)` до тех пор, пока есть элементы, и как только все они будут исчерпаны, вернет `None`, указывая на завершение итерации. Отдельные итераторы могут возобновить итерацию, поэтому повторный вызов [`next`][7] может в какой-то момент снова начать возвращать `Some(Item)`.

>
> Итераторы также являются составными, и их часто объединяют в цепочки для выполнения более сложных видов обработки.

В [Rust] существует три способа итерации по коллекции:
- Функция `iter()` перебирает заимствованные элементы (`&T`), поэтому используется для операций чтения с коллекцией.
- Функция `iter_mut()` выполняет итерацию по _изменяемо заимствованным_ элементам (`&mut T`), поэтому используется, когда требуется изменение элементов на месте.
- Функция `into_iter()` перебирает элементы, находящиеся в собственности (`T`), поэтому используется, когда требуется преобразование всей коллекции и/или перемещение элементов.

Важно помнить, __что итераторы (и их адаптеры) являются ленивыми__. [`Итератор`] ничего не делает, если не вызван его метод [`next()`][7]. Это свойство приводит к следующему: __итераторы не обязательно должны быть конечными__. Поэтому, если вам нужна своего рода бесконечная коллекция (например, бесконечная [последовательность Фибоначчи][8]), реализация с помощью [`Итератора` — это подходящий вариант, поскольку каждый новый элемент будет вычисляться лениво по запросу.

[`Iterator`] поставляется с множеством мощных и полезных [адаптеров][9] в библиотеке `std`, что делает их очень компонуемыми и удобными в использовании. Если возможностей `std` недостаточно для ваших нужд, рассмотрите возможность использования крейта [`itertools`], который предоставляет больше нетривиальных адаптеров.


Чтобы лучше понять назначение, дизайн, ограничения и варианты использования итераторов в Rust, ознакомьтесь со следующей информацией:
- [Rust By Example: 16.4. Iterators][6]
- [Official `std::iter` docs][`std::iter`]




## Неизменяемые коллекции

[Неизменяемые коллекции][10] (также известные как «постоянные структуры данных») — это коллекции, которые сохраняют интерфейс и поведение своих изменяемых аналогов, но имеют другую внутреннюю реализацию, которая __позволяет каждому фрагменту кода работать со своей собственной копией всей коллекции, не беспокоясь о случайном изменении элементов для других__. Ключевой особенностью является неявная дедупликация данных. Это неизбежно происходит за счет производительности, поэтому неизменяемые коллекции имеют [другие гарантии производительности][11], чем изменяемые.

В экосистеме [Rust] есть crates [`im`] и [`rpds`], которые предоставляют неизменяемые реализации для некоторых коллекций.

Чтобы лучше понять природу, структуру и мотивацию создания неизменяемых коллекций, ознакомьтесь со следующей информацией:
- [Official `im` crate docs][`im`]
- [Wikipedia: Persistent data structure][10]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 1][15_1]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 2][15_2]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 3][15_3]




## Совместно используемые коллекции (Concurrent collections)

Когда необходимо работать с одной и той же коллекцией из нескольких потоков, наиболее распространенный и очевидный способ — это использовать для этого примитив синхронизации (например, `Arc<RwLock<VecDeque<T>>>`). Однако это _слишком плохо_ работает при интенсивном использовании коллекции. Именно поэтому существуют параллельные коллекции: они _позволяют использовать коллекцию из нескольких потоков без явной синхронизации_ и _обеспечивают эффективный механизм синхронизации внутри_ (обычно, используя алгоритмы без блокировок).

В экосистеме [Rust] есть крейты [`crossbeam`] и [`lockfree`], предоставляющие эффективные реализации без блокировок для некоторых коллекций, обычно используемых в параллельном контексте. Также рассмотрите крейты [`flurry`] и [`chashmap`] для совместной реализации [хэш-карты][`HashMap`].

Чтобы лучше понять природу, структуру и мотивацию создания параллельных коллекций, ознакомьтесь со следующими материалами:
- [Aaron Turon: Lock-freedom without garbage collection][13]
- [Stjepan Glavina: Lock-free Rust: Crossbeam in 2019][14]
- [Wikipedia: Non-blocking algorithm][12]
- [Ibraheem Ahmed: A Lock-Free Vector][16]
- [Julian Goldstein: Lock-Free Rust: How to Build a Rollercoaster While It’s on Fire][17]




## Task

Write a simple `UsersRepository` trait, which supports 3 operations (consider to chose correct collections):
- returns single `User` by its ID;
- returns multiple `User`s by their IDs;
- return IDs of `User`s which `nickname` contains given string (search function).

Provide an implementation of `UsersRepository` trait backed by some [immutable collection](#immutable-collections).

Prove your implementation correctness with tests.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is a collection? What is an iterator? How do they differ? How are they used? Which limitations does each one have?
- What are immutable collections? How do they work? Why shouldn't we use them all the time? When does it make sense to use them?
- What are concurrent collections? How do they work? Why are they better than explicit synchronization on a normal collection?




[`chashmap`]: https://docs.rs/chashmap
[`crossbeam`]: https://docs.rs/crossbeam
[`flurry`]: https://docs.rs/flurry
[`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
[`im`]: https://docs.rs/im
[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`itertools`]: https://docs.rs/itertools
[`lockfree`]: https://docs.rs/lockfree
[`rpds`]: https://docs.rs/rpds
[`std::collections`]: https://doc.rust-lang.org/std/collections/index.html
[`std::iter`]: https://doc.rust-lang.org/std/iter/index.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection
[2]: https://doc.rust-lang.org/std/collections/index.html#performance
[3]: https://doc.rust-lang.org/rust-by-example/std/vec.html
[4]: https://doc.rust-lang.org/rust-by-example/std/hash.html
[5]: https://doc.rust-lang.org/book/ch08-00-common-collections.html
[6]: https://doc.rust-lang.org/rust-by-example/trait/iter.html
[7]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
[8]: https://en.wikipedia.org/wiki/Fibonacci_number
[9]: https://doc.rust-lang.org/std/iter/index.html#adapters
[10]: https://en.wikipedia.org/wiki/Persistent_data_structure
[11]: https://docs.rs/im/#performance-notes
[12]: https://en.wikipedia.org/wiki/Non-blocking_algorithm
[13]: https://aturon.github.io/blog/2015/08/27/epoch
[14]: ../../archive/Stjepan_Glavina_Lock-free_Rust_Crossbeam_in_2019.md
[15_1]: https://hypirion.com/musings/understanding-persistent-vector-pt-1
[15_2]: https://hypirion.com/musings/understanding-persistent-vector-pt-2
[15_3]: https://hypirion.com/musings/understanding-persistent-vector-pt-3
[16]: https://ibraheem.ca/posts/a-lock-free-vector
[17]: https://yeet.cx/blog/lock-free-rust
