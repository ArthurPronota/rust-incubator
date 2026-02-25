Шаг 3: Общая экосистема
========================

__Estimated time__: 2 days

В этих шагах описаны распространенные библиотеки и инструменты в экосистеме [Rust], необходимые для разработки приложений и библиотек.

> ❗️Перед выполнением этого шага необходимо выполнить все его подшаги.


После выполнения этих заданий вы сможете ответить на следующие вопросы:

 - [Какие возможности тестирования предлагает Rust и когда их следует использовать? Почему следует придерживаться стиля BDD?][23]

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

    Простой Unit Test:

```rust
// 1. Основная функция, которую мы хотим протестировать
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 2. Модуль тестов. Атрибут #[cfg(test)] говорит Rust: 
// "Компилируй это только тогда, когда запущена команда cargo test"
#[cfg(test)]
mod tests {
    // Импортируем всё из внешнего модуля (нашу функцию add)
    use super::*;

    // 3. Конкретный тест
    #[test]
    fn test_add_positive_numbers() {
        // Утверждение: результат должен быть равен 4
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-1, -1), -2);
    }

    // Тест, который ожидает панику (ошибку)
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_division_by_zero() {
        let _ = 1 / 0;
    }
}
```

    Запуск Unit Test:
```bash
cargo test
```

    Б. Интеграционные тесты (Integration Tests)

        Располагаются в отдельной папке tests/ в корне проекта.

  - Когда: Для проверки того, как ваш крейт выглядит «снаружи». Они вызывают только публичный API.
  - Плюс: Гарантируют, что пользователь вашей библиотеки сможет её использовать.

    Структура проекта:
```
my_app/
├── Cargo.toml
├── src/
│   └── lib.rs      <-- Код здесь (должен быть lib, а не только main)
└── tests/
    └── my_test.rs  <-- Интеграционный тест здесь
```

    Код в src/lib.rs  (Библиотека)
```rust
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```
    Код в tests/my_test.rs (Тест):

    Интеграционные тесты работают только с библиотечными крейтами или со всем приложением в целом.

```rust
// Импортируем нашу библиотеку как внешний пользователь
use my_app::add_numbers;

#[test]
fn test_integration_add() {
    // Вызываем публичную функцию
    let result = add_numbers(10, 20);
    
    // Проверяем результат
    assert_eq!(result, 30);
}

#[test]
fn test_with_setup() {
    // Здесь обычно инициализируют окружение, например БД или логи
    let data = 5;
    assert!(add_numbers(data, 5) > 0);
}
```
    Запуск Integration Tests
```bash
cargo test
```

    В. Документационные тесты (Doc-tests)

        Примеры кода прямо в комментариях ///.

  - Когда: Всегда. Это «живая» документация, которая падает, если код устарел.
  - Плюс: Вы убиваете двух зайцев: обучаете пользователя и тестируете API.

    Содержимое src/lib.rs:
```rust
/// Складывает два числа.
///
/// # Examples
///
/// ```
/// use my_app::add; // Замените my_app на имя вашего крейта в Cargo.toml
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
    Запуск Doc-test:
```bash
cargo test --doc
```

    Г. Property-based Testing (через крейт proptest)

        Вместо конкретных значений вы задаете стратегию (например, «любая строка Unicode»).
        
  - Когда: Для поиска пограничных случаев (edge cases), о которых вы даже не догадывались.

    Cargo.toml
```toml
[dev-dependencies]
proptest = "1.6"
```

```rust
use proptest::prelude::*;

fn add(a: i32, b: i32) -> i32 {
    // Представим, что здесь сложная логика
    a.wrapping_add(b)
}

// 1. Используем макрос proptest!
proptest! {
    // 2. Описываем входные данные: "любые i32 для a и b"
    #[test]
    fn test_add_is_commutative(a in any::<i32>(), b in any::<i32>()) {
        // 3. Проверяем свойство: коммутативность (от перестановки слагаемых сумма не меняется)
        prop_assert_eq!(add(a, b), add(b, a));
    }

    #[test]
    fn test_add_with_zero(a in any::<i32>()) {
        // Свойство: прибавление нуля не меняет число
        prop_assert_eq!(add(a, 0), a);
    }
}
```
    Заруск Property-based Test:
```bash
cargo test
```

#### 2. Почему стоит придерживаться стиля BDD?

BDD (Behavior-Driven Development:) — это разработка основанная поведение. В Rust для этого часто используют крейт cucumber или k8s-openapi для сложных систем.

  Преимущества BDD:

  1. Общий язык: Тесты пишутся на языке «Дано — Когда — Тогда» (Given-When-Then), понятном и программисту, и бизнесу.
  2. Фокус на требованиях: Вы тестируете не как реализована функция, а какую задачу она решает. Это делает тесты устойчивыми к рефакторингу.
  3. Живая спецификация: BDD-сценарии служат актуальным описанием того, как работает система.

```gherkin
Feature: Калькулятор

  Scenario: Сложение двух чисел
    Given Я ввел число 10
    And Я ввел число 5
    When Я нажимаю кнопку сложения
    Then Результат должен быть 15

```

#### 3. Сводная таблица применения

|Тип теста|Инструмент|Когда использовать?|
|Unit|#[test]|Логика мелких функций и алгоритмов.|
|Integration|tests/*.rs|Проверка публичного API и взаимодействия модулей.|
|Doc-test|///|Короткие примеры использования для документации.|
|BDD|cucumber|Описание бизнес-процессов (напр. «Оформить заказ»).|

#### Золотое правило Rust-тестирования:

Сначала полагайтесь на систему типов (чтобы ошибку нельзя было выразить в коде), затем на модульные тесты для логики, и в конце на интеграционные BDD-сценарии для проверки работы всей системы в сборе\.


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

[23]: https://github.com/ArthurPronota/rust-incubator/tree/main/3_ecosystem#какие-возможности-тестирования-предлагает-rust-и-когда-их-следует-использовать-почему-следует-придерживаться-стиля-bdd
