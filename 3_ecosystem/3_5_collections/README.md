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

It's important to remember, that __iterators (and their adapters) are lazy__. [`Iterator`] does _nothing_, unless its [`next()`][7] method is called. This property leads to the next one: __iterators do not have to be finite__. So, if you need a sort of an infinite collection (like endless [fibonacci sequence][8]), an [`Iterator`] implementation is a way to go, as each new element will be evaluated lazily on request.

[`Iterator`] comes with a lot of powerful and useful [adapters][9] in `std` library, which makes them highly composable and pleasant to use. If `std` capabilities are not enough for your needs, consider to use [`itertools`] crate, which provides more non-trivial adapters.

To better understand [Rust] iterators' purpose, design, limitations and use cases, read through:
- [Rust By Example: 16.4. Iterators][6]
- [Official `std::iter` docs][`std::iter`]




## Immutable collections

[Immutable collections][10] (aka "persistent data structures") are collections which preserve interface and behavior of its mutable analogues, but have a different implementation under-the-hood, which __allows each piece of code to work with its own copy of a whole collection without worrying about accidentally changing elements for others__. The key feature is in implicit data deduplication. This inevitably comes in a price of performance, so immutable collection has [other performance guarantees][11] than mutable ones.

[Rust] ecosystem has [`im`] and [`rpds`] crates, which provide immutable implementations for some collections.

To better understand immutable collections' nature, design, and a motivation behind them, read through:
- [Official `im` crate docs][`im`]
- [Wikipedia: Persistent data structure][10]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 1][15_1]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 2][15_2]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 3][15_3]




## Concurrent collections

When you need to operate with the same collection from multiple threads, the most common and obvious way to go is to put it behind some synchronization primitive (like `Arc<RwLock<VecDeque<T>>>`, for example). However, this _performs too bad_ for an extensive use of a collection. That's why concurrent collections exist: they __allow usage of a collection from multiple threads without explicit synchronization__ and __provide efficient synchronization mechanism under-the-hood__ (usually, leveraging lock-free algorithms).

[Rust] ecosystem has [`crossbeam`] and [`lockfree`] crates, providing efficient lock-free implementations for some collections usually used in a concurrent context. Also, consider [`flurry`] and [`chashmap`] crates for a concurrent [hash map][`HashMap`] implementation.

To better understand concurrent collections' nature, design, and a motivation behind them, read through:
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
