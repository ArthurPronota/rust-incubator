Шаг 3: Общая экосистема
========================

__Estimated time__: 2 days

В этих шагах описаны распространенные библиотеки и инструменты в экосистеме [Rust], необходимые для разработки приложений и библиотек.

> ❗️Перед выполнением этого шага необходимо выполнить все его подшаги.


После выполнения этих заданий вы сможете ответить на следующие вопросы:

 - [Какие возможности тестирования предлагает Rust и когда их следует использовать? Почему следует придерживаться стиля BDD?][23]
- [Что такое макросы? Чем они отличаются? Какие преимущества дает их использование? Когда следует писать макрос?][24]
- [Как работать с датой и временем в Rust? Как хранить время? Как передавать его другим приложениям?][25]
- [Как используются регулярные выражения в Rust? Когда их недостаточно? Как написать собственный парсер в Rust?][26]
- How do iterator and collection compare and differ in [Rust]? What is the purpose of immutable collections? Why should I care about using concurrent collections?
- [В чём заключаются различия между итераторами и коллекциями в Rust? Каково назначение неизменяемых коллекций? Почему важно использовать параллельные коллекции?][]

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

    Структуры проекта:
```
my_project/
├── Cargo.toml
├── src/lib.rs
└── tests/
    ├── cucumber.rs          <-- Код тестов
    └── features/
        └── calculator.feature <-- Описание поведения
```

    Описание функции, файл tests/features/calculator.feature:
```gherkin
Feature: Калькулятор

  Scenario: Сложение двух чисел
    Given Я ввел число 10
    And Я ввел число 5
    When Я нажимаю кнопку сложения
    Then Результат должен быть 15
```
Cargo.toml:
```toml
[dev-dependencies]
cucumber = "0.20"
tokio = { version = "1", features = ["full"] } # Cucumber в Rust асинхронен
```

tests/cucumber.rs:
```rust
use cucumber::{given, when, then, World};

// 1. "Мир" (World) — это состояние нашего теста между шагами
#[derive(Debug, Default, World)]
pub struct CalculatorWorld {
    inputs: Vec<i32>,
    result: i32,
}

// 2. Описываем шаги (Steps)
#[given(expr = "Я ввел число {int}")]
fn enter_number(world: &mut CalculatorWorld, num: i32) {
    world.inputs.push(num);
}

#[when("Я нажимаю кнопку сложения")]
fn click_add(world: &mut CalculatorWorld) {
    world.result = world.inputs.iter().sum();
}

#[then(expr = "Результат должен быть {int}")]
fn check_result(world: &mut CalculatorWorld, expected: i32) {
    assert_eq!(world.result, expected);
}

// 3. Запуск рантайма
#[tokio::main]
async fn main() {
    CalculatorWorld::run("tests/features").await;
}
```
  В методологии BDD и библиотеке cucumber-rs ключевые слова __Given__, __When__, __Then__ и __And__ служат для удобства чтения человеком, но для программного движка они работают по принципу сопоставления с шаблоном.

  Примечание:
   - Ключевое слово __And__: В языке Gherkin (файлы .feature) слово __And__ (или И в русской локализации) просто повторяет тип предыдущего шага.

    __Связь между {int} в атрибуте и аргументом num в функции:__

   Связь между {int} в атрибуте и аргументом num в функции является позиционной. В Rust 2026 библиотека cucumber-rs использует процедурные макросы, чтобы автоматически «прошить» передачу данных.

   Вот как выглядит эта связь на схеме:
```mermaid
graph LR
    subgraph "Feature File (Gherkin)"
        A["Я ввел число 10"]
    end

    subgraph "Macro Attribute"
        B["expr = 'Я ввел число {int}'"]
    end

    subgraph "Rust Function"
        C["fn enter_number(world, num: i32)"]
    end

    A -->|сопоставление| B
    B -->|захват значения '10'| D[Data Extractor]
    D -->|парсинг в тип i32| C
```
  Порядок обработки:

  1. Захват (Capture): Библиотека видит в шаблоне плейсхолдер {int}. Когда она встречает строку «Я ввел число 10», она «вырезает» текст 10.
  2. Сопоставление с аргументами:
    1. Первый аргумент функции всегда world (состояние теста). Он игнорируется при поиске данных из строки.
    2. Второй аргумент функции — num: i32. Библиотека видит, что в шаблоне есть один плейсхолдер, а в функции есть один дополнительный аргумент после world.
  3. Автоматическая конвертация: cucumber-rs вызывает метод FromStr для типа i32, превращая строку "10" в число 10.
  4. Вызов: Библиотека выполняет вызов enter_number(current_world, 10).


#### 3. Сводная таблица применения

