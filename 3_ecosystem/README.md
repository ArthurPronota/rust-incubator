Шаг 3: Общая экосистема
========================

__Estimated time__: 2 days

В этих шагах описаны распространенные библиотеки и инструменты в экосистеме [Rust], необходимые для разработки приложений и библиотек.

> ❗️Перед выполнением этого шага необходимо выполнить все его подшаги.


После выполнения этих заданий вы сможете ответить на следующие вопросы:

 - [Какие возможности тестирования предлагает Rust и когда их следует использовать? Почему следует придерживаться стиля BDD?](#какие-возможности-тестирования-предлагает-rust-и-когда-их-следует-использовать-почему-следует-придерживаться-стиля-bdd)

- What are macros? How do they differ? What benefits does their usage give? When should I write one?
- How to work with date and time in [Rust]? How should I store time? How should I return it to other applications?
- How are regular expressions used in [Rust]? When are they not enough? How can I write a custom parser in [Rust]?
- How do iterator and collection compare and differ in [Rust]? What is the purpose of immutable collections? Why should I care about using concurrent collections?
- What should I use for serialization in [Rust]? Why this is good or bad?
- How can I generate randomness in [Rust]? Which guarantees of random generator should I choose and when?
- What should I use for password hashing in [Rust]? How can I encrypt a message with [Rust]? How should I compare secret values and why?
- How logging is organized in [Rust] ecosystem? Why should I care about structured logging?
- What should I use for building [CLI] interface in [Rust]? How can I organize a configuration for my application and why?
- Why multithreading is required for [Rust] programs and what problems does it solve? How threads concurrency differs with parallelism? How can I parallelize code in [Rust]?
- What is asynchronicity and what problems does it solve? How is it compared to threads concurrency? What is [Rust] solution for asynchronicity and why it has such design?
- What are actors? When are they useful?




## Task

Write a [CLI] tool for stripping [JPEG] images [metadata][21] and minimizing their size (a simplified analogue of [tinyjpg.com]).

Requirements:
- Accept input list of files and remote [URL]s via: either [CLI] arguments, [STDIN], or read it from a specified file ([EOL]-separated).
- Allow configuring how much images are processed at the same time.
- Allow configuring the output directory to store processed images in.
- Allow configuring the output [JPEG] quality of processed images.
- Read configuration with ascending priority from: a file (format is on your choice), [environment variables][22], [CLI] arguments. All are optional for specifying.
- Support `RUST_LOG` environment variable, allowing granular tuning of log levels per module.
- Print execution time in logs, so it's easy to see how much which operation takes during the execution.

If you have enough time after implementing base requirements, consider to add the following to your solution:
- Allow configuring download speed limit for images from remote [URL]s.
- Cover your implementation with unit and E2E tests.
- Support [PNG] images as well.
- Add comprehensive documentation to your code.


<hr>

### Какие возможности тестирования предлагает Rust и когда их следует использовать? Почему следует придерживаться стиля BDD?

В Rust тестирование — это не надстройка, а часть языка. Система типов и владения исключает целые классы багов, а встроенные инструменты позволяют закрыть пирамиду тестирования без внешних зависимостей.

#### 1. Возможности тестирования в Rust

    А. Модульные тесты (Unit Tests)
        
        Пишутся в том же файле, что и код, в специальном модуле `#[cfg(test)] mod tests`.

  - Когда: Для проверки логики отдельных функций, особенно приватных.
  - Плюс: Доступ к приватным полям и методам.

    Б. Интеграционные тесты (Integration Tests)

        Располагаются в отдельной папке tests/ в корне проекта.

  - Когда: Для проверки того, как ваш крейт выглядит «снаружи». Они вызывают только публичный API.
  - Плюс: Гарантируют, что пользователь вашей библиотеки сможет её использовать.

    В. Документационные тесты (Doc-tests)

        Примеры кода прямо в комментариях ///.

  - Когда: Всегда. Это «живая» документация, которая падает, если код устарел.
  - Плюс: Вы убиваете двух зайцев: обучаете пользователя и тестируете API.

    Г. Property-based Testing (через крейт proptest)

        Вместо конкретных значений вы задаете стратегию (например, «любая строка Unicode»).
        
  - Когда: Для поиска пограничных случаев (edge cases), о которых вы даже не догадывались.


<hr>

[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[EOL]: https://en.wikipedia.org/wiki/Newline
[JPEG]: https://en.wikipedia.org/wiki/JPEG
[PNG]: https://en.wikipedia.org/wiki/PNG
[Rust]: https://www.rust-lang.org
[STDIN]: https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)
[tinyjpg.com]: https://tinyjpg.com
[URL]: https://en.wikipedia.org/wiki/URL

[21]: https://picvario.com/what-is-image-metadata-role-and-benefits
[22]: https://en.wikipedia.org/wiki/Environment_variable
