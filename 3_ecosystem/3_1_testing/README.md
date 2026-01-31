Шаг 3.1: Тестирование и имитация
=============================

__Estimated time__: 1 day

[Rust] testing ecosystem [is not huge, but has grown quite well][1], providing some interesting libraries and solutions.
Экосистема тестирования [Rust] не огромна, [но довольно хорошо развилась][1], предоставляя некоторые интересные библиотеки и решения.



## Встроенные возможности тестирования

[Rust] предоставляет довольно хорошие встроенные возможности тестирования, которые очень хорошо описаны в следующих статьях:
- [Rust Book: 11. Writing Automated Tests][2]
- [Rust By Example: 21. Testing][3]
- [Rust By Example: 12.3. Tests][4]




## BDD style

Стиль тестирования [BDD] [разработка, управляемая поведением: <b>behavior-driven development</b>] подразумевает, _что тестовые примеры представляют собой спецификацию программы_, _а сами тесты доказывают корректность спецификации_.

Хотя в экосистеме [Rust] есть [некоторые крейты для тестирования в стиле BDD][11] (самый зрелый из них — крейт [`cucumber`]), их использование не является обязательным для следования стилю [BDD] (поскольку они могут быть слишком сложными для некоторых тривиальных случаев, таких как [модульное тестирование: unit testing][12]). Ничто не мешает вам следовать стилю [BDD] в обычных тестах [Rust]. Поэтому вместо:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash() {
        let h = hash("some_string");
        
        assert_eq!(h.len(), 64);
        assert!(!h.contains("z"));
    }
}
```
Вы всегда можете написать это более осмысленно:
```rust
#[cfg(test)]
mod hash_spec {
    use super::*;
    
    #[test]
    fn has_64_symbols_len() {
        assert_eq!(hash("some_string").len(), 64);
    }
    
