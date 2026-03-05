Шаг 4.1: Базы данных, пулы соединений и ORM
==============================================

__Estimated time__: 1 day

Текущая ситуация с интеграцией баз данных в экосистему [Rust] хорошо иллюстрируется в [разделе «Awesome Rust»][1] и в [теме «Database» раздела «Are we web yet?»][2]: большинство драйверов полностью реализованы на [Rust], и лишь немногие используют существующие библиотеки в качестве оберток, и, конечно же, большинство из них используют [асинхронный ввод-вывод][3].


## Пул соединений

Важно понимать концепцию [пула соединений][11]. Она широко используется в ситуациях, когда программа представляет собой долго работающее приложение (например, [демоны][12] или [серверы][13]). Ключевой момент заключается в том, что __вместо создания нового соединения с базой данных каждый раз__ нам нужно взаимодействовать с ней, мы __предпочтительно предварительно создаем [пул][14] таких соединений и повторно используем их__. Поскольку создание соединения — довольно ресурсоемкая операция, применение этого шаблона приводит к значительному повышению производительности.

К счастью, экосистема [Rust] предоставляет универсальные реализации независимого от базы данных [пула соединений][1] в обоих вариантах: синхронном и асинхронном.

Чтобы лучше понять [пул соединений][1], прочтите следующее:
 - [Чарли Кастер: Что такое объединение соединений и почему это важно][15]

### Синхронный

Для синхронных соединений существует крейт [`r2d2`] (пионер среди таких крейтов, существовавший задолго до появления [асинхронного ввода-вывода][3] в [Rust]). Вы можете легко адаптировать его для своего конкретного случая использования (или базы данных), просто реализовав [его трейты][22]. Очевидно, что [реализации для распространенных драйверов][21] уже существуют.

Для получения более подробной информации ознакомьтесь с:
- [Официальная документация к библиотеке `r2d2`][`r2d2`]

### Асинхронный

Для асинхронных соединений в экосистеме [Rust] существует гораздо больше вариантов, обусловленных историческими причинами и большей конкуренцией (в результате большей популярности [асинхронного ввода-вывода][3]).


Самым первым историческим примером был крейт [`bb8`]. Он является аналогом крейта [`r2d2`] для асинхронных соединений (только [`tokio`]) и изначально был основан на нем. Аналогично, уже существуют [реализованные мосты для распространенных драйверов][23].


[`deadpool`] — это [альтернативная и очень зрелая][25] реализация шаблона [пула соединений][11], поддерживающая как [`tokio`], так и [`async-std`], обладающая [собственной обширной экосистемой][24].

Другой альтернативной реализацией является крейт [`mobc`], вдохновленный крейтами [`deadpool`] и [`r2d2`]. Аналогично, он поддерживает как [`tokio`], так и [`async-std`] и предоставляет некоторые [мосты для распространенных драйверов][26].


[`qp`] (Quick Pool) — это очень простая и [ограниченная][29] реализация шаблона [пула соединений][11], [использующая примитивы без блокировок][27] и [ориентированная на производительность][28].


Для получения более подробной информации ознакомьтесь с:
- [Official `bb8` crate docs][`bb8`]
- [Official `deadpool` crate docs][`deadpool`]
- [Official `mobc` crate docs][`mobc`]
- [Official `qp` crate docs][`qp`]



## Конструктор запросов

Построитель запросов — это, по сути, __шаблон проектирования [81], применяемый для построения запросов [SQL]__ (или других языков запросов данных [82]), __позволяющий писать их как обычный код [Rust]__ (и, следовательно, [используя встроенный DSL (Domain-specific language) вместо внешнего DSL][83]).

Каноническая реализация этого шаблона в экосистеме [Rust] представлена ​​крейтами [`sea-query`] и [`sql_query_builder`].

С другой стороны, [`barrel`] crate позволяет писать [миграции схемы][61], а не запрашивать данные.

