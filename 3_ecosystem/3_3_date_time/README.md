Шаг 3.3: Дата и время
=======================

__Estimated time__: 1 day

В [Rust] есть простой модуль [`std::time`], содержащий самые базовые примитивы для измерения времени. Для работы с датами, часовыми поясами, эпохами и другими подобными вещами в экосистеме [Rust] используются crates [`time`] и [`chrono`].


Основное различие между ними (за исключением API, эргономики и активности поддержки) заключается в том, что crate [`chrono`] параметризует часовой пояс в типах, в то время как crate [`time`] обрабатывает его во время выполнения. На практике мы рекомендуем использовать crate [`time`] (если только [`chrono`] лучше подходит для ваших нужд), поскольку он гораздо активнее поддерживается и развивается.

Если вы сталкиваетесь с ограничениями библиотек [`time`] и [`chrono`] в отношении их точности (например, пропуск [високосных секунд][3]) или поддерживаемых форматов/стандартов (например, [TAI]), рассмотрите возможность использования библиотеки [`hifitime`], представляющей собой научно точную и [формально проверенную][4] библиотеку даты и времени.


Для лучшего понимания и ознакомления с этой темой, прочтите следующие материалы:
- [Official `std::time` docs][`std::time`]
- [Official `time` crate docs][`time`]
- [Official `chrono` crate docs][`chrono`]
- [Official `hifitime` crate docs][`hifitime`]




## Измерение длительности выполнения кода

Обратите внимание, что для измерения продолжительности какой-либо операции следует использовать не примитивы крейта [`time`] или [`std::time::SystemTime`], а только [`std::time::Instant`], поскольку они обеспечивают [монотонное измерение времени][1] (в противном случае ваше измерение времени может быть непоследовательным из-за [дрейфа системных часов][2]).


## Task

Provide implementations for `User::age()` and `User::is_adult()` methods in [this step's crate](src/main.rs).

Prove your implementation correctness with additional tests. For tests reproducibility consider that "now time" is the date specified in the `NOW` constant.




# Questions


После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- How does system clock and monotonic clock differ? What are use-cases for both?

- Why is system clock is not reliable for measuring duration? What causes its drift?
- What is the main practical difference between [`chrono`] and [`time`] crates?
- When [`hifitime`] crate could be useful?

<hr>


<hr>

[`chrono`]: https://docs.rs/chrono
[`hifitime`]: https://docs.rs/hifitime
[`std::time`]: https://doc.rust-lang.org/std/time/index.html
[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`time`]: https://docs.rs/time
[Rust]: https://www.rust-lang.org
[TAI]: https://en.wikipedia.org/wiki/International_Atomic_Time

[1]: https://stackoverflow.com/questions/3523442/difference-between-clock-realtime-and-clock-monotonic
[2]: https://en.wikipedia.org/wiki/Clock_drift
[3]: https://en.wikipedia.org/wiki/Leap_second
[4]: https://model-checking.github.io/kani-verifier-blog/2023/03/31/how-kani-helped-find-bugs-in-hifitime.html