    #[test]
    fn contains_hex_chars_only() {
        assert!(!hash("some_string").contains("z"));
    }
}
```
Это делает тесты более детализированными (и, следовательно, позволяет выявлять более значимые ошибки в тестах), а цели тестирования становятся более понятными для читателей.

## Mocking (имитация)

Экосистема [Rust] имеет [достаточно решений][1] для [имитации][41], некоторые из них достаточно зрелые.

В данный момент наиболее интересным является крейт [`mockiato`], поскольку он достаточно удобен в использовании и поддерживает стабильный [Rust]. Крейт [`unimock`] работает очень похожим образом, но поддерживает супертрейты, так как использует единственный тип `Unimock` для имитации объектов. Крейты [`faux`] и [`mry`] ориентированы на имитацию структур (вместо трейтов).

Кроме того, следует упомянуть крейты [`mockito`] и [`wiremock`], которые весьма полезны для тестирования HTTP-трафика.

Однако самым мощным является ящик [`mockall`]. Подробнее см. [этот обзор][43].

Чтобы лучше понять и ознакомиться с [mocking][41] в [Rust], прочтите:
- [Jorge Ortiz-Fuentes: Rust unit testing: test doubles & stubs][46]
- [Alan Somers: Rust Mock Shootout!][43]
- [Oduah Chigozie: Mocking in Rust: Mockall and alternatives][45]
- [Official `mockall` crate docs][`mockall`]
- [Official `mockiato` crate docs][`mockiato`]
- [Official `unimock` crate docs][`unimock`]
- [Audun Halland: How to write a type-level mock library in Rust][44]




## Тестирование свойств (Property testing)

[Тестирование свойств][21] — это еще одна парадигма тестирования, которую следует рассмотреть. Вкратце, ее можно объяснить следующим образом:

> _Проверка свойств_ — это система тестирования кода путем проверки того, выполняются ли определенные свойства его выходных данных или поведения для всех входных данных. Эти входные данные генерируются автоматически, и, что очень важно, при обнаружении неработающего входного значения, оно автоматически сводится к _минимальному_ тестовому случаю.


Экосистема [Rust] имеет довольно хорошие крейты [`proptest`] и [`quickcheck`], которые предоставляют инструменты и примитивы для [тестирования свойств][21].

Чтобы лучше понять и ознакомиться с [тестированием свойств][21] в [Rust], прочтите:
- [`proptest` crate description][`proptest`]
- [`quickcheck` crate description][`quickcheck`]
- [Proptest Book][22]



## Тестирование на невалидных данных (Fuzzing)

[Fuzzing][31] — это еще один метод тестирования, который включает в себя предоставление недопустимых, неожиданных или случайных данных в качестве входных данных для компьютерной программы. Он [действительно помогает][32] выявлять сбои программы и утечки памяти в крайних случаях.


В экосистеме [Rust] на данный момент есть [несколько инструментов][33] для [Fuzzing][31]. Наиболее известные из них:
- [`cargo-fuzz`] является оболочкой командной строки для использования [`libFuzzer`].
- [afl.rs] позволяет запускать [AFL (американский нечеткий алгоритм - american fuzzy lop)][AFL] на коде, написанном на [Rust].
— [`honggfuzz`] — это ориентированный на безопасность fuzzer с мощными возможностями анализа, поддерживающий эволюционный fuzzing на основе обратной связи и анализа покрытия кода (программного и аппаратного обеспечения).

Чтобы лучше понять и освоить [fuzzing][31] в [Rust], прочтите:
- [Rust Fuzz Book][34]
- [Official `cargo-fuzz` crate docs][`cargo-fuzz`]
- [Official `honggfuzz` crate docs][`honggfuzz`]
- [Adrian Taylor: Comparative fuzzing parallel Rust tools][35]


## More reading

- [Aleksey Kladov: How to Test][61]
- [Joshua Mo: Everything you need to know about testing in Rust][62]


## Task

For the implementation of a small [guessing game][51] in [this step's crate](src/main.rs) provide all possible tests you're able to write.




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [`Что такое стиль TDD? Что такое стиль BDD? В чём заключается основной акцент в стиле BDD?`](#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D1%81%D1%82%D0%B8%D0%BB%D1%8C-tdd-%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D1%81%D1%82%D0%B8%D0%BB%D1%8C-bdd-%D0%B2-%D1%87%D1%91%D0%BC-%D0%B7%D0%B0%D0%BA%D0%BB%D1%8E%D1%87%D0%B0%D0%B5%D1%82%D1%81%D1%8F-%D0%BE%D1%81%D0%BD%D0%BE%D0%B2%D0%BD%D0%BE%D0%B9-%D0%B0%D0%BA%D1%86%D0%B5%D0%BD%D1%82-%D0%B2-%D1%81%D1%82%D0%B8%D0%BB%D0%B5-bdd)
- [`Что такое mocking (имитация)? Когда она полезна?`](#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-mocking-%D0%B8%D0%BC%D0%B8%D1%82%D0%B0%D1%86%D0%B8%D1%8F-%D0%BA%D0%BE%D0%B3%D0%B4%D0%B0-%D0%BE%D0%BD%D0%B0-%D0%BF%D0%BE%D0%BB%D0%B5%D0%B7%D0%BD%D0%B0)

- What is property testing? How does it achieve its goals?
- What is fuzzing? How does it differ from property testing?

<hr>

<h3>Что такое стиль TDD? Что такое стиль BDD? В чём заключается основной акцент в стиле BDD?</h3>

<h4>TDD (`Test-Driven Development`) в Rust</h4>

TDD — это цикл разработки через тестирование: «Красный — Зелёный — Рефакторинг». В Rust этот стиль максимально естественен благодаря встроенному тестовому фреймворку в Cargo.

1. Red: Пишется модульный тест для небольшой функции (обычно в `src/lib.rs` в блоке `cfg(test)`). Код не компилируется или тест падает.
2. `Green`: Пишется минимально необходимый код, чтобы тест прошёл.
3. `Refactor`: Улучшается структура кода, убирается дублирование, при этом компилятор Rust гарантирует безопасность памяти.

Акцент `TDD`: Техническая корректность и чистота реализации каждой отдельной функции или модуля.

<h4>BDD (Behavior-Driven Development)</h4>

`BDD` — это развитие `TDD`, которое переносит фокус с проверки кода на проверку поведения системы с точки зрения пользователя.

В Rust для этого часто используются внешние интеграционные тесты (папка tests/) или специализированные инструменты вроде `Cucumber-rust`. Тесты описываются на человекоподобном языке (часто в формате `Gherkin`): `Given` (Дано), `When` (Когда), `Then` (Тогда).

Акцент `BDD`: Взаимодействие между компонентами и соответствие функционала бизнес-требованиям.

<h4>Основной акцент стиля BDD</h4>

Главный акцент BDD заключается в коммуникации и спецификации.

- `Спецификация поведения`: Вы описываете не то, «как работает алгоритм внутри», а то, «что получает пользователь на выходе».
- `Живая документация`: Тесты становятся документацией, которую может прочитать менеджер или аналитик.
- `Сквозные сценарии`: BDD заставляет проверять цепочки действий (например, «создать пользователя -> активировать -> выполнить вход»), а не изолированные методы.

<h4>Сравнение для Rust-разработчика</h4>

|Характеристика|TDD|BDD|
|--------------|---|---|
|Уровень|Unit-тесты (внутри кода).|Интеграционные/Приемочные тесты.|
|Инструмент|#[test] в модуле.|Gherkin, tests/ директория.|
|Вопрос|Правильно ли написан код?|Тот ли это код, который нужен бизнесу?|
|Типизация|Проверка типов и логики функций.|Проверка пользовательских сценариев.|

<h4>Итог:</h4>

В профессиональной разработке идиоматично использовать TDD для отладки сложной внутренней логики и BDD для гарантии того, что весь механизм (например, веб-сервис или CLI-утилита) работает так, как ожидает заказчик.
Хотите посмотреть пример реализации интеграционного теста в папке tests/ для вашего проекта?

<hr>

<h4>Что такое mocking (имитация)? Когда она полезна?</h4>

Mocking в Rust — это техника тестирования, при которой реальные зависимости (базы данных, внешние API, системные ресурсы) заменяются упрощенными объектами-пустышками («моками»), имитирующими поведение оригиналов.

В Rust Mocking неразрывно связан с использованием трейтов, так как это единственный способ подменить реализацию в статически типизированном коде.

<h4>Когда мокинг полезен?</h4>

1. Изоляция внешних систем: Вам не нужно запускать реальный `PostgreSQL` или подключаться к интернету, чтобы проверить логику обработки данных.
2. Тестирование граничных случаев: Легко заставить мок вернуть специфическую ошибку (например, `ConnectionTimeout`), которую трудно воспроизвести с реальной железкой.
3. Скорость: Тесты с моками выполняются мгновенно в памяти, не требуя настройки окружения и сетевых задержек.
4. Детерминизм: Мок всегда возвращает предсказуемый результат, исключая «мерцающие» (`flaky`) тесты, зависящие от внешних факторов.

<h4>Как это реализуется в Rust?</h4>

<h4>1. Ручной способ (через Трейты)</h4>

Вы определяете интерфейс (трейт) и создаете две реализации: рабочую и тестовую.

```rust
trait Database {
    fn save_user(&self, name: &str);
}

