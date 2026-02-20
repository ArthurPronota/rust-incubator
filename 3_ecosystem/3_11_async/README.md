Шаг 3.11: Асинхронный ввод-вывод, фьючерсы (futures) и акторы (actors)
========================================

__Estimated time__: 2 days

В то время как [threads](../3_10_threads) представляют собой решение для проблем, связанных с процессором, для проблем, связанных с вводом-выводом, традиционно решением является [async (non-blocking) I/O][1].


На данный момент в стандартной библиотеке [Rust] нет асинхронных примитивов, поэтому «по умолчанию» ввод-вывод `std` работает синхронно (блокирует текущий [thread][33]). Однако он предоставляет [core abstractions][`std::future`] для их создания, с помощью которых crates экосистемы (например, [`tokio`]) реализуют и предоставляют примитивы для [async I/O][1].


Важно отметить, что асинхронная разработка в [Rust] [все еще][2] [находится в стадии развития][3]. Именно поэтому сейчас все может быть [довольно громоздким][5], часто [вызывая разочарование][6] (особенно, когда дело касается [абстракций][7]). [wg-async][4] (рабочая группа по асинхронной разработке) работает над тем, чтобы сделать это проще, удобнее, эргономичнее и мощнее в будущем.


## `Future`

Основной примитив асинхронного процесса в [Rust] — это [future abstraction][8] (также часто называемая «promise» в некоторых других языках программирования). Существуют две основные концепции, которые отличают [реализацию фьючерсов в Rust][9] от других языков программирования:

1. Фьючерсы [основаны на опросе][10], а не на отправке. Это означает, что после создания фьючерс не будет автоматически выполняться на месте, а должен быть явно выполнен каким-либо исполнителем (средой выполнения/циклом событий для фьючерсов). __Фьючерс ничего не делает, если его не опрашивают__, поэтому обычно представляет собой [ленивое вычисление][12].

2. Futures имеют [нулевая стоимость][11]. Это означает, что код, написанный на фьючерсах, компилируется в нечто эквивалентное (или лучшее), чем «ручная» реализация, которая обычно использует ручные конечные автоматы и тщательное управление памятью.


В [Rust] предоставляются только базовые определения трейтов в модуле [`std::future`]. Чтобы использовать возможности фьючерсов во всей их полноте, рассмотрите возможность использования крейта [`futures`] (и/или аналогичных, таких как [`futures-lite`], [`futures-time`] и т. д.).

