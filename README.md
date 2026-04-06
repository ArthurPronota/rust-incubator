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
    - [Как распознать, что данные выделяются в куче, а не в стеке? Когда данные следует выделять в куче?][010001]
    - [Что такое копирование и клонирование данных в Rust? В чём разница? Когда и зачем их использовать?][010002]
    - [Как один и тот же фрагмент данных может принадлежать нескольким частям программы? Когда и почему это обычно необходимо?][010003]
    - [Каким образом могут быть нарушены правила заимствования? Какой ценой? Когда и почему это обычно требуется?][010004]
    - [Как одновременно работать с собственными и заимствованными данными? Когда и почему это обычно необходимо?][010005]
    - [Как обмениваться значениями между потоками? Что такое маркеры `Send` и `Sync`? Зачем они нужны и когда их следует использовать?][010006]
    - [Чем отличаются статические и динамические диспетчеризации? Зачем они нужны? Когда и почему следует выбирать между ними?][010007]
    - [Зачем существуют типы данных ?Sized ? Как они используются? Почему они важны для меня?][010008]
    - [Почему существуют фантомные типы? Какие проблемы они решают?][010009]
    - [Операции с AtomicPtr, AtomicUsize, ptr::read, std::mem::forget][010010]
    - [ ] [Шаг 1.1: Значения по умолчанию, клонирование и копирование][Step 1.1] (1 day)
        - [Какова функция трейта `Default` в Rust?][010101]
        - [На что способна директива #[derive(Default)] из стандартной библиотеки? В чём её ошибка? Какие есть альтернативы?][010102]
        - [Что означает слово `Clone` в семантическом смысле?][010103]
        - [Что означает Copy в семантическом смысле? Как оно связано со словом Clone? Какие у него ограничения и почему?][010104]    
    - [ ] [Шаг 1.2: Boxing и pinning][Step 1.2] (1 day)
        - [Что означает "boxing" в Rust? В чём её польза? Когда и зачем она необходима?][010201]
        - [Что такое Pin и зачем он нужен? Какие гарантии он предоставляет? Как он их обеспечивает?][010202]
        - [Как Unpin влияет на Pin? Что это означает?][010203]
        - [Разрешено ли перемещать закрепленные данные после того, как Pin перестает работать? Почему?][010204]
        - [Что такое структурное закрепление? Когда его следует использовать и почему?][010205]
        - [Что такое проекция Pin? Зачем она нужна? Как она используется?][010206]
    - [ ] [Шаг 1.3: Разделяемое владение и внутренняя изменчивость + Atomic Types in Rust][Step 1.3] (1 day)
        - [Что такое разделяемое? Какую проблему она решает? Какие у неё недостатки?][010301]
        - [Что такое внутренняя изменяемость? Зачем она нужна в Rust? Какова её цена?][010302]
        - [Можно ли написать собственный тип с возможностью внутренней изменчивости без использования std? Почему?][010303]
        - [Что такое разделяемая изменяемость? В каких случаях она наиболее распространена?][010304]
        - [Как обеспечить пользователям доступ к API без возникновения паники/взаимной блокировки при использовании внутренней изменяемости?][010305]
        - [Atomic Types in Rust][010306]
    - [ ] [Шаг 1.4: Клонирование при записи (Clone-on-write)][Step 1.4] (1 day)
        - [Что такое Cow? Как это работает?][010401]
        - [Когда Cow полезно и почему? Приведите несколько показательных примеров.][010402]    
    - [ ] [Шаг 1.5: Преобразования, приведение типов и разыменование][Step 1.5] (1 day)
        - [Как в Rust представлено преобразование значений? Какова связь между ошибочным и безошибочным преобразованием?][010501]
        - [Как в Rust представлено преобразование ссылок? Чем отличаются его трейты? Когда и какой из них следует использовать?][010502]
        - [Как в Rust можно осуществить преобразование внутренних ссылок во внешние? Какие для этого необходимы условия?][010503]
        - [Что такое разыменование в Rust? Как его можно использовать не по назначению? Почему его не следует использовать не по назначению?][010504]
        - [Почему использование ключевого слова `as` не является хорошей практикой в ​​Rust? Почему мы всё ещё его используем?][010505]
    - [ ] [Шаг 1.6: Статическая и динамическая диспетчеризация][Step 1.6] (1 day)
        - [Что такое диспетчеризация? Когда вызов функции представляет собой диспетчеризацию, а когда нет?][010601]
        - [Как работает статическая диспетчеризация?][010602]
        - [Как работает динамическая диспетчеризация? Зачем она нужна? Какие ограничения она имеет в Rust? Почему она существует?][010603]
        - [Когда динамическую диспетчеризацию можно заменить статической? Когда нет? Каковы компромиссы?][010604]
        - [Как уменьшить размер кода, сгенерированного компилятором, при использовании статической диспетчеризации?][010605]
    - [ ] [Шаг 1.7: `Sized` и `?Sized` типы][Step 1.7] (1 day)
        - [Что означает свойство Sized? Когда Rust его подразумевает? А когда нет?][010701]
        - [Почему важна привязка признака `?Sized`? Когда и почему её следует использовать?][010702]    
    - [ ] [Шаг 1.8: Потоковая безопасность][Step 1.8] (1 day)
        - [Что означает "безбоязненная конкурентность" в Rust? С помощью каких механизмов Rust точно выполняет эту гарантию?][010801]
        - [Зачем вообще существуют Send и Sync? Как это связано с внутренней изменчивостью?][010802]
    - [ ] [Шаг 1.9: Фантомные типы, Ковариантность, Контравариантность, Инвариантность][Step 1.9] (1 day)
        - [Ковариантность, Контравариантность, Инвариантность в Rust][010901]
        - [Зачем в Rust существует PhantomData? Какие проблемы он решает?][010902]
        - [Как на практике работает прозрачность PhantomData?][010903]
        - [Какие существуют альтернативы PhantomData? Когда их целесообразно использовать?][010904]
- [ ] [Шаг 2: Идиомы][Step 2] (2 days, after all sub-steps)
    - [Почему меня должны волновать типы и способ выражения информации с помощью типов? Как типы помогают повысить гарантии корректности программы?][020001]
    - [Что необходимо для написания хорошо спроектированных и эргономичных API на Rust и почему?][020002]
    - [Зачем существует `mem::replace` и какую цель он преследует? Когда и почему он действительно полезен?][020003]
    - [Как обычно организуется полиморфизм типов входных данных в API на Rust? Какова его стоимость?][020004]
    - [Какие существуют способы и инструменты для обеспечения перспективности исходного кода на Rust?][020005]
    - [ ] [Шаг 2.1: Расширенные типы обеспечивают корректность][Step 2.1] (1 day)
        - [Почему выражение семантики в типах — это хорошо? Каковы преимущества и недостатки?][020101]
        - [Что такое шаблон NewType? Как он работает? Какие гарантии он предоставляет?][020102]
        - [Что такое шаблон проектирования "типов состояний"? Как он работает? Какие гарантии он предоставляет?][020103]
    - [ ] [Шаг 2.2: Обмен значений с помощью `mem::replace`][Step 2.2] (1 day)
        - [В чём причина существования `mem::replace` в Rust? Что он нам даёт? Почему мы не можем решить те же проблемы без него?][020201]
        - [Приведите несколько наглядных примеров использования `mem::replace` в Rust.][020202]
    - [ ] [Шаг 2.3: Ограничение поведения, а не данных.][Step 2.3] (1 day)
        - [Какие проблемы создают ограничения трейтов в Rust при их размещении на определении типа?][020301]
        - [Почему размещение ограничений на трейты в блоках impl предпочтительнее?][020302]
        - [Когда это невозможно, и следует использовать ограничения трейтов в определении типа? Когда это предпочтительнее?][020303]
        - [В чём заключаются проблемы с макросами `std` derive в отношении параметров типов? Как их можно решить?][020304]
    - [ ] [Шаг 2.4: Абстрактный входной тип, конкретный выходной тип][Step 2.4] (1 day)
       - [Почему абстрагирование от типов входных данных — это хорошо? Какие проблемы оно создает и как их можно преодолеть?][020401]
       - [Когда возврат конкретного типа оправдан? А когда нет? На какие компромиссы следует пойти?][020402]
    - [ ] [Шаг 2.5: Исчерпывание][Step 2.5] (1 day)
        - [Как может быть полезна проверка полноты в коде Rust для перечислений и структур? Когда её следует использовать, а когда нет?][020501]
        - [Как работает атрибут #[non_exhaustive] в Rust? В каких случаях он используется? Когда его следует использовать, а когда нет?][020502]
    - [ ] [Шаг 2.6: Запечатывание][Step 2.6] (1 day)
        - [Полное руководство по закрытым свойствам (sealed traits) в Rust][020601]
        - [Что означает «запечатывание» в программировании в широком смысле?][020602]
        - [Что такое «запечатывание трейтов» в Rust? Когда это полезно?][020603]
        - [Какие ограничения имеет механизм изоляции трейтов в Rust? Что он мог бы обеспечить, если бы поддерживался компилятором?][020604]
- [ ] [Шаг 3: Общая экосистема][Step 3] (2 days, after all sub-steps)
    - [Какие возможности тестирования предлагает Rust и когда их следует использовать? Почему следует придерживаться стиля BDD?][14]
    - [Что такое макросы? Чем они отличаются? Какие преимущества дает их использование? Когда следует писать макрос?][15]
    - [Как работать с датой и временем в Rust? Как хранить время? Как передавать его другим приложениям?][16]
    - [Как используются регулярные выражения в Rust? Когда их недостаточно? Как написать собственный парсер в Rust?][17]
    - [В чём заключаются различия между итераторами и коллекциями в Rust? Каково назначение неизменяемых коллекций? Почему важно использовать параллельные коллекции?][18]
    - [Что следует использовать для сериализации в Rust? Почему это хорошо или плохо?][19]
    - [Как генерировать случайные числа в Rust? Какие гарантии генератора случайных чисел следует выбирать и когда?][20]
    - [Что следует использовать для хеширования паролей в Rust? Как зашифровать сообщение с помощью Rust? Как следует сравнивать секретные значения и почему?][21]
    - [Как организовано логирование в экосистеме Rust? Почему мне следует интересоваться структурированным логированием?][22]
    - [Что следует использовать для создания интерфейса командной строки (CLI) на Rust? Как организовать конфигурацию для моего приложения и зачем?][23]
    - [Почему многопоточность необходима для программ на Rust и какие проблемы она решает? Чем отличается многопоточность от параллельной обработки? Как можно распараллелить код на Rust?][24]
    - [Что такое асинхронность и какие проблемы она решает? Чем она отличается от параллельного выполнения потоков? Какое решение для асинхронности предлагает Rust и почему он имеет именно такую ​​архитектуру?][25]
    - [Что такое акторы? Когда они полезны?][26]
        - [ ] [Шаг 3.1: Тестирование и имитация][Step 3.1] (1 day)
            - [Что такое стиль TDD? Что такое стиль BDD? В чём заключается основной акцент в стиле BDD?][030101]
            - [Что такое mocking (имитация)? Когда она полезна?][030102]
            - [Что такое проверка свойств? Как она достигает своих целей?][030103]
            - [Что такое fuzzing? Чем он отличается от проверки свойств?][030104]
        - [ ] [Шаг 3.2: Декларативные и процедурные макросы][Step 3.2] (1 day)
            - [Что такое макросы? Какую проблему они решают?][030201]
            - [Какие преимущества имеют декларативные макросы в Rust по сравнению с процедурными? Какие у них недостатки и ограничения?][030202]
            - [Какие виды процедурных макросов существуют в Rust?][030203]
            - [Какие существуют распространённые крейты для реализации процедурных макросов в Rust? Какие обязанности выполняет каждый из них? Какие из них обязательны, а какие нет?][030204]
            - [Каковы лучшие практики реализации процедурных макросов в Rust?][030205]
        - [ ] [Шаг 3.3: Дата и время][Step 3.3] (1 day)
            - [Чем отличаются системные часы от монотонных? Для чего используются оба типа часов?][030301]
            - [Почему системные часы ненадежны для измерения длительности? Что вызывает их дрейф?][030302]
            - [В чём заключается основное практическое различие между crates `chrono` и `time`?][030303]
            - [Когда может быть полезен фреймворк `hifitime`?][030304]
        - [ ] [Шаг 3.4: Регулярные выражения и пользовательские парсеры][Step 3.4] (1 day)
            - [Как библиотека `regex` достигает линейной временной сложности? Какой ценой?][030401]
            - [Как избежать перекомпиляции регулярных выражений в Rust? Почему это важно?][030402]
            - [Какие библиотеки наиболее распространены для написания пользовательских парсеров на Rust? Какие преимущества у каждой из них?][030403]
            - [Какие преимущества дают библиотеки при написании собственного парсера? Являются ли они обязательными? Когда имеет смысл избегать использования библиотек для реализации парсера?][030404]
            - [Базовые конструкции regex][030405]
            - [Парзер winnow, делающиё разбор лёгким (руководство)][030406]
        - [ ] [Шаг 3.5: Коллекции и итераторы][Step 3.5] (1 day)
            - [Что такое коллекция? Что такое итератор? Чем они отличаются? Как они используются? Какие ограничения есть у каждого из них?][030501]
            - [Что такое неизменяемые коллекции? Как они работают? Почему не стоит использовать их постоянно? Когда имеет смысл их использовать?][030502]
            - [Что такое коллекции одновременного доступа (concurrent)? Как они работают? Почему они лучше, чем явная синхронизация для обычной коллекции?][030503]
        - [ ] [Шаг 3.6: Сериализация и десериализация][Step 3.6] (1 day)
            - [Как `serde` достигает своей производительности? Как он моделирует данные и разделяет обязанности?][030601]
            - [Когда имеет смысл предпочесть `musli` вместо `serde`?][030602]
            - [Что такое десериализация с нулевым копированием? Почему она полезна? Как она работает в `serde`? Как она работает в `rkyv`?][030603]
            - [Inlining][030604]
        - [ ] [Шаг 3.7: Случайность и криптография][Step 3.7] (1 day)
            - [В чём заключается главный компромисс при генерации случайных чисел? Как это применяется на практике?][030701]
            - [Что такое симметричная криптография? Что такое асимметричная криптография? Какие преимущества имеет каждая из них?][030702]
            - [Что такое подпись в асимметричной криптографии? Что такое шифрование в асимметричной криптографии? Как они работают при наличии одних и тех же закрытого и открытого ключей?][030703]
            - [Что такое хеш-функция? Что такое хеширование паролей? Почему для хеширования паролей недостаточно использовать только обычную хеш-функцию?][030704]
            - [Что такое сравнение за постоянное время? Когда и почему его следует использовать?][030705]
            - [Какие существуют варианты использования TLS в Rust? Каковы преимущества и недостатки каждого из них?][030706]
        - [ ] [Шаг 3.8: Логирование и трассировка][Step 3.8] (1 day)
            - [Как библиотека `log` обеспечивает возможность повторного использования в экосистеме? Каковы лежащие в её основе идеи?][030801]
            - [Почему логирование предпочтительнее вывода текста на экран (использование `println!`)? А когда это не так?][030802]
            - [Что такое структурированное логирование? Какие преимущества оно предоставляет?][030803]
            - [Почему крейт `tracing` хорош для логирования? Чем он предпочтительнее крейтов `slog` и `log`?][030804]
            - [Что такое трассировка? Почему она полезна для обеспечения наблюдаемости?][030805]
        - [ ] [Шаг 3.9: Аргументы командной строки, переменные среды и конфигурации][Step 3.9] (1 day)
            - [В чём преимущества строгой типизации конфигурации?][030901]
            - [Почему переменные окружения полезны для настройки приложения? В чём их основное назначение?][030902]
            - [В чём польза crate `config`? Зачем его использовать, если нельзя просто десериализовать файл в структуры с помощью `serde`?][030903]
        - [ ] [Шаг 3.10: Многопоточность и параллелизм][Step 3.10] (1 day)
            - [Что такое параллелизм? Что такое конкуренция? Как они связаны друг с другом и чем отличаются?][031001]
            - [Как в Rust реализован параллелизм? Какие крейты обычно используются для его применения?][031002]
            - [Какие основные способы синхронизации потоков существуют в Rust? Каковы преимущества и недостатки каждого из них? Для чего используется каждый из них?][031003]
        - [ ] [Шаг 3.11: Асинхронный ввод-вывод, фьючерсы (futures) и акторы (actors)][Step 3.11] (2 days)
            - [Что такое асинхронное программирование? Как оно связано с многопоточностью? Какие проблемы оно решает? Каковы предпосылки для его существования?][031101]
            - [Как работает неблокирующий ввод-вывод? Чем он отличается от блокирующего ввода-вывода?][031102]
            - [Что такое `Future`? Зачем он нам нужен? Как он работает в Rust и чем его семантика отличается от других языков программирования? Что делает его нулевым по стоимости?][031103]
            - [Что такое `async`/`.await`? Как они преобразуют `future` в `future`? Почему они так важны для эргономики?][031104]
            - [Что такое асинхронная задача? Чем она отличается от `Future`?][031105]
            - [Что такое `Waker`? Как он работает? Зачем он нужен?][031106]
            - [Что такое асинхронная среда выполнения? Из каких частей она обычно состоит?][031107]
            - [Какой тип многозадачности представлен `Future` в Rust? Какие у них преимущества и недостатки?][031108]
            - [Какие типы асинхронных сред выполнения существуют в Rust для многопоточности? Каковы преимущества и недостатки каждой из них?][031109]
            - [Почему блокировка в асинхронной среде выполнения — это плохо? Как этого избежать на практике?][031110]
            - [В чём заключаются ключевые моменты парадигмы параллельного выполнения в рамках модели акторов? Какую пользу она может принести в Rust?][031111]
- [ ] [Шаг 4: Экосистема бэкэнда][Step 4] (3 days, after all sub-steps)
    - [Как и зачем мне взаимодействовать с базами данных в приложении на Rust? Как организовать миграции для моего проекта?][0401]
    - [Что следует использовать для реализации HTTP-сервера в Rust, когда и почему? А как насчет WebSocket-соединений?][0402]
    - [Какие существуют варианты для отправки HTTP-запросов (включая запросы WebSocket)?][0403]
    - [Что такое RPC (Remote procedure call)? Назовите несколько наиболее распространенных технологий, их преимущества и недостатки, объясните, какая из них может быть использована в каких обстоятельствах, и для чего и где она наиболее подходит?][0404]
    - [ ] [Шаг 4.1: Базы данных, пулы соединений и ORM][Step 4.1] (1 day)
        - [Что такое шаблон пула соединений? Как он работает? Какие проблемы он решает?][4101]
        - [Что такое паттерн ORM? Чем он отличается от построения запросов? Какие преимущества они дают?][4102]
        - [Почему написание SQL-запросов без использования сторонних инструментов может быть целесообразным? В каких случаях это применимо и когда это предпочтительнее, чем использование ORM?][4103]
        - [Что такое миграции? Зачем их использовать? Как они работают?][4104]
        - [Какие виды миграции существуют? Каковы их преимущества и недостатки? Когда и какой вид миграции предпочтительнее?][4105]
    - [ ] [Шаг 4.2: HTTP-серверы и клиенты][Step 4.2] (1 day)
        - [Что такое HTTP? Что подразумевает HTTP/2? Что подразумевает HTTP/3?][4201]
        - [Как парадигмы «перераспределения задач» и «поток на ядро» влияют на практическое программирование веб-сервера? Какая из них лучше и когда? Когда этот вопрос (выбор) становится актуальным на практике?][4202]
        - [Какие существуют распространённые крейты для выполнения HTTP-запросов в Rust? Какие компромиссы они имеют?][4203]
        - [Что такое WebSocket? Как он используется и когда? В двух словах, как он работает?][4204]
    - [ ] [Шаг 4.3: API-серверы, клиенты и инструменты][Step 4.3] (1 day)
        - [Что такое API? Что такое RPC? Как они связаны?][4301]
        - [Что означает подход «сначала код»? Что означает подход «сначала схема»? Каковы их преимущества и недостатки?][4302]
        - [Что означает парадигма REST? Каковы основные характеристики RESTful API? В чём её сильные стороны? Чего ей не хватает?][4303]
        - [Что такое OpenAPI? Что такое Swagger? Как они связаны? Почему они полезны для RESTful API?][4304]
        - [Что такое GraphQL? Каковы сильные стороны этой технологии? Какие проблемы она создает на практике?][4305]
        - [Что такое gRPC? В чём его преимущества? В каких случаях его применение целесообразно, а в каких — нет? Почему?][4306]

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

C:\Users\user\work\MyWorks\Rust\rust-incubator\3_ecosystem\README.md

[14]: 3_ecosystem/README.md#q-030001
[15]: 3_ecosystem/README.md#q-030002
[16]: 3_ecosystem/README.md#q-030003
[17]: 3_ecosystem/README.md#q-030004
[18]: 3_ecosystem/README.md#q-030005
[19]: 3_ecosystem/README.md#q-030006
[20]: 3_ecosystem/README.md#q-030007
[21]: 3_ecosystem/README.md#q-030008
[22]: 3_ecosystem/README.md#q-030009
[23]: 3_ecosystem/README.md#q-030010
[24]: 3_ecosystem/README.md#q-030011
[25]: 3_ecosystem/README.md#q-030012
[26]: 3_ecosystem/README.md#q-030013

[4101]: 4_backend/4_1_db/README.md#q-301
[4102]: 4_backend/4_1_db/README.md#q-302
[4103]: 4_backend/4_1_db/README.md#q-303
[4104]: 4_backend/4_1_db/README.md#q-304
[4105]: 4_backend/4_1_db/README.md#q-305

[4201]: 4_backend/4_2_http/README.md#q-4201
[4202]: 4_backend/4_2_http/README.md#q-4202
[4203]: 4_backend/4_2_http/README.md#q-4203
[4204]: 4_backend/4_2_http/README.md#q-4204

[4301]: 4_backend/4_3_api/README.md#q-4301
[4302]: 4_backend/4_3_api/README.md#q-4302
[4303]: 4_backend/4_3_api/README.md#q-4303
[4304]: 4_backend/4_3_api/README.md#q-4304
[4305]: 4_backend/4_3_api/README.md#q-4305
[4306]: 4_backend/4_3_api/README.md#q-4306

[0401]: 4_backend/README.md#q-0401
[0402]: 4_backend/README.md#q-0402
[0403]: 4_backend/README.md#q-0403
[0404]: 4_backend/README.md#q-0404

[030101]: 3_ecosystem/3_1_testing/README.md#q-030101
[030102]: 3_ecosystem/3_1_testing/README.md#q-030102
[030103]: 3_ecosystem/3_1_testing/README.md#q-030103
[030104]: 3_ecosystem/3_1_testing/README.md#q-030104

[030201]: 3_ecosystem/3_2_macro/README.md#q-030201
[030202]: 3_ecosystem/3_2_macro/README.md#q-030202
[030203]: 3_ecosystem/3_2_macro/README.md#q-030203
[030204]: 3_ecosystem/3_2_macro/README.md#q-030204
[030205]: 3_ecosystem/3_2_macro/README.md#q-030205

[030301]: 3_ecosystem/3_3_date_time/README.md#q-030301
[030302]: 3_ecosystem/3_3_date_time/README.md#q-030302
[030303]: 3_ecosystem/3_3_date_time/README.md#q-030303
[030304]: 3_ecosystem/3_3_date_time/README.md#q-030304

[030401]: 3_ecosystem/3_4_regex_parsing/README.md#q-030401
[030402]: 3_ecosystem/3_4_regex_parsing/README.md#q-030402
[030403]: 3_ecosystem/3_4_regex_parsing/README.md#q-030403
[030404]: 3_ecosystem/3_4_regex_parsing/README.md#q-030404
[030405]: 3_ecosystem/3_4_regex_parsing/README.md#q-030405
[030406]: 3_ecosystem/3_4_regex_parsing/README.md#q-030406

[030501]: 3_ecosystem/3_5_collections/README.md#q-030501
[030502]: 3_ecosystem/3_5_collections/README.md#q-030502
[030503]: 3_ecosystem/3_5_collections/README.md#q-030503

[030601]: 3_ecosystem/3_6_serde/README.md#q-030601
[030602]: 3_ecosystem/3_6_serde/README.md#q-030602
[030603]: 3_ecosystem/3_6_serde/README.md#q-030603
[030604]: 3_ecosystem/3_6_serde/README.md#q-030604

[030701]: 3_ecosystem/3_7_rand_crypto/README.md#q-030701
[030702]: 3_ecosystem/3_7_rand_crypto/README.md#q-030702
[030703]: 3_ecosystem/3_7_rand_crypto/README.md#q-030703
[030704]: 3_ecosystem/3_7_rand_crypto/README.md#q-030704
[030705]: 3_ecosystem/3_7_rand_crypto/README.md#q-030705
[030706]: 3_ecosystem/3_7_rand_crypto/README.md#q-030706

[030801]: 3_ecosystem/3_8_log/README.md#q-030801
[030802]: 3_ecosystem/3_8_log/README.md#q-030802
[030803]: 3_ecosystem/3_8_log/README.md#q-030803
[030804]: 3_ecosystem/3_8_log/README.md#q-030804
[030805]: 3_ecosystem/3_8_log/README.md#q-030805

[030901]: 3_ecosystem/3_9_cmd_env_conf/README.md#q-030901
[030902]: 3_ecosystem/3_9_cmd_env_conf/README.md#q-030902
[030903]: 3_ecosystem/3_9_cmd_env_conf/README.md#q-030903

[031001]: 3_ecosystem/3_10_threads/README.md#q-031001
[031002]: 3_ecosystem/3_10_threads/README.md#q-031002
[031003]: 3_ecosystem/3_10_threads/README.md#q-031003

[031101]: 3_ecosystem/3_11_async/README.md#q-031101
[031102]: 3_ecosystem/3_11_async/README.md#q-031102
[031103]: 3_ecosystem/3_11_async/README.md#q-031103
[031104]: 3_ecosystem/3_11_async/README.md#q-031104
[031105]: 3_ecosystem/3_11_async/README.md#q-031105
[031106]: 3_ecosystem/3_11_async/README.md#q-031106
[031107]: 3_ecosystem/3_11_async/README.md#q-031107
[031108]: 3_ecosystem/3_11_async/README.md#q-031108
[031109]: 3_ecosystem/3_11_async/README.md#q-031109
[031110]: 3_ecosystem/3_11_async/README.md#q-031110
[031111]: 3_ecosystem/3_11_async/README.md#q-031111

[020601]: 2_idioms/2_6_sealing/README.md#q-020601
[020602]: 2_idioms/2_6_sealing/README.md#q-020602
[020603]: 2_idioms/2_6_sealing/README.md#q-020603
[020604]: 2_idioms/2_6_sealing/README.md#q-020604

[020501]: 2_idioms/2_5_exhaustivity/README.md#q-020501
[020502]: 2_idioms/2_5_exhaustivity/README.md#q-020502

[020401]: 2_idioms/2_4_generic_in_type_out/README.md#q-020401
[020402]: 2_idioms/2_4_generic_in_type_out/README.md#q-020402

[020301]: 2_idioms/2_3_bound_impl/README.md#q-020301
[020302]: 2_idioms/2_3_bound_impl/README.md#q-020302
[020303]: 2_idioms/2_3_bound_impl/README.md#q-020303
[020304]: 2_idioms/2_3_bound_impl/README.md#q-020304

[020201]: 2_idioms/2_2_mem_replace/README.md#q-020201
[020202]: 2_idioms/2_2_mem_replace/README.md#q-020202

[020101]: 2_idioms/2_1_type_safety/README.md#q-020101
[020102]: 2_idioms/2_1_type_safety/README.md#q-020102
[020103]: 2_idioms/2_1_type_safety/README.md#q-020103

[020001]: 2_idioms/README.md#q-020001
[020002]: 2_idioms/README.md#q-020002
[020003]: 2_idioms/README.md#q-020003
[020004]: 2_idioms/README.md#q-020004
[020005]: 2_idioms/README.md#q-020005

[010901]: 1_concepts/1_9_phantom/README.md#q-010901
[010902]: 1_concepts/1_9_phantom/README.md#q-010902
[010903]: 1_concepts/1_9_phantom/README.md#q-010903
[010904]: 1_concepts/1_9_phantom/README.md#q-010904

[010801]: 1_concepts/1_8_thread_safety/README.md#q-010801
[010802]: 1_concepts/1_8_thread_safety/README.md#q-010802

[010701]: 1_concepts/1_7_sized/README.md#q-010701
[010702]: 1_concepts/1_7_sized/README.md#q-010702

[010601]: 1_concepts/1_6_dispatch/README.md#q-010601
[010602]: 1_concepts/1_6_dispatch/README.md#q-010602
[010603]: 1_concepts/1_6_dispatch/README.md#q-010603
[010604]: 1_concepts/1_6_dispatch/README.md#q-010604
[010605]: 1_concepts/1_6_dispatch/README.md#q-010605


[010501]: 1_concepts/1_5_convert_cast_deref/README.md#q-010501
[010502]: 1_concepts/1_5_convert_cast_deref/README.md#q-010502
[010503]: 1_concepts/1_5_convert_cast_deref/README.md#q-010503
[010504]: 1_concepts/1_5_convert_cast_deref/README.md#q-010504
[010505]: 1_concepts/1_5_convert_cast_deref/README.md#q-010505


[010401]: 1_concepts/1_4_cow/README.md#q-010401
[010402]: 1_concepts/1_4_cow/README.md#q-010402

[010301]: 1_concepts/1_3_rc_cell/README.md#q-010301
[010302]: 1_concepts/1_3_rc_cell/README.md#q-010302
[010303]: 1_concepts/1_3_rc_cell/README.md#q-010303
[010304]: 1_concepts/1_3_rc_cell/README.md#q-010304
[010305]: 1_concepts/1_3_rc_cell/README.md#q-010305
[010306]: 1_concepts/1_3_rc_cell/README.md#q-010306

[010201]: 1_concepts/1_2_box_pin/README.md#q-010201
[010202]: 1_concepts/1_2_box_pin/README.md#q-010202
[010203]: 1_concepts/1_2_box_pin/README.md#q-010203
[010204]: 1_concepts/1_2_box_pin/README.md#q-010204
[010205]: 1_concepts/1_2_box_pin/README.md#q-010205
[010206]: 1_concepts/1_2_box_pin/README.md#q-010206


[010101]: 1_concepts/1_1_default_clone_copy/README.md#q-010101
[010102]: 1_concepts/1_1_default_clone_copy/README.md#q-010102
[010103]: 1_concepts/1_1_default_clone_copy/README.md#q-010103
[010104]: 1_concepts/1_1_default_clone_copy/README.md#q-010104

[010001]: 1_concepts/README.md#q-010001
[010002]: 1_concepts/README.md#q-010002
[010003]: 1_concepts/README.md#q-010003
[010004]: 1_concepts/README.md#q-010004
[010005]: 1_concepts/README.md#q-010005
[010006]: 1_concepts/README.md#q-010006
[010007]: 1_concepts/README.md#q-010007
[010008]: 1_concepts/README.md#q-010008
[010009]: 1_concepts/README.md#q-010009
[010010]: 1_concepts/README.md#q-010010