// Реальный объект
struct Postgres;
impl Database for Postgres { 
    fn save_user(&self, name: &str) { /* реальный SQL */ } 
}

// Мок
struct MockDb;
impl Database for MockDb {
    fn save_user(&self, name: &str) { println!("Записан: {}", name); }
}
```

<h4>2. Использование библиотек</h4>

В стандартом для автоматической генерации моков являются:

- Mockall: Самая мощная библиотека, позволяющая создавать моки для трейтов и даже структур одной аннотацией `#[automock]`.
- Moxie: Используется для тестирования сложных асинхронных потоков.

<h4>Основные недостатки и компромиссы</h4>

- Риск «тестирования тестов»: Если вы слишком сильно завязываетесь на моки, вы проверяете не логику программы, а то, что программа вызывает методы мока в определенном порядке.
- Ложное чувство безопасности: Тест может пройти с моком, но упасть в реальности из-за нюансов работы настоящей базы данных (например, конфликта транзакций).
- Сложность архитектуры: Чтобы сделать код тестируемым, вам приходится использовать `Dependency Injection` и оборачивать зависимости в `Arc` или `Box`, что может усложнить дизайн.

<h4>Рекомендация:</h4>

Используйте моки для внешних систем (API, Файловая система), но старайтесь использовать реальные структуры данных и «чистые» функции для тестирования бизнес-логики.



<hr>

[`cargo-fuzz`]: https://docs.rs/cargo-fuzz
[`cucumber`]: https://docs.rs/cucumber
[`faux`]: https://docs.rs/faux
[`honggfuzz`]: https://docs.rs/honggfuzz
[`libFuzzer`]: https://llvm.org/docs/LibFuzzer.html
[`mockall`]: https://docs.rs/mockall
[`mockiato`]: https://docs.rs/mockiato
[`mockito`]: https://docs.rs/mockito
[`mry`]: https://docs.rs/mry
[`proptest`]: https://docs.rs/proptest
[`quickcheck`]: https://docs.rs/quickcheck
[`unimock`]: https://docs.rs/unimock
[`wiremock`]: https://docs.rs/wiremock
[AFL]: http://lcamtuf.coredump.cx/afl
[afl.rs]: https://github.com/rust-fuzz/afl.rs
[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[Rust]: https://www.rust-lang.org

[1]: https://github.com/rust-unofficial/awesome-rust#testing
[2]: https://doc.rust-lang.org/book/ch11-00-testing.html
[3]: https://doc.rust-lang.org/rust-by-example/testing.html
[4]: https://doc.rust-lang.org/rust-by-example/cargo/test.html
[11]: https://crates.io/search?q=bdd
[12]: https://en.wikipedia.org/wiki/Unit_testing
[21]: https://en.wikipedia.org/wiki/Property_testing
[22]: https://altsysrq.github.io/proptest-book/intro.html
[31]: https://en.wikipedia.org/wiki/Fuzzing
[32]: https://github.com/rust-fuzz/trophy-case
[33]: https://crates.io/search?q=fuzzing
[34]: https://rust-fuzz.github.io/book/cargo-fuzz.html
[35]: https://medium.com/@adetaylor/comparative-fuzzing-parallel-rust-tools-fac5ce9c9c2d
[41]: https://en.wikipedia.org/wiki/Mock_object
[43]: https://asomers.github.io/mock_shootout
[44]: https://audunhalland.github.io/blog/how-to-write-a-type-level-mock-library-in-rust
[45]: https://blog.logrocket.com/mocking-rust-mockall-alternatives
[46]: https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_stub
[51]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
[61]: https://matklad.github.io/2021/05/31/how-to-test.html
[62]: https://www.shuttle.rs/blog/2024/03/21/testing-in-rust