Чтобы лучше понять концепции и дизайн [Rust] futures, ознакомьтесь со следующими материалами:
- [Aaron Turon: Zero-cost futures in Rust][11]
- [Aaron Turon: Designing futures for Rust][9]
- [Rust RFC 2592: `futures_api`][13]
- [Asynchronous Programming in Rust: 2.1. The `Future` Trait][20]
- [Conrad Ludgate: Let's talk about this async][14]

Важно отметить, что до стабилизации [дизайна futures][13] в течение довольно длительного времени в экосистеме [Rust] использовался крейт [`futures@0.1`], что привело к тому, что большая часть экосистемы была построена на его основе. К счастью, на данный момент лишь немногие устаревшие или неработающие крейты все еще используют [`futures@0.1`], и, к счастью, их все еще можно использовать одновременно с современной экосистемой на основе [`std::future`] с помощью [слоя совместимости][15].

### `async`/`.await`

[Ключевые слова `async`/`.await`][16] делают асинхронное программирование гораздо более интуитивным, эргономичным и [решают множество проблем с типами и заимствованиями][19] (что может быть довольно сложно при использовании сырых [`futures`]).


> Используйте `async` перед `fn`, `closure` или `block`, чтобы преобразовать помеченный код в `Future`. Таким образом, код не будет выполняться немедленно, а будет оцениваться только после того, как возвращенный `Future` будет выполнен с помощью `.await`.

[Rust] автоматически [превразает асинхронные функции и блоки в функции, возвращающие `Future`][17], применяя правильные [правила пожизненного захвата и исключения][18] для эргономики синтаксиса.


Хотя [ключевое слово `async` пока не поддерживается в методах трейтов][2], существует крейт `async-trait`, который позволяет это сделать для трейтов, преобразуя их в `Future` в блочном формате (главный недостаток которого — непрозрачность по отношению к автотрейтам, таким как `Send`/`Sync`).

Для лучшего понимания принципов работы, десахаризации, использования и особенностей ключевых слов `async`/`.await`, ознакомьтесь со следующей информацией:
- [Rust RFC 2394: `async_await`][16]
- [Asynchronous Programming in Rust: 3. `async`/`.await`][21]
- [Hayden Stainsby: how I finally understood async/await in Rust (part 1)][63]
- [David Tolnay: Await a minute, why bother?][19]
- [Arpad Borsos: Implementation Details of async Rust][27]
- [Tyler Madry: How Rust optimizes async/await I][29]
- [Tyler Madry: How Rust optimizes async/await II: Program analysis][30]


### Задачи и будящий (Tasks and `Waker`)

Помимо самой [будущей абстракции][8], важно понимать, что такое [асинхронная задача][22]:
> Каждый раз, когда опрашивается объект Future, это происходит в рамках «задачи». Задачи — это объекты Future верхнего уровня, которые были переданы исполнителю.

Когда задача приостанавливается из-за ожидания завершения какой-либо неблокирующей операции (это называется «припарковано»), должен существовать способ сообщить исполнителю о необходимости продолжить опрос этой задачи после завершения операции. Объект [`Waker`] (предоставляемый в [`task::Context`]) служит именно этой цели:

> `Waker` предоставляет метод `wake()`, который можно использовать для того, чтобы сообщить исполнителю о необходимости пробуждения связанной с ним задачи. При вызове метода `wake()` исполнитель знает, что задача, связанная с `Waker`, готова к выполнению, и ее будущее следует проверить еще раз.

Чтобы лучше понять дизайн, использование и особенности [`Waker`], ознакомьтесь со следующими материалами:
- [Official `std::task::Waker` docs][`Waker`]
- [Asynchronous Programming in Rust: 2.2. Task Wakeups with `Waker`][22]
- [Hayden Stainsby: how I finally understood async/await in Rust (part 2)][64]
- [Arpad Borsos: Rust Futures and Tasks][28]


### More reading

- [Matt Sarmiento: Async Rust: Futures, Tasks, Wakers—Oh My!][26]
- [Bert Peters: How does async Rust work][31]
- [Tokio Tutorial: Async in depth][24]
- [Asynchronous Programming in Rust][23]
- [Amos: Understanding Rust futures by going way too deep][25]
- [Hayden Stainsby: how I finally understood async/await in Rust (part 4)][67]
- [Saoirse Shipwreckt: Why async Rust?][69]
- [Saoirse Shipwreckt: Let futures be futures][70]
- [Saoirse Shipwreckt: FuturesUnordered and the order of futures][71]




## Async I/O

Асинхронный ввод-вывод в [Rust] возможен благодаря двум основным компонентам: __[неблокирующим операциям ввода-вывода][1]__, предоставляемым операционной системой, и __асинхронной среде выполнения__, которая оборачивает эти операции в удобные асинхронные абстракции и предоставляет [цикл событий][48] для их выполнения и доведения до завершения.


### Non-blocking I/O

Асинхронное программирование невозможно без поддержки [неблокирующего ввода-вывода][1], которая представлена ​​различными [API] в разных операционных системах, например: [epoll] в [Linux] (или многообещающий [io_uring]), [kqueue] в [macOS]/[iOS], [IOCP] в [Windows].


Низкоуровневые крейты, такие как [`mio`] (обеспечивающий работу [`tokio`]) и [`polling`] (обеспечивающий работу [`async-std`]), предоставляют единый многоплатформенный унифицированный интерфейс для большинства этих [API]. Существуют также низкоуровневые крейты, специализированные на конкретном [API], например [`io-uring`].


Для лучшего понимания этой темы, ознакомьтесь со следующими материалами:
- [Official `mio` crate docs][`mio`]
- [Official `polling` crate docs][`polling`]


### Runtime

Высокоуровневые крейты, такие как [`tokio`] (первый и наиболее зрелый на сегодняшний день) и [`async-std`] (не путать с его названием, оно не является официальным и не связано с `std`, это просто название, выбранное авторами), предоставляют не только [реализацию исполнителя][32] для выполнения [`Future`], но и высокоуровневые [API] для [неблокирующего ввода-вывода][1], [таймеров][`tokio::time`] и [примитивов синхронизации][`tokio::sync`] для использования в асинхронных контекстах ([обычные примитивы синхронизации нельзя использовать между точками `.await`][34], поскольку они заблокируют весь исполнитель в его текущем [потоке][33]).


Все асинхронные среды выполнения [Rust] для [`Future`] реализуют идею [кооперативной многозадачности][35], что означает, что задачи (в нашем случае [`Future`]) добровольно передают управление своей среде выполнения (в нашем случае в точках `.await`), в отличие от [вытесняющей многозадачности][36], где среда выполнения может приостанавливать и брать управление обратно, когда ей это нужно (как в [потоках ОС][33] или [виртуальной машине Erlang][37]). Это дает преимущество точного контроля над тем, что и как выполняется, но имеет недостаток, заключающийся в необходимости проявлять большую осторожность в организации [асинхронных задач][22] (например, [избегать блокировки][39] их синхронными или [процессорно-зависимыми] операциями и [передавать управление вручную][38] в занятых циклах).


Также важно классифицировать асинхронные среды выполнения [Rust] следующим образом:

- __Однопоточные среды__ выполнения, __планирующие и выполняющие [`Future`] только в текущем [потоке][33]__, в котором они выполняются.
_Примеры: [`планировщик текущего потока` tokio][40], [`tokio-uring`], [`futures::executor::LocalPool`]_.


- __Многопоточные__ среды выполнения, планирование и выполнение [`Future`] в [пуле потоков][41]:
    - При __[work-stealing][42]__, когда [`Future`] планируются и выполняются на разных [потоках][33]__, так что один [поток][33] может [забрать и выполнить `Future`, изначально запланированный на другом потоке][43], и в результате рабочая нагрузка распределяется более равномерно по стоимости накладных расходов на синхронизацию ([`Future`] необходимо [`Send`]).

      _Examples: [`tokio`'s multi-thread scheduler][44], [`async-executor`] of [`async-std`], [`futures::executor::ThreadPool`]._
      
      _Примеры: [многопоточный планировщик `tokio`][44], [`async-executor`] из [`async-std`], [`futures::executor::ThreadPool`]_.

    - Using __[thread-per-core][45]__ model, where [`Future`]s are __scheduled on different [threads][33], but never leave their [thread][33] until executed completely__, and so, avoid any synchronization overhead ([`Future`]s are not required to be [`Send`]).  
      _Examples: [`actix-rt`] built on top of multiple [`tokio`'s current-thread schedulers][40], [`glommio`]._

Unfortunately, at the moment, there is no meaningful way to abstract over multiple asynchronous runtimes in [Rust]. That's why authors of the libraries using [non-blocking I/O][1] either stick with a single concrete runtime only ([`tokio`], mostly), or support multiple runtimes via [Cargo features][46].

To better understand this topic, read through:
- [Official `tokio` crate docs][`tokio`]
- [Official `async-std` crate docs][`async-std`]
- [Tokio Tutorial][47]
- [Nick Cameron: What is an async runtime?][59]
- [Sylvain Kerkour: Async Rust: Cooperative vs Preemptive scheduling][60]
- [Sylvain Kerkour: Async Rust: What is a runtime? Here is how tokio works under the hood][61]
- [Hayden Stainsby: how I finally understood async/await in Rust (part 3)][65]
- [Ibraheem Ahmed: Learning Async Rust With Entirely Too Many Web Servers][66]
- [Saoirse Shipwreckt: Thread-per-core][68]
- [Milos Gajdos: Rust tokio task cancellation patterns][72]




## Actors

[Actor model][49] is another very spread and famous [concurrency programming paradigm][50]. It fits quite good for solving major concurrent communication problems, so many languages adopted it as their main [concurrency paradigm][50] (the most famous implementations are [Akka][51] and [Erlang][52]).

> [Actor model][53] was put forth by [Carl Hewitt] in 1973 and it adopts the philosophy that everything is an actor. This is similar to the everything is an object philosophy used by some object-oriented programming languages.
>
> It is inherently asynchronous, a message sender will not block whether the reader is ready to pull from the mailbox or not, instead the message goes into a queue usually called a "mailbox". Which is convenient, but it's a bit harder to reason about and mailboxes potentially have to hold a lot of messages.
>
> Each process has a single mailbox, messages are put into the receiver's mailbox by the sender, and fetched by the receiver.

It's somewhat very similar to and interchangeable with [Communicating Sequential Processes (CSP) model][54], as operates on the same level of abstractions, but the main [difference][55] can be described like this:
> [Actors model][49] represents identifiable processes (actors) with non-identifiable communication (message delivery), while [CSP model][54] represents non-identifiable processes with identifiable communication (channels). To deliver a message in [actors model][49] we should "name" the actor, while in [CSP model][54] we should "name" the channel.

In [Rust], [actor abstraction][49] is __mainly useful for expressing some long-living state__ to communicate with (like [background worker][56] or [WebSocket connection][57], for example).

The most famous [actors][49] implementation in [Rust] is [`actix`]. At the time it was designed, it also served as __a "glue" to unite sync and async worlds__, providing both sync and async [actors][49] implementations. Nowadays, however, using [`spawn_blocking()`][39] is usually a more convenient alternative for this.

[`quickwit-actors`] is another simple implementation of [actors][49], with its own advantages, built [specifically for Quickwit needs][62].

More general-purpose and complex [actors system][49] implementations (similar to [Akka]) are [`bastion`], [`riker`] and [`hydra`].

To better understand [actors'][49] design, concepts, usage and implementations, read through:
- [Karan Pratap Singh: CSP vs Actor model for concurrency][55]
- [Official `actix` crate docs][`actix`]
- [Official `actix` user guide][58]
- [Evance Soumaoro: Efficient indexing with Quickwit Rust actor framework][62]




## More reading

- [Nazmul Idris: Build with Naz: Rust async, non-blocking, concurrent, parallel, event loops, graceful shutdown][73]
- [Willem Vanhulle: Functional async][74]




## Task

Implement an async-driven [CLI] tool, which downloads specified web pages:
```bash
cargo run -p step_3_11 -- [--max-threads=<number>] <file>
```
It must read a list of links from the `<file>`, and then concurrently download a content of each link into a separate `.html` file (named by a link).

`--max-threads` argument must control the maximum number of _simultaneously running threads_ in the program (should default to CPUs number).




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is asynchronous programming? How does it relate to multithreading? Which problems does it solve? What are the prerequisites for its existing?
- How does non-blocking I/O works? How does it differs from blocking I/O?
- What is a [`Future`]? Why do we need it? How does it work in [Rust] and how do its semantics differ from other programming languages? What makes it zero-cost?
- What is `async`/`.await`? How do they desugar into a [`Future`]? Why are they vital for ergonomics?
- What is an asynchronous task? How does it compare to a [`Future`]?
- What is a [`Waker`]? How does it work? Why is it required?
- What is an asynchronous runtime? From which parts does it usually consist?
- What kind of multitasking is represented by [`Future`]s in [Rust]? Which advantages and disadvantages does it have?
- What kinds of asynchronous runtimes do exist in [Rust] regarding multithreading? Which advantages and disadvantages does each one have?
- Why blocking an asynchronous runtime is bad? How to avoid it in practice?
- What are the key points of actor model concurrency paradigm? How may it be useful in [Rust]?




[`actix`]: https://docs.rs/actix
[`actix-rt`]: https://docs.rs/actix-rt
[`async-executor`]: https://docs.rs/async-executor
[`async-std`]: https://docs.rs/async-std
[`async-trait`]: https://docs.rs/async-trait
[`bastion`]: https://www.bastion-rs.com
[`Box`]: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
[`Future`]: https://doc.rust-lang.org/stable/std/future/trait.Future.html
[`futures`]: https://docs.rs/futures
[`futures@0.1`]: https://docs.rs/futures/0.1
[`futures::executor::LocalPool`]: https://docs.rs/futures/latest/futures/executor/struct.LocalPool.html
[`futures::executor::ThreadPool`]: https://docs.rs/futures/latest/futures/executor/struct.ThreadPool.html
[`futures-lite`]: https://docs.rs/futures-lite
[`futures-time`]: https://docs.rs/futures-time
[`glommio`]: https://docs.rs/glommio
[`hydra`]: https://docs.rs/hydra
[`io-uring`]: https://docs.rs/io-uring
[`mio`]: https://docs.rs/mio
[`polling`]: https://docs.rs/polling
[`quickwit-actors`]: https://docs.rs/quickwit-actors
[`riker`]: https://riker.rs
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`std::future`]: https://doc.rust-lang.org/std/future/index.html
[`task::Context`]: https://doc.rust-lang.org/std/task/struct.Context.html
[`tokio`]: https://docs.rs/tokio
[`tokio::sync`]: https://docs.rs/tokio/latest/tokio/sync/index.html
[`tokio::time`]: https://docs.rs/tokio/latest/tokio/time/index.html
[`tokio-uring`]: https://docs.rs/tokio-uring
[`Waker`]: https://doc.rust-lang.org/stable/std/task/struct.Waker.html
[Akka]: https://akka.io
[API]: https://en.wikipedia.org/wiki/API
[Carl Hewitt]: https://en.wikipedia.org/wiki/Carl_Hewitt
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[epoll]: https://en.wikipedia.org/wiki/Epoll
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[io_uring]: https://en.wikipedia.org/wiki/Io_uring
[IOCP]: https://learn.microsoft.com/windows/win32/fileio/i-o-completion-ports
[iOS]: https://en.wikipedia.org/wiki/IOS
[kqueue]: https://en.wikipedia.org/wiki/Kqueue
[Linux]: https://en.wikipedia.org/wiki/Linux_kernel
[macOS]: https://en.wikipedia.org/wiki/MacOS
[Rust]: https://www.rust-lang.org
[Windows]: https://en.wikipedia.org/wiki/Microsoft_Windows

[1]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[2]: https://areweasyncyet.rs#async-extensions
[3]: https://rust-lang.github.io/wg-async/design_docs.html
[4]: https://rust-lang.github.io/wg-async/welcome.html
[5]: https://eta.st/2021/03/08/async-rust-2.html
[6]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo.html
[7]: https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html#waiting-for-better-future
[8]: https://en.wikipedia.org/wiki/Futures_and_promises
[9]: https://aturon.github.io/blog/2016/09/07/futures-design
[10]: http://aturon.github.io/blog/2016/09/07/futures-design#what-worked-the-demand-driven-aka-readiness-based-approach
[11]: https://aturon.github.io/blog/2016/08/11/futures
[12]: https://en.wikipedia.org/wiki/Lazy_evaluation
[13]: https://rust-lang.github.io/rfcs/2592-futures.html
[14]: https://web.archive.org/web/20240917182746/https://conradludgate.com/posts/async
[15]: https://rust-lang.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html
[16]: https://rust-lang.github.io/rfcs/2394-async_await.html
[17]: https://rust-lang.github.io/rfcs/2394-async_await.html#reference-level-explanation
[18]: https://rust-lang.github.io/rfcs/2394-async_await.html#lifetime-capture-in-the-anonymous-future
[19]: https://docs.rs/dtolnay/latest/dtolnay/macro._01__await_a_minute.html
[20]: https://rust-lang.github.io/async-book/02_execution/02_future.html
[21]: https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
[22]: https://rust-lang.github.io/async-book/02_execution/03_wakeups.html
[23]: https://rust-lang.github.io/async-book
[24]: https://tokio.rs/tokio/tutorial/async
[25]: https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep
[26]: https://msarmi9.github.io/posts/async-rust
[27]: https://swatinem.de/blog/async-codegen
[28]: https://swatinem.de/blog/futures-n-tasks
[29]: https://tmandry.gitlab.io/blog/posts/optimizing-await-1
[30]: https://tmandry.gitlab.io/blog/posts/optimizing-await-2
[31]: https://bertptrs.nl/2023/04/27/how-does-async-rust-work.html
[32]: https://tokio.rs/tokio/tutorial/async#executors
[33]: https://en.wikipedia.org/wiki/Thread_(computing)
[34]: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use
[35]: https://en.wikipedia.org/wiki/Cooperative_multitasking
[36]: https://en.wikipedia.org/wiki/Preemption_(computing)
[37]: https://blog.stenmans.org/theBeamBook#CH-Scheduling
[38]: https://docs.rs/tokio/latest/tokio/task/index.html#yield_now
[39]: https://docs.rs/tokio/latest/tokio/task/index.html#blocking-and-yielding
[40]: https://docs.rs/tokio/latest/tokio/runtime/index.html#current-thread-scheduler
[41]: https://en.wikipedia.org/wiki/Thread_pool
[42]: https://en.wikipedia.org/wiki/Work_stealing
[43]: https://tokio.rs/blog/2019-10-scheduler#work-stealing-scheduler
[44]: https://docs.rs/tokio/latest/tokio/runtime/index.html#multi-thread-scheduler
[45]: https://www.datadoghq.com/blog/engineering/introducing-glommio
[46]: https://doc.rust-lang.org/cargo/reference/features.html
[47]: https://tokio.rs/tokio/tutorial
[48]: https://en.wikipedia.org/wiki/Event_loop
[49]: https://en.wikipedia.org/wiki/Actor_model
[50]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[51]: https://doc.akka.io/docs/akka/current/typed/actors.html
[52]: https://www.dmi.unict.it/barba/FOND-LING-PROG-DISTR/PROGRAMMI-TESTI/READING-MATERIAL/shortNotesOnErlang.html
[53]: https://arxiv.org/abs/1008.1459
[54]: https://en.wikipedia.org/wiki/Communicating_sequential_processes
[55]: https://dev.to/karanpratapsingh/csp-vs-actor-model-for-concurrency-1cpg
[56]: https://en.wikipedia.org/wiki/Background_process
[57]: https://levelup.gitconnected.com/websockets-in-actix-web-full-tutorial-websockets-actors-f7f9484f5086
[58]: https://actix.rs/docs/actix/actor
[59]: https://ncameron.org/blog/what-is-an-async-runtime
[60]: https://kerkour.com/cooperative-vs-preemptive-scheduling
[61]: https://kerkour.com/rust-async-await-what-is-a-runtime
[62]: https://quickwit.io/blog/quickwit-actor-framework
[63]: https://hegdenu.net/posts/understanding-async-await-1
[64]: https://hegdenu.net/posts/understanding-async-await-2
[65]: https://hegdenu.net/posts/understanding-async-await-3
[66]: https://ibraheem.ca/posts/too-many-web-servers
[67]: https://hegdenu.net/posts/understanding-async-await-4
[68]: https://without.boats/blog/thread-per-core
[69]: https://without.boats/blog/why-async-rust
[70]: https://without.boats/blog/let-futures-be-futures
[71]: https://without.boats/blog/futures-unordered
[72]: https://cybernetist.com/2024/04/19/rust-tokio-task-cancellation-patterns
[73]: https://developerlife.com/2024/05/19/effective-async-rust
[74]: https://willemvanhulle.tech/blog/streams/func-async
