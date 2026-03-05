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


For more details, read through:
- [Official `r2d2` crate docs][`r2d2`]


### Asynchronous

For asynchronous connections there are much more options in [Rust] ecosystem, due to historical reasons and bigger competitiveness (as the result of bigger [async I/O][3] popularity).

The very first one, historically, was the [`bb8`] crate. It mirrors the [`r2d2`] crate for asynchronous connections ([`tokio`] only), and originally was based on it. Similarly, there are [implemented bridges for common drivers][23] already.

[`deadpool`] is an [alternative and very mature][25] implementation of the [connection pool][11] pattern, supporting both [`tokio`] and [`async-std`], provided with [its own large ecosystem][24].

Another alternative implementation is the [`mobc`] crate, yet inspired by [`deadpool`] and [`r2d2`] crates. Similarly, supports both [`tokio`] and [`async-std`] and provides some [bridges for common drivers][26].

[`qp`] (Quick Pool) is a very simple and [limited][29] implementation of the [connection pool][11] pattern, [utilizing lock-free primitives][27] and [focused on being performant][28].

For more details, read through:
- [Official `bb8` crate docs][`bb8`]
- [Official `deadpool` crate docs][`deadpool`]
- [Official `mobc` crate docs][`mobc`]
- [Official `qp` crate docs][`qp`]




## Query builder

Query builder is effectively a __[builder pattern][81] applied for building [SQL]__ (or other [data query languages][82]) queries, and __allowing to write them as a regular [Rust] code__ (and so, [using an embedded DSL instead of external DSL][83]).

The canonical implementation of this pattern in [Rust] ecosystem is represented by [`sea-query`] and [`sql_query_builder`] crates.

[`barrel`] crate, on the other hand, allows to write [schema migrations][61], rather than querying data.

For more details, read through:
- [Official `sea-query` crate docs][`sea-query`]
- [Official `sql_query_builder` crate docs][`sql_query_builder`]
- [Official `barrel` crate docs][`barrel`]


### Non-[DSL] toolkit

[`sqlx`] crate, while being a feature-rich toolkit for [SQL], takes a [completely opposite approach][91] here: it focuses on writing pure [SQL] queries (no custom [DSL], no [query building](#query-builder)), which are statically checked to be correct at compile-time.

To better understand [`sqlx`]'s design, concepts, usage and features, read through:
- [Official `sqlx` crate docs][`sqlx`]




## ORM

Regarding the [ORM pattern][41], there are [multiple][42] feature-rich and mature implementation in [Rust] ecosystem at the moment. Every one has its own unique design, advantages and disadvantages.

The very first [ORM][41] created in [Rust] was the [`diesel`] crate. Even now, it supports [only synchronous][43] connections (as was created before [async I/O][3] has landed in [Rust]). However, still may be used with asynchronous connections, thankfully to the [`diesel-async`] extension.

[`sea-orm`] (built on top of [`sea-query`]) is an alternative feature-rich and [mature][46] implementation of the [ORM] pattern in [Rust], focused on [dynamic querying to avoid complexity of static checks ("fighting the ORM")][47].

[`ormx`] is a lightweight extension of the [`sqlx`] crate, aimed to provide it with [ORM][41]-like features.

[`rustorm`] is a very simple and [SQL]-centered [ORM][41], focused on easing conversions of database types to their appropriate [Rust] types.

To better understand [ORMs'][41] design, concepts, usage and features, read through:
- [Official `diesel` crate docs][`diesel`]
- [Official `diesel` crate guides][44]
- [Official `sea-orm` crate docs][`sea-orm`]
- [Official `sea-orm` crate guides][45]
- [Official `ormx` crate docs][`ormx`]
- [Official `rustorm` crate docs][`rustorm`]




## Migrations

For [database migrations][61] there are [multiple tools][62] in [Rust] ecosystem.

For [`diesel`] users, the obvious choice is the [`diesel_migrations`] crate (which may be used directly via [`diesel_cli`]). Though, doesn't require the [`diesel`] itself to be used, and may be used as a fully separate tool.

For [`sqlx`] users, similarly, the [`sqlx-cli`] tool [provides migrations][64] out-of-the-box, while also may be used [directly in the application code][65].

[`refinery`] and [`migrant`] are another standalone [Rust] tools for [migrations][61], allowing both [CLI] and ["in-application-code"][66] usage. The interesting part about the [`refinery`] crate is that it also allows to write "in-application-code" [migrations][61] via the [`barrel`] schema migration builder.

To be familiar with [migrations][61] tools, their similarities and differences, read through:
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

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is connection pool pattern? How does it work? Which problems does it solve?
- What is ORM pattern? How does it differ from query building? What benefits do they give?
- Why writing raw SQL queries could be meaningful? Which are use-cases for it and when is it preferred over ORMs?
- What are migrations? Why should we use them? How do they work? 
- Which kinds of migrations do exist? What are their advantages and disadvantages? When and which kind is preferred?  




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