|Тип теста|Инструмент|Когда использовать?|
|---------|----------|-------------------|
|Unit|#[test]|Логика мелких функций и алгоритмов.|
|Integration|tests/*.rs|Проверка публичного API и взаимодействия модулей.|
|Doc-test|///|Короткие примеры использования для документации.|
|BDD|cucumber|Описание бизнес-процессов (напр. «Оформить заказ»).|

#### Золотое правило Rust-тестирования:

Сначала полагайтесь на систему типов (чтобы ошибку нельзя было выразить в коде), затем на модульные тесты для логики, и в конце на интеграционные BDD-сценарии для проверки работы всей системы в сборе\.

<hr>

### Что такое макросы? Чем они отличаются? Какие преимущества дает их использование? Когда следует писать макрос?

В Rust макросы — это инструменты метапрограммирования, которые позволяют писать код, генерирующий другой код. В отличие от функций, которые работают со значениями, макросы работают с самой структурой программы (токенами и синтаксическим деревом).

#### 1. Какие типы макросов существуют?

В Rust есть две принципиально разные категории макросов:

  А. Декларативные макросы (macro_rules!)

   Самый распространенный тип. Работают по принципу «сопоставления с образцом» (pattern matching).
   - Как работают: Вы описываете шаблон (как в match), и если вызываемый код ему соответствует, он заменяется на тело макроса.
   - Пример: vec![1, 2, 3], println!(), log!().

  Б. Процедурные макросы (Procedural Macros)

  Это функции на Rust, которые принимают код как входные данные и возвращают измененный код.

  - #[derive(...)]: Генерирует код автоматически (например, Deserialize для структур).
  - Атрибуты: #[tokio::main], #[test].
  - Функциональные: Похожи на декларативные, но имеют доступ к полноценному парсингу через крейты syn и quote.

#### 2. Чем макросы отличаются от функций?

|Характеристика|Функция|Макрос|
|--------------|-------|------|
|Выполнение|Во время работы программы (Runtime).|Во время компиляции (Compile-time).|
|Аргументы|Фиксированное количество и типы.|Переменное (Variadic) и любые токены.|
|Синтаксис|Ограничен правилами языка.|Может создавать свои правила (DSL).|
|Область видимости|Не может менять структуру программы.|Может генерировать новые типы, модули.|

#### 3. Какие преимущества они дают?

  1. DRY (Don't Repeat Yourself): Позволяют избежать копипасты там, где функции бессильны (например, реализация одного и того же трейта для 10 разных структур).
  2. Абстракция над синтаксисом: Позволяют создавать предметно-ориентированные языки (DSL - Domain Specific Language). Например, SQL-запросы прямо в коде или описание HTML-шаблонов.
  3. Нулевая стоимость в рантайме: Весь тяжелый труд по генерации кода происходит при компиляции. Процессор выполняет уже готовый, оптимизированный код.
  4. Безопасность: Макросы в Rust «гигиеничны» (hygienic) — они не смешивают свои внутренние переменные с переменными вашего кода, предотвращая случайные баги.

#### 4. Когда СЛЕДУЕТ писать макрос?

Макросы — это мощный, но сложный инструмент. Их стоит писать, если:

- Нужно переменное число аргументов: Как в println! или join!.
- Нужно захватить контекст вызова: Например, автоматически получить имя файла (file!()) или номер строки (line!()) для логов.
- Нужно реализовать трейт для многих типов: Если вы видите, что копируете один и тот же код для 5 структур, напишите derive макрос или macro_rules!.
- Вы создаете DSL: Если стандартный синтаксис Rust слишком громоздок для вашей специфической задачи (например, парсинг сложных протоколов) [2.2].

<hr>

### Как работать с датой и временем в Rust? Как хранить время? Как передавать его другим приложениям?

#### 1. Как хранить и измерять время

Для разных задач используются разные типы данных:

- std::time::Instant: «Монотонное» время. Не зависит от перевода часов.
    - Для чего: Замер длительности операций (профилирование).
- std::time::SystemTime: «Настенное» время ОС. Может «прыгать» назад при синхронизации через NTP.
    - Для чего: Метки времени создания файлов, логов.
- chrono::DateTime: Золотой стандарт индустрии. Поддерживает часовые пояса (UTC, Local) и форматирование.
    - Для чего: Бизнес-логика, базы данных [1.1].

#### 2. Как работать (Крейт Chrono)

Добавьте в Cargo.toml: `chrono = { version = "0.4", features = ["serde"] }`

```rust
use chrono::{Utc, Local, DateTime, TimeZone};

// Текущее время в UTC
let now_utc = Utc::now(); 

// Конвертация в локальное время
let now_local: DateTime<Local> = DateTime::from(now_utc);

// Создание конкретной даты
let dt = Utc.with_ymd_and_hms(2026, 2, 25, 12, 0, 0).unwrap();
```

#### 3. Как передавать другим приложениям

Для совместимости с другими языками и системами используйте два подхода:

  1. ISO 8601 / RFC 3339 (Строки): Понятно человеку и большинству API.
    1. Пример: "2026-02-25T15:45:00Z"
    2. В Rust: now.to_rfc3339().
  2. Unix Timestamp (Числа): Секунды/миллисекунды с 1 января 1970 года. Самый быстрый и компактный способ для БД и бинарных протоколов.
    1. В Rust: now.timestamp() (секунды) или timestamp_millis().

#### 4. Передача через Serde (JSON/TOML)

Если вы используете serde, chrono автоматически сериализует дату в строку RFC 3339.

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Event {
    name: String,
    ts: DateTime<Utc>, // В JSON будет: "2026-02-25T..."
}
```

#### Рекомендация
Если вам нужна экстремальная производительность или вы работаете в no_std (embedded), посмотрите на крейт time. Он строже относится к проверке типов и часто работает быстрее chrono в специфических сценариях.

<hr>

### Как используются регулярные выражения в Rust? Когда их недостаточно? Как написать собственный парсер в Rust?

В Rust работа с текстом делится на три уровня сложности: от простого поиска по шаблону до создания полноценных языков программирования.

#### 1. Регулярные выражения (Regex)

Для регулярных выражений стандартом является крейт regex. Он гарантирует выполнение за линейное время __O(n)__, что защищает от атак типа «отказ в обслуживании» (ReDoS).

Как использовать:

```rust
use regex::Regex;

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let text = "Дата: 2026-02-25";

if let Some(caps) = re.captures(text) {
    println!("Год: {}, Месяц: {}", &caps[1], &caps[2]);
}
```

#### 2. Когда Regex недостаточно?

Регулярные выражения хороши для плоских строк, но они бессильны, если:

- Вложенность: Вы не можете распарсить HTML, XML или JSON с помощью Regex (проблема контекстно-свободных грамматик) [1.1].
- Сложная логика: Если поведение парсера зависит от предыдущих данных (например, «прочитать N байт, а затем интерпретировать их как число»).
- Производительность: Regex требует компиляции в рантайме. Для критических узлов самописный парсер на Zero-copy будет в разы быстрее.
- Читаемость: Огромные «регулярки» превращаются в «write-only» код, который невозможно поддерживать.

#### 3. Как написать собственный парсер?

Вместо того чтобы писать парсер «с нуля» на циклах, в Rust используют парсер-комбинаторы. Это функции-кирпичики, которые вы собираете в сложную логику.

  А. [Winnow](https://docs.rs/winnow/latest/winnow/) (Рекомендуемый выбор)

Это современный наследник nom. Он очень быстрый и позволяет писать парсеры в стиле «изменяемого состояния», что привычно для Rust.

   - Для чего: Бинарные протоколы, сложные форматы данных.
  
  Б. Pest (Для грамматик)

Вы описываете правила в отдельном файле .pest (в стиле [PEG](wikipedia.org, _анализирующая_выражение)).

   - Для чего: Свои языки программирования, сложные конфигурационные файлы.

  В. Logos (Сверхбыстрый лексер)

Если вам нужно просто разбить текст на токены (числа, слова, скобки), logos генерирует на этапе компиляции невероятно быстрые конечные автоматы.

#### Сводная таблица выбора

|Инструмент|Сложность|Скорость|Применение|
|----------|---------|--------|----------|
|str methods|Низкая|Макс.|Поиск подстроки, split, trim.|
|regex|Средняя|Средняя|Валидация почты, дат, логов.|
|winnow|Высокая|Высокая|Парсинг протоколов (Redis, HTTP).|
|pest|Высокая|Средняя|Создание DSL и языков (JSON, SQL).|

__Итог__: Если структура текста сложнее одного предложения или имеет вложенность — переходите на winnow. Это сэкономит недели отладки.

<hr>

### В чём заключаются различия между итераторами и коллекциями в Rust? Каково назначение неизменяемых коллекций? Почему важно использовать параллельные коллекции?


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
[24]: https://github.com/ArthurPronota/rust-incubator/tree/main/3_ecosystem#что-такое-макросы-чем-они-отличаются-какие-преимущества-дает-их-использование-когда-следует-писать-макрос
[25]: https://github.com/ArthurPronota/rust-incubator/tree/main/3_ecosystem#как-работать-с-датой-и-временем-в-rust-как-хранить-время-как-передавать-его-другим-приложениям
[26]: https://github.com/ArthurPronota/rust-incubator/tree/main/3_ecosystem#как-используются-регулярные-выражения-в-rust-когда-их-недостаточно-как-написать-собственный-парсер-в-rust