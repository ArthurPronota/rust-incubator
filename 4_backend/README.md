Шаг 4: Экосистема бэкэнда
=========================

__Estimated time__: 3 days

В этих шагах описаны распространенные библиотеки и инструменты в экосистеме [Rust], необходимые для разработки веб-бэкенда.

> ❗️Перед завершением этого шага необходимо выполнить все его подшаги.


После выполнения этих заданий вы сможете ответить на следующие вопросы:
- [Как и зачем мне взаимодействовать с базами данных в приложении на Rust? Как организовать миграции для моего проекта?][0401]

- What should I use for [HTTP] server implementation in [Rust], when and why? What about [WebSocket] connections?
- What are options for making [HTTP] request (including [WebSocket] ones)?
- What is [RPC]? Name several the most adopted technologies, their advantages and disadvantages, explain which one could be used under which circumstances, and what and where is their best fit? 




## Task

Write a simple [GraphQL] API server with the following data model:
- `User` has `id` (unique), `name` (unique) and `friends` (list of other `User`s) fields.
- `User` is able to authenticate with its `password`.

API requirements:
- Ability to register users.
- Ability to authenticate users.
- Ability to retrieve a single user and all its friends (with their friends) (should require authorization).
- Ability to add some user to friends list and remove from there (should require authorization).

Web frameworks, tools and database choices are up to you. Keep things simple to fit into the dedicated time.

If you have enough time after implementing base requirements, consider to add the following to your solution:
- Provide migrations for database schema (if possible).
- Add comprehensive documentation to your code and [API], and generate it in [HTML] form.
- Cover your implementation with unit and E2E tests.
- Implement [GraphQL] query [depth limiting][21].
- Use [dataloading][22] to optimize interaction with database in [GraphQL] resolvers. 

<hr>

<a name="q-0401"><h3>Как и зачем мне взаимодействовать с базами данных в приложении на Rust? Как организовать миграции для моего проекта?</h3></a>

Взаимодействие с базами данных в Rust позволяет строить высокопроизводительные и безопасные бэкенд-сервисы, используя строгую типизацию для предотвращения ошибок на этапе компиляции.

#### Зачем использовать Rust для работы с БД?

- Безопасность памяти и типов: Rust гарантирует отсутствие утечек памяти и состояния гонки при многопоточном доступе к данным.
- Производительность: Минимальные накладные расходы (zero-cost abstractions) позволяют достичь скорости C++ при работе с запросами.
- Проверка запросов при компиляции: Некоторые инструменты (например, SQLx) проверяют синтаксис ваших SQL-запросов и типы данных еще до запуска приложения. 


<hr>

[API]: https://en.wikipedia.org/wiki/API
[GraphQL]: https://graphql.org/learn
[HTML]: https://en.wikipedia.org/wiki/HTML
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[Rust]: https://www.rust-lang.org
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[21]: https://escape.tech/blog/cyclic-queries-and-depth-limit
[22]: https://medium.com/the-marcy-lab-school/how-to-use-dataloader-js-9727c527efd0

[0401]: #q-0401