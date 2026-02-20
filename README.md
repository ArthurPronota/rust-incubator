Rust Инкубатор
==============

> It wasn’t always so clear, but the Rust programming language is fundamentally about _empowerment_: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.
_<div align="right">Rust Book's Foreword</div>_

This project represents a hard-way step-by-step [Rust] learning course from language basics to a capability of web backend development.




## Prerequisites


### Toolchain

- [rustup] for installing the [Rust] toolchain and keeping it up-to-date.
- [CLion]/[IntelliJ IDEA] + [IntelliJ Rust] + [Toml][IntelliJ Toml] plugins as the development environment (or any other of your choice).


### Bookshelf

- [Rust Book] teaches and explains [Rust] basics.
- [Rust By Example] teaches you [Rust] basics using editable examples.
- [Rust Reference] is not a formal spec, but is more detailed and comprehensive than the [Rust Book].
- [Cheats.rs] and [Rust SVG Cheatsheet] for quick reference.
- [Rust Edition Guide] for considering the improvements in [Rust 2018] and [Rust 2021].
- [Rust std lib] documentation.
- [Cargo Book] is a guide to [Cargo], [Rust]'s build tool and dependency manager.
- [Rustdoc Book] is a guide to the `rustdoc` documentation tool.
- [Rust Cookbook] is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using crates from the [Rust] ecosystem.
- [Rust Design Patterns] is an open source repository of [Rust] design patterns and idioms.
- [Effective Rust] is a collection of guidelines that had been learned from real world experience of creating software in [Rust].
- [Rust API Guidelines] is a set of recommendations on how to design and present APIs for [Rust].
- [Rust FAQ] answers common questions about [Rust].
- [Rust Playground] allows sharing and checking runnable [Rust] code snippets online.
- [Awesome Rust] is a curated list of [Rust] code and resources.
- [This Week in Rust] represents handpicked and subscribable [Rust] weekly updates.
- [Baby Steps] blog by [Nicholas Matsakis](https://github.com/nikomatsakis) shares useful [Rust] patterns, ideas and design decisions.
- [Learning Material for Idiomatic Rust] is a curated list of resources to help you write ergonomic and idiomatic [Rust] code.




## Steps


### Before you start

[Create][1] a new [GitHub repository] for yourself using this one [as a template][11].

> __NOTE__: __This learning course is constantly improving and evolving over time.__ 
>
> To be up-to-date with the recent changes in your own copy of this repository, attach the upstream history with the following commands:
> ```bash
> git remote add upstream https://github.com/instrumentisto/rust-incubator.git
> git fetch upstream main
> git merge upstream/main --allow-unrelated-histories
> ```
> And then, whenever you want to grab some new changes, do the following:
> ```bash
> git fetch upstream main
> git merge upstream/main
> ```
> Additionally, to be aware about new changes, you may either [watch this repository on GitHub][2], or even track it via [RSS subscription].


### Schedule

Each step must be performed as a separate [PR (pull request)][PR] with an appropriate name and check-marked here in the README's schedule after completion. Each step is a [Cargo workspace member][13], so you can run/test it from the project root (i.e. `cargo run -p step_1_8`). __Consider using [rustfmt] and [Clippy] when you're writing [Rust] code.__

- [ ] [Шаг 0: Ознакомьтесь с основами Rust.][Step 0] (3 days)
- [ ] [1. Концепции][Step 1] (2 days, after all sub-steps)
    - [ ] [Шаг 1.1: Значения по умолчанию, клонирование и копирование][Step 1.1] (1 day)
    - [ ] [Шаг 1.2: Boxing и pinning][Step 1.2] (1 day)
    - [ ] [Шаг 1.3: Разделяемое владение и внутренняя изменчивость + Atomic Types in Rust][Step 1.3] (1 day)
    - [ ] [Шаг 1.4: Клонирование при записи (Clone-on-write)][Step 1.4] (1 day)
    - [ ] [Шаг 1.5: Преобразования, приведение типов и разыменование][Step 1.5] (1 day)
    - [ ] [Шаг 1.6: Статическая и динамическая диспетчеризация][Step 1.6] (1 day)
    - [ ] [Шаг 1.7: `Sized` и `?Sized` типы][Step 1.7] (1 day)
    - [ ] [Шаг 1.8: Потоковая безопасность][Step 1.8] (1 day)
    - [ ] [Шаг 1.9: Фантомные типы, Ковариантность, Контравариантность, Инвариантность][Step 1.9] (1 day)
- [ ] [Шаг 2: Идиомы][Step 2] (2 days, after all sub-steps)
    - [ ] [Шаг 2.1: Расширенные типы обеспечивают корректность][Step 2.1] (1 day)
    - [ ] [Шаг 2.2: Обмен значений с помощью `mem::replace`][Step 2.2] (1 day)
    - [ ] [Шаг 2.3: Ограничение поведения, а не данных.][Step 2.3] (1 day)
    - [ ] [Шаг 2.4: Абстрактный входной тип, конкретный выходной тип][Step 2.4] (1 day)
    - [ ] [Шаг 2.5: Исчерпывание][Step 2.5] (1 day)
        - [`Как может быть полезна проверка полноты в коде [Rust] для перечислений и структур? Когда её следует использовать, а когда нет?`](2_idioms/2_5_exhaustivity#как-может-быть-полезна-проверка-полноты-в-коде-rust-для-перечислений-и-структур-когда-её-следует-использовать-а-когда-нет)
        - [`Как работает атрибут #[non_exhaustive] в [Rust]? В каких случаях он используется? Когда его следует использовать, а когда нет?`](2_idioms/2_5_exhaustivity#как-работает-атрибут-non_exhaustive-в-rust-в-каких-случаях-он-используется-когда-его-следует-использовать-а-когда-нет)
    - [ ] [Шаг 2.6: Запечатывание][Step 2.6] (1 day)
        - [`Полное руководство по закрытым свойствам (sealed traits) в Rust`](2_idioms/2_6_sealing#%D0%BF%D0%BE%D0%BB%D0%BD%D0%BE%D0%B5-%D1%80%D1%83%D0%BA%D0%BE%D0%B2%D0%BE%D0%B4%D1%81%D1%82%D0%B2%D0%BE-%D0%BF%D0%BE-%D0%B7%D0%B0%D0%BA%D1%80%D1%8B%D1%82%D1%8B%D0%BC-%D1%81%D0%B2%D0%BE%D0%B9%D1%81%D1%82%D0%B2%D0%B0%D0%BC-sealed-traits-%D0%B2-rust)
        - [`Что означает «запечатывание» в программировании в широком смысле?`](2_idioms/2_6_sealing#%D1%87%D1%82%D0%BE-%D0%BE%D0%B7%D0%BD%D0%B0%D1%87%D0%B0%D0%B5%D1%82-%D0%B7%D0%B0%D0%BF%D0%B5%D1%87%D0%B0%D1%82%D1%8B%D0%B2%D0%B0%D0%BD%D0%B8%D0%B5-%D0%B2-%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D0%BC%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D0%B8-%D0%B2-%D1%88%D0%B8%D1%80%D0%BE%D0%BA%D0%BE%D0%BC-%D1%81%D0%BC%D1%8B%D1%81%D0%BB%D0%B5)
        - [`Что такое «запечатывание трейтов» в [Rust]? Когда это полезно?`](2_idioms/2_6_sealing#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D0%B7%D0%B0%D0%BF%D0%B5%D1%87%D0%B0%D1%82%D1%8B%D0%B2%D0%B0%D0%BD%D0%B8%D0%B5-%D1%82%D1%80%D0%B5%D0%B9%D1%82%D0%BE%D0%B2-%D0%B2-rust-%D0%BA%D0%BE%D0%B3%D0%B4%D0%B0-%D1%8D%D1%82%D0%BE-%D0%BF%D0%BE%D0%BB%D0%B5%D0%B7%D0%BD%D0%BE)
        - [`Какие ограничения имеет механизм изоляции трейтов в [Rust]? Что он мог бы обеспечить, если бы поддерживался компилятором?`](2_idioms/2_6_sealing#%D0%BA%D0%B0%D0%BA%D0%B8%D0%B5-%D0%BE%D0%B3%D1%80%D0%B0%D0%BD%D0%B8%D1%87%D0%B5%D0%BD%D0%B8%D1%8F-%D0%B8%D0%BC%D0%B5%D0%B5%D1%82-%D0%BC%D0%B5%D1%85%D0%B0%D0%BD%D0%B8%D0%B7%D0%BC-%D0%B8%D0%B7%D0%BE%D0%BB%D1%8F%D1%86%D0%B8%D0%B8-%D1%82%D1%80%D0%B5%D0%B9%D1%82%D0%BE%D0%B2-%D0%B2-rust-%D1%87%D1%82%D0%BE-%D0%BE%D0%BD-%D0%BC%D0%BE%D0%B3-%D0%B1%D1%8B-%D0%BE%D0%B1%D0%B5%D1%81%D0%BF%D0%B5%D1%87%D0%B8%D1%82%D1%8C-%D0%B5%D1%81%D0%BB%D0%B8-%D0%B1%D1%8B-%D0%BF%D0%BE%D0%B4%D0%B4%D0%B5%D1%80%D0%B6%D0%B8%D0%B2%D0%B0%D0%BB%D1%81%D1%8F-%D0%BA%D0%BE%D0%BC%D0%BF%D0%B8%D0%BB%D1%8F%D1%82%D0%BE%D1%80%D0%BE%D0%BC)
- [ ] [3. Common ecosystem][Step 3] (2 days, after all sub-steps)
    - [ ] [Шаг 3.1: Тестирование и имитация][Step 3.1] (1 day)
        - [`Что такое стиль TDD? Что такое стиль BDD? В чём заключается основной акцент в стиле BDD?`](3_ecosystem/3_1_testing#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D1%81%D1%82%D0%B8%D0%BB%D1%8C-tdd-%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D1%81%D1%82%D0%B8%D0%BB%D1%8C-bdd-%D0%B2-%D1%87%D1%91%D0%BC-%D0%B7%D0%B0%D0%BA%D0%BB%D1%8E%D1%87%D0%B0%D0%B5%D1%82%D1%81%D1%8F-%D0%BE%D1%81%D0%BD%D0%BE%D0%B2%D0%BD%D0%BE%D0%B9-%D0%B0%D0%BA%D1%86%D0%B5%D0%BD%D1%82-%D0%B2-%D1%81%D1%82%D0%B8%D0%BB%D0%B5-bdd)
        - [`Что такое mocking (имитация)? Когда она полезна?`](3_ecosystem/3_1_testing#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-mocking-%D0%B8%D0%BC%D0%B8%D1%82%D0%B0%D1%86%D0%B8%D1%8F-%D0%BA%D0%BE%D0%B3%D0%B4%D0%B0-%D0%BE%D0%BD%D0%B0-%D0%BF%D0%BE%D0%BB%D0%B5%D0%B7%D0%BD%D0%B0)
        - [`Что такое проверка свойств? Как она достигает своих целей?`](3_ecosystem/3_1_testing#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-%D0%BF%D1%80%D0%BE%D0%B2%D0%B5%D1%80%D0%BA%D0%B0-%D1%81%D0%B2%D0%BE%D0%B9%D1%81%D1%82%D0%B2-%D0%BA%D0%B0%D0%BA-%D0%BE%D0%BD%D0%B0-%D0%B4%D0%BE%D1%81%D1%82%D0%B8%D0%B3%D0%B0%D0%B5%D1%82-%D1%81%D0%B2%D0%BE%D0%B8%D1%85-%D1%86%D0%B5%D0%BB%D0%B5%D0%B9)
        - [`Что такое fuzzing? Чем он отличается от проверки свойств?`](3_ecosystem/3_1_testing#%D1%87%D1%82%D0%BE-%D1%82%D0%B0%D0%BA%D0%BE%D0%B5-fuzzing-%D1%87%D0%B5%D0%BC-%D0%BE%D0%BD-%D0%BE%D1%82%D0%BB%D0%B8%D1%87%D0%B0%D0%B5%D1%82%D1%81%D1%8F-%D0%BE%D1%82-%D0%BF%D1%80%D0%BE%D0%B2%D0%B5%D1%80%D0%BA%D0%B8-%D1%81%D0%B2%D0%BE%D0%B9%D1%81%D1%82%D0%B2)
    - [ ] [Шаг 3.2: Декларативные и процедурные макросы][Step 3.2] (1 day)
        - [`Что такое макросы? Какую проблему они решают?`](3_ecosystem/3_2_macro#что-такое-макросы-какую-проблему-они-решают)
        - [`Какие преимущества имеют декларативные макросы в [Rust] по сравнению с процедурными? Какие у них недостатки и ограничения?`](3_ecosystem/3_2_macro#какие-преимущества-имеют-декларативные-макросы-в-rust-по-сравнению-с-процедурными-какие-у-них-недостатки-и-ограничения)
        - [`Какие виды процедурных макросов существуют в [Rust]?`](3_ecosystem/3_2_macro#какие-виды-процедурных-макросов-существуют-в-rust)
        - [`Какие существуют распространённые крейты для реализации процедурных макросов в [Rust]? Какие обязанности выполняет каждый из них? Какие из них обязательны, а какие нет?`](3_ecosystem/3_2_macro#какие-существуют-распространённые-крейты-для-реализации-процедурных-макросов-в-rust-какие-обязанности-выполняет-каждый-из-них-какие-из-них-обязательны-а-какие-нет)
        - [`Какие существуют распространённые крейты для реализации процедурных макросов в [Rust]? Какие обязанности выполняет каждый из них? Какие из них обязательны, а какие нет?`](3_ecosystem/3_2_macro#какие-существуют-распространённые-крейты-для-реализации-процедурных-макросов-в-rust-какие-обязанности-выполняет-каждый-из-них-какие-из-них-обязательны-а-какие-нет)
        - [`Каковы лучшие практики реализации процедурных макросов в [Rust]?`](3_ecosystem/3_2_macro#каковы-лучшие-практики-реализации-процедурных-макросов-в-rust)
    - [ ] [Шаг 3.3: Дата и время][Step 3.3] (1 day)
        - [`Чем отличаются системные часы от монотонных? Для чего используются оба типа часов?`](3_ecosystem/3_3_date_time#чем-отличаются-системные-часы-от-монотонных-для-чего-используются-оба-типа-часов)
        - [`Почему системные часы ненадежны для измерения длительности? Что вызывает их дрейф?`](3_ecosystem/3_3_date_time#почему-системные-часы-ненадежны-для-измерения-длительности-что-вызывает-их-дрейф)
        - [`В чём заключается основное практическое различие между crates [`chrono`] и [`time`]?`](3_ecosystem/3_3_date_time#в-чём-заключается-основное-практическое-различие-между-crates-chrono-и-time)
        - [`Когда может быть полезен фреймворк [hifitime]?`](3_ecosystem/3_3_date_time#когда-может-быть-полезен-фреймворк-hifitime)
    - [ ] [Шаг 3.4: Регулярные выражения и пользовательские парсеры][Step 3.4] (1 day)
        - [`Как библиотека [regex] достигает линейной временной сложности? Какой ценой?`](3_ecosystem/3_4_regex_parsing#как-библиотека-regex-достигает-линейной-временной-сложности-какой-ценой)
        - [`Как избежать перекомпиляции регулярных выражений в [Rust]? Почему это важно?`](3_ecosystem/3_4_regex_parsing#как-избежать-перекомпиляции-регулярных-выражений-в-rust-почему-это-важно)
        - [`Какие библиотеки наиболее распространены для написания пользовательских парсеров на Rust? Какие преимущества у каждой из них?`](3_ecosystem/3_4_regex_parsing#какие-библиотеки-наиболее-распространены-для-написания-пользовательских-парсеров-на-rust-какие-преимущества-у-каждой-из-них)
        - [`Какие преимущества дают библиотеки при написании собственного парсера? Являются ли они обязательными? Когда имеет смысл избегать использования библиотек для реализации парсера?`](3_ecosystem/3_4_regex_parsing#какие-преимущества-дают-библиотеки-при-написании-собственного-парсера-являются-ли-они-обязательными-когда-имеет-смысл-избегать-использования-библиотек-для-реализации-парсера)
        - [`Базовые конструкции regex`](3_ecosystem/3_4_regex_parsing#базовые-конструкции-regex)
        - [`Парзер winnow, делающиё разбор лёгким (руководство)`](3_ecosystem/3_4_regex_parsing#парзер-winnow-делающиё-разбор-лёгким-руководство)
    - [ ] [Шаг 3.5: Коллекции и итераторы][Step 3.5] (1 day)
        - [Что такое коллекция? Что такое итератор? Чем они отличаются? Как они используются? Какие ограничения есть у каждого из них?](3_ecosystem/3_5_collections#что-такое-коллекция-что-такое-итератор-чем-они-отличаются-как-они-используются-какие-ограничения-есть-у-каждого-из-них)
        - [Что такое неизменяемые коллекции? Как они работают? Почему не стоит использовать их постоянно? Когда имеет смысл их использовать?](3_ecosystem/3_5_collections#что-такое-неизменяемые-коллекции-как-они-работают-почему-не-стоит-использовать-их-постоянно-когда-имеет-смысл-их-использовать)
        - [Что такое коллекции одновременного доступа (concurrent)? Как они работают? Почему они лучше, чем явная синхронизация для обычной коллекции?](3_ecosystem/3_5_collections#что-такое-коллекции-одновременного-доступа-concurrent-как-они-работают-почему-они-лучше-чем-явная-синхронизация-для-обычной-коллекции)
    - [ ] [Шаг 3.6: Сериализация и десериализация][Step 3.6] (1 day)
        - [Как `serde` достигает своей производительности? Как он моделирует данные и разделяет обязанности?](3_ecosystem/3_6_serde#как-serde-достигает-своей-производительности-как-он-моделирует-данные-и-разделяет-обязанности)
        - [Когда имеет смысл предпочесть `musli` вместо `serde`?](3_ecosystem/3_6_serde#когда-имеет-смысл-предпочесть-musli-вместо-serde)
        - [Что такое десериализация с нулевым копированием? Почему она полезна? Как она работает в `serde`? Как она работает в `rkyv`?](3_ecosystem/3_6_serde#что-такое-десериализация-с-нулевым-копированием-почему-она-полезна-как-она-работает-в-serde-как-она-работает-в-rkyv)
        - [Inlining](3_ecosystem/3_6_serde#inlining)
    - [ ] [Шаг 3.7: Случайность и криптография][Step 3.7] (1 day)
        - [В чём заключается главный компромисс при генерации случайных чисел? Как это применяется на практике?](3_ecosystem/3_7_rand_crypto#в-чём-заключается-главный-компромисс-при-генерации-случайных-чисел-как-это-применяется-на-практике)
        - [Что такое симметричная криптография? Что такое асимметричная криптография? Какие преимущества имеет каждая из них?](3_ecosystem/3_7_rand_crypto#что-такое-симметричная-криптография-что-такое-асимметричная-криптография-какие-преимущества-имеет-каждая-из-них)
        - [Что такое подпись в асимметричной криптографии? Что такое шифрование в асимметричной криптографии? Как они работают при наличии одних и тех же закрытого и открытого ключей?](3_ecosystem/3_7_rand_crypto#что-такое-подпись-в-асимметричной-криптографии-что-такое-шифрование-в-асимметричной-криптографии-как-они-работают-при-наличии-одних-и-тех-же-закрытого-и-открытого-ключей)
        - [Что такое хеш-функция? Что такое хеширование паролей? Почему для хеширования паролей недостаточно использовать только обычную хеш-функцию?](3_ecosystem/3_7_rand_crypto#что-такое-хеш-функция-что-такое-хеширование-паролей-почему-для-хеширования-паролей-недостаточно-использовать-только-обычную-хеш-функцию)
        - [Что такое сравнение за постоянное время? Когда и почему его следует использовать?](3_ecosystem/3_7_rand_crypto#что-такое-сравнение-за-постоянное-время-когда-и-почему-его-следует-использовать)
        - [Какие существуют варианты использования TLS в Rust? Каковы преимущества и недостатки каждого из них?](3_ecosystem/3_7_rand_crypto#какие-существуют-варианты-использования-tls-в-rust-каковы-преимущества-и-недостатки-каждого-из-них)
    - [ ] [Шаг 3.8: Логирование и трассировка][Step 3.8] (1 day)
        - [Как библиотека `log` обеспечивает возможность повторного использования в экосистеме? Каковы лежащие в её основе идеи?](3_ecosystem/3_8_log/README.md#как-библиотека-log-обеспечивает-возможность-повторного-использования-в-экосистеме-каковы-лежащие-в-её-основе-идеи)
        - [Почему логирование предпочтительнее вывода текста на экран (использование `println!`)? А когда это не так?](3_ecosystem/3_8_log/README.md#почему-логирование-предпочтительнее-вывода-текста-на-экран-использование-println-а-когда-это-не-так)
        - [Что такое структурированное логирование? Какие преимущества оно предоставляет?](3_ecosystem/3_8_log/README.md#что-такое-структурированное-логирование-какие-преимущества-оно-предоставляет)
        - [Почему крейт `tracing` хорош для логирования? Чем он предпочтительнее крейтов `slog` и `log`?](3_ecosystem/3_8_log/README.md#почему-крейт-tracing-хорош-для-логирования-чем-он-предпочтительнее-крейтов-slog-и-log)
        - [Что такое трассировка? Почему она полезна для обеспечения наблюдаемости?](3_ecosystem/3_8_log#что-такое-трассировка-почему-она-полезна-для-обеспечения-наблюдаемости)
    - [ ] [Шаг 3.9: Аргументы командной строки, переменные среды и конфигурации][Step 3.9] (1 day)
        - [В чём преимущества строгой типизации конфигурации?](3_ecosystem/3_9_cmd_env_conf#в-чём-преимущества-строгой-типизации-конфигурации)
        - [Почему переменные окружения полезны для настройки приложения? В чём их основное назначение?](3_ecosystem/3_9_cmd_env_conf#почему-переменные-окружения-полезны-для-настройки-приложения-в-чём-их-основное-назначение)
        - [В чём польза crate `config`? Зачем его использовать, если нельзя просто десериализовать файл в структуры с помощью `serde`?](3_ecosystem/3_9_cmd_env_conf#в-чём-польза-crate-config-зачем-его-использовать-если-нельзя-просто-десериализовать-файл-в-структуры-с-помощью-serde)
    - [ ] [Шаг 3.10: Многопоточность и параллелизм][Step 3.10] (1 day)
        - [Что такое параллелизм? Что такое конкуренция? Как они связаны друг с другом и чем отличаются?](3_ecosystem/3_10_threads#что-такое-параллелизм-что-такое-конкуренция-как-они-связаны-друг-с-другом-и-чем-отличаются)
        - [Как в Rust реализован параллелизм? Какие крейты обычно используются для его применения?](3_ecosystem/3_10_threads#как-в-rust-реализован-параллелизм-какие-крейты-обычно-используются-для-его-применения)
        - [Какие основные способы синхронизации потоков существуют в Rust? Каковы преимущества и недостатки каждого из них? Для чего используется каждый из них?](3_ecosystem/3_10_threads#какие-основные-способы-синхронизации-потоков-существуют-в-rust-каковы-преимущества-и-недостатки-каждого-из-них-для-чего-используется-каждый-из-них)
    - [ ] [Шаг 3.11: Асинхронный ввод-вывод, фьючерсы (futures) и акторы (actors)][Step 3.11] (2 days)
    
- [ ] [4. Backend ecosystem][Step 4] (3 days, after all sub-steps)
    - [ ] [4.1. Databases, connection pools and ORMs][Step 4.1] (1 day)
    - [ ] [4.2. HTTP servers and clients][Step 4.2] (1 day)
    - [ ] [4.3. API servers, clients and tools][Step 4.3] (1 day)




## More practice

- [Rustlings][rustlings] is a collection of small exercises to get you used to reading and writing [Rust] code.
- [Rust on Exercism] provides coding exercises with mentoring.
- [Rust Quiz] for medium to hard [Rust] questions with explanations.




[Step 0]: 0_basics
[Step 1]: 1_concepts
[Step 1.1]: 1_concepts/1_1_default_clone_copy
[Step 1.2]: 1_concepts/1_2_box_pin
[Step 1.3]: 1_concepts/1_3_rc_cell
[Step 1.4]: 1_concepts/1_4_cow
[Step 1.5]: 1_concepts/1_5_convert_cast_deref
[Step 1.6]: 1_concepts/1_6_dispatch
[Step 1.7]: 1_concepts/1_7_sized
[Step 1.8]: 1_concepts/1_8_thread_safety
[Step 1.9]: 1_concepts/1_9_phantom
[Step 2]: 2_idioms
[Step 2.1]: 2_idioms/2_1_type_safety
[Step 2.2]: 2_idioms/2_2_mem_replace
[Step 2.3]: 2_idioms/2_3_bound_impl
[Step 2.4]: 2_idioms/2_4_generic_in_type_out
[Step 2.5]: 2_idioms/2_5_exhaustivity
[Step 2.6]: 2_idioms/2_6_sealing
[Step 3]: 3_ecosystem
[Step 3.1]: 3_ecosystem/3_1_testing
[Step 3.2]: 3_ecosystem/3_2_macro
[Step 3.3]: 3_ecosystem/3_3_date_time
[Step 3.4]: 3_ecosystem/3_4_regex_parsing
[Step 3.5]: 3_ecosystem/3_5_collections
[Step 3.6]: 3_ecosystem/3_6_serde
[Step 3.7]: 3_ecosystem/3_7_rand_crypto
[Step 3.8]: 3_ecosystem/3_8_log
[Step 3.9]: 3_ecosystem/3_9_cmd_env_conf
[Step 3.10]: 3_ecosystem/3_10_threads
[Step 3.11]: 3_ecosystem/3_11_async
[Step 4]: 4_backend
[Step 4.1]: 4_backend/4_1_db
[Step 4.2]: 4_backend/4_2_http
[Step 4.3]: 4_backend/4_3_api

[Awesome Rust]: https://github.com/rust-unofficial/awesome-rust
[Baby Steps]: http://smallcultfollowing.com/babysteps
[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Cheats.rs]: https://cheats.rs
[CLion]: https://www.jetbrains.com/clion
[Clippy]: https://github.com/rust-lang/rust-clippy
[Effective Rust]: https://www.lurklurk.org/effective-rust
[GitHub repository]: https://help.github.com/articles/github-glossary/#repository
[IntelliJ IDEA]: https://www.jetbrains.com/idea
[IntelliJ Rust]: https://intellij-rust.github.io
[IntelliJ Toml]: https://plugins.jetbrains.com/plugin/8195-toml
[Learning Material for Idiomatic Rust]: https://corrode.dev/blog/idiomatic-rust-resources
[PR]: https://help.github.com/articles/github-glossary/#pull-request
[RSS subscription]: https://github.com/instrumentisto/rust-incubator/commits/main.atom
[Rust]: https://www.rust-lang.org
[Rust 2018]: https://doc.rust-lang.org/edition-guide/rust-2018/index.html
[Rust 2021]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust Design Patterns]: https://rust-unofficial.github.io/patterns
[Rust Edition Guide]: https://doc.rust-lang.org/edition-guide
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[Rust on Exercism]: https://exercism.org/tracks/rust/exercises
[Rust Playground]: https://play.rust-lang.org
[Rust Quiz]: https://github.com/dtolnay/rust-quiz
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust std lib]: https://doc.rust-lang.org/std
[Rust SVG Cheatsheet]: https://web.archive.org/web/20241001012119/https://www.breakdown-notes.com/make/load/rust_cs_canvas/true
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[rustfmt]: https://github.com/rust-lang/rustfmt
[rustlings]: https://rustlings.cool
[rustup]: https://rustup.rs
[This Week in Rust]: https://this-week-in-rust.org

[1]: https://github.com/instrumentisto/rust-incubator/generate
[2]: https://github.com/instrumentisto/rust-incubator/subscription
[11]: https://help.github.com/en/articles/creating-a-repository-from-a-template
[13]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