Для получения более подробной информации ознакомьтесь с:
- [Official `sea-query` crate docs][`sea-query`]
- [Official `sql_query_builder` crate docs][`sql_query_builder`]
- [Official `barrel` crate docs][`barrel`]


### Не-[DSL] инструментарий

[`sqlx`] crate, будучи многофункциональным инструментом для [SQL], здесь использует [совершенно противоположный подход][91]: он фокусируется на написании чистых [SQL] запросов (без пользовательского [DSL], без [построения запросов](#query-builder)), корректность которых статически проверяется во время компиляции.

Чтобы лучше понять архитектуру, концепции, использование и возможности [`sqlx`], ознакомьтесь со следующими материалами:
- [Official `sqlx` crate docs][`sqlx`]


## ORM: Object-Relational Mapping (Объектно-реляционное отображение)

Что касается [паттерна ORM][41], то в экосистеме [Rust] на данный момент существует [множество][42] многофункциональных и зрелых реализаций. Каждая из них имеет свой уникальный дизайн, преимущества и недостатки.


Самым первым [ORM][41], созданным в [Rust], был крейт [`diesel`]. Даже сейчас он поддерживает [только синхронные][43] соединения (как и до появления [асинхронного ввода-вывода][3] в [Rust]). Однако, благодаря расширению [`diesel-async`], его все еще можно использовать с асинхронными соединениями.


[`sea-orm`] (построенный на основе [`sea-query`]) — это альтернативная, многофункциональная и [зрелая][46] реализация паттерна [ORM] в [Rust], ориентированная на [динамические запросы, чтобы избежать сложности статических проверок ("борьба с ORM")][47].

[`ormx`] — это облегченное расширение крейта [`sqlx`], призванное предоставить ему возможности, аналогичные [ORM][41].

[`rustorm`] — это очень простой и ориентированный на [SQL] [ORM][41], предназначенный для упрощения преобразования типов баз данных в соответствующие типы [Rust].

Чтобы лучше понять дизайн, концепции, использование и функции [ORM][41], прочтите следующее:
- [Official `diesel` crate docs][`diesel`]
- [Official `diesel` crate guides][44]
- [Official `sea-orm` crate docs][`sea-orm`]
- [Official `sea-orm` crate guides][45]
- [Official `ormx` crate docs][`ormx`]
- [Official `rustorm` crate docs][`rustorm`]



## Миграции

__Миграции баз данных это система "контроля версий" для вашей базы данных, которая позволяет команде разработчиков синхронно и безопасно обновлять структуру БД.__

Для [миграции баз данных][61] в экосистеме [Rust] существует [множество инструментов][62].


Для пользователей [`diesel`] очевидным выбором является крейт [`diesel_migrations`] (который можно использовать напрямую через [`diesel_cli`]). Однако он не требует использования самого [`diesel`] и может использоваться как полностью отдельный инструмент.

Для пользователей [`sqlx`] инструмент [`sqlx-cli`] [предоставляет миграции][64] «из коробки», а также может использоваться [напрямую в коде приложения][65].

[`refinery`] и [`migrant`] — это ещё один автономный [Rust] инструмент для [миграций][61], позволяющий использовать его как в командной строке, так и в ["коде приложения"][66]. Интересной особенностью крейта [`refinery`] является то, что он также позволяет писать [миграции" в "коде приложения"][61] с помощью построителя миграций схемы [`barrel`].

Чтобы ознакомиться с инструментами [миграции][61], их сходствами и различиями, прочтите следующее:
- [Official `diesel_migrations` crate docs][`diesel_migrations`]
- [Official `diesel_cli` crate docs][`diesel_cli`]
- [Official `diesel` crate guides: Getting Started][63]
- [Official `sqlx` crate docs: Macro `sqlx::migrate`][65]
- [Official `refinery` crate docs][`refinery`]
- [Official `migrant` crate docs][`migrant`]




## Task

Create an [SQL] database ([PostgreSQL], [MySQL] or [SQLite], on your choice) consisting of the following tables:
- `users`: `id`, `name` and any other fields on your choice; 
- `roles`: [`slug`][201] as a primary key, `name` and `permissions` (the concrete format on your choice) fields;
- `users_roles`: `users.id` to `roles.slug` many-to-many relationship.

Write a simple [CLI] application which allows to [CRUD] data in your database tables in the following ways:
- create and delete `users` and `roles` (a `user` must always have an assigned `role`);
- update fields of a single `user` or a `role`;
- assign or unassign a `role` to/from a `user`;
- list all `roles` or a single `role` by its `slug`;
- list all `users` or a single `user` by its `id` (a `user` should be displayed with all the `roles` assigned to him).

Consider to ensure [data consistency][202] in your database as much as possible.




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- [Что такое шаблон пула соединений? Как он работает? Какие проблемы он решает?][301]

- What is ORM pattern? How does it differ from query building? What benefits do they give?
- Why writing raw SQL queries could be meaningful? Which are use-cases for it and when is it preferred over ORMs?
- What are migrations? Why should we use them? How do they work? 
- Which kinds of migrations do exist? What are their advantages and disadvantages? When and which kind is preferred?  

<hr>

### <a name="q-301"></a> Что такое шаблон пула соединений? Как он работает? Какие проблемы он решает?

В Rust шаблон пула соединений (Connection Pool) — это стратегия управления дорогостоящими ресурсами (базами данных, сетевыми сокетами), при которой вместо создания нового соединения для каждого запроса используется набор уже открытых, «подогретых» соединений.

#### 1. Как это работает?

Вместо цепочки «Открыть -> Использовать -> Закрыть», пул реализует логику «Взять из очереди -> Использовать -> Вернуть в очередь».

- Инициализация: При старте приложения пул создает 
 соединений (например, 10) и держит их открытыми.
- Запрос (Checkout): Когда вашему коду нужно отправить запрос в БД, он запрашивает соединение у пула. Если свободное есть — оно выдается мгновенно.
- Ожидание: Если все 10 соединений заняты, ваш поток (или асинхронная задача) встает в очередь и ждет, пока кто-то другой освободит ресурс.
- Возврат (Release): Как только вы закончили работу, соединение не закрывается физически, а помечается как свободное в пуле.

#### 2. Какие проблемы он решает?

- Низкая задержка (Latency): Установление TCP-соединения и TLS-рукопожатие с БД может занимать 50–200 мс. Пул сокращает это время до микросекунд, так как соединение уже готово.
- Защита ресурсов (Backpressure): Базы данных имеют лимит на количество одновременных подключений. Пул служит «предохранителем», не позволяя приложению случайно «положить» БД созданием тысячи потоков.
- Экономия CPU и RAM: Создание и уничтожение объектов соединений нагружает аллокатор памяти и сборщик мусора (в других языках), а в Rust — просто тратит лишние циклы процессора.

#### 3. Популярные решения в Rust

- sqlx: Современный стандарт для асинхронного Rust. Пул встроен прямо в него (PgPool, MySqlPool).
- deadpool: Универсальный асинхронный пул, который можно использовать для чего угодно (Redis, Postgress, даже для ваших собственных структур).
- r2d2: Классический синхронный пул для блокирующего кода.

#### Пример на sqlx:

```rust
// Создаем пул из 5 соединений
let pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect("mysql://user:pass@localhost/db")
    .await?;

// Это не открывает новое соединение, а берет его из пула
let row = sqlx::query("SELECT ...").fetch_one(&pool).await?; 
```

___Итог___: Пул соединений — это обязательный элемент любого серверного приложения на Rust, который делает его быстрым и стабильным под нагрузкой.

<hr>

[`async-std`]: https://docs.rs/async-std
[`barrel`]: https://docs.rs/barrel
[`bb8`]: https://docs.rs/bb8
[`deadpool`]: https://docs.rs/deadpool
[`diesel`]: https://docs.rs/diesel
[`diesel_cli`]: https://docs.rs/diesel_cli
[`diesel_migrations`]: https://docs.rs/diesel_migrations
[`diesel-async`]: https://docs.rs/diesel-async
[`migrant`]: https://docs.rs/migrant
[`mobc`]: https://docs.rs/mobc
[`ormx`]: https://docs.rs/ormx
[`qp`]: https://github.com/Astro36/qp
[`r2d2`]: https://docs.rs/r2d2
[`refinery`]: https://docs.rs/refinery
[`rustorm`]: https://docs.rs/crate/rustorm
[`sea-orm`]: https://docs.rs/sea-orm
[`sea-query`]: https://docs.rs/sea-query
[`sql_query_builder`]: https://docs.rs/sql_query_builder
[`sqlx`]: https://docs.rs/crate/sqlx
[`sqlx-cli`]: https://docs.rs/crate/sqlx-cli
[`tokio`]: https://docs.rs/tokio
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[CRUD]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
[DSL]: https://en.wikipedia.org/wiki/Domain-specific_language
[MySQL]: https://www.mysql.com
[PostgreSQL]: https://www.postgresql.org
[Rust]: https://www.rust-lang.org
[SQL]: https://en.wikipedia.org/wiki/SQL
[SQLite]: https://www.sqlite.org

[1]: https://github.com/rust-unofficial/awesome-rust#database-1
[2]: https://www.arewewebyet.org/topics/database
[3]: ../../3_ecosystem/3_11_async
[11]: https://en.wikipedia.org/wiki/Connection_pool
[12]: https://en.wikipedia.org/wiki/Daemon_(computing)
[13]: https://en.wikipedia.org/wiki/Server_(computing)
[14]: https://en.wikipedia.org/wiki/Object_pool_pattern
[15]: https://www.cockroachlabs.com/blog/what-is-connection-pooling
[21]: https://crates.io/search?q=r2d2
[22]: https://docs.rs/r2d2#traits
[23]: https://crates.io/search?q=bb8
[24]: https://crates.io/search?q=deadpool
[25]: https://docs.rs/deadpool#reasons-for-yet-another-connection-pool
[26]: https://crates.io/search?q=mobc
[27]: https://github.com/Astro36/qp#bb8-vs-qp
[28]: https://github.com/Astro36/qp#performance-comparison
[29]: https://github.com/Astro36/qp#dbcp
[41]: https://en.wikipedia.org/wiki/Object-relational_mapping
[42]: https://www.arewewebyet.org/topics/database#orms
[43]: https://github.com/diesel-rs/diesel/issues/399
[44]: https://diesel.rs/guides
[45]: https://www.sea-ql.org/SeaORM/docs/index
[46]: https://docs.rs/sea-orm#whos-using-seaorm
[47]: https://www.sea-ql.org/SeaORM/docs/internal-design/diesel#programming-paradigm
[61]: https://en.wikipedia.org/wiki/Schema_migration
[62]: https://www.arewewebyet.org/topics/database#tooling
[63]: https://diesel.rs/guides/getting-started
[64]: https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#create-and-run-migrations
[65]: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
[66]: https://docs.rs/refinery/latest/refinery/macro.embed_migrations.html
[81]: https://en.wikipedia.org/wiki/Builder_pattern
[82]: https://en.wikipedia.org/wiki/Query_language
[83]: https://en.wikipedia.org/wiki/Domain-specific_language#External_and_Embedded_Domain_Specific_Languages
[91]: https://github.com/launchbadge/sqlx#sqlx-is-not-an-orm
[201]: https://en.wikipedia.org/wiki/Clean_URL#Slug 
[202]: https://en.wikipedia.org/wiki/Consistency_(database_systems)

[301]: https://github.com/ArthurPronota/rust-incubator/tree/main/4_backend/4_1_db#q-301