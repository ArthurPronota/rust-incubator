Шаг 4: Экосистема бэкэнда
=========================

__Estimated time__: 3 days

В этих шагах описаны распространенные библиотеки и инструменты в экосистеме [Rust], необходимые для разработки веб-бэкенда.

> ❗️Перед завершением этого шага необходимо выполнить все его подшаги.


После выполнения этих заданий вы сможете ответить на следующие вопросы:
- [Как и зачем мне взаимодействовать с базами данных в приложении на Rust? Как организовать миграции для моего проекта?][0401]
- [Что следует использовать для реализации HTTP-сервера в Rust, когда и почему? А как насчет WebSocket-соединений?][0402]

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

#### Как взаимодействовать с БД?

Выбор инструмента зависит от нужного уровня абстракции:

1. SQLx (Query Builder / SQL-first):
   - Пишете чистый SQL.
   - Главная фишка: Макрос sqlx::query! проверяет ваши запросы на соответствие реальной схеме БД во время компиляции.
   - Полностью асинхронный и не требует сложной настройки моделей.
2. Diesel (ORM / Query Builder):
   - Строгая типизация таблиц через макросы.
   - Самая высокая производительность среди ORM в Rust.
   - Требует предварительной генерации схемы (schema.rs), что делает код очень предсказуемым.
3. SeaORM (Async ORM):
   - Построен на базе SQLx, но предоставляет привычный объектно-реляционный интерфейс (как в Hibernate или Entity Framework).
   - Удобен для быстрой разработки сложных бизнес-сущностей. 

#### Как организовать миграции?

Миграции позволяют версионировать изменения схемы БД (создание таблиц, добавление колонок) аналогично Git.

- SQLx: Имеет встроенную систему миграций. Вы создаете папку migrations/, добавляете туда SQL-файлы вида 20231010_create_users.sql, и вызываете sqlx migrate run через CLI или прямо в коде при старте приложения.
- Diesel: Использует собственный CLI инструмент. Команда diesel migration run применяет изменения, а структура БД фиксируется в коде проекта.
- Многоразовые скрипты: Миграции обычно содержат блоки up (применение изменений) и down (откат). Это гарантирует, что база данных на сервере будет идентична вашей локальной копии. 

<hr>

<a name="q-0402"><h3>Что следует использовать для реализации HTTP-сервера в Rust, когда и почему? А как насчет WebSocket-соединений?</h3></a>

Для реализации HTTP-сервера в Rust сейчас есть три лидера. Выбор зависит от того, что вам важнее: стабильность, скорость разработки или полный контроль.

#### 1. Axum (Рекомендация №1)

Это современный стандарт, который разрабатывается командой tokio (основная асинхронная среда Rust).

- Когда: В 90% случаев для новых проектов и микросервисов.
- Почему: Использует систему типов Rust для обработки запросов (Extractors). Если вы ошиблись в аргументах функции-обработчика, код просто не скомпилируется. Очень гибкий и легко расширяемый через слои (middleware) экосистемы tower.

#### 2. Actix-web

Старый, проверенный и экстремально быстрый фреймворк.

- Когда: Если нужна максимальная пропускная способность и проверенная годами стабильность.
- Почему: Построен на собственной системе акторов. У него огромное сообщество и куча готовых примеров. Он стабильно занимает топовые места в мировых бенчмарках (TechEmpower).

#### 3. Rocket

Фреймворк с упором на удобство разработчика (DX).

- Когда: Для прототипов или если вы переходите с Flask (Python) или Ruby on Rails.
- Почему: Минимум «бойлерплейта» (шаблонного кода). Использует макросы, чтобы сделать код максимально чистым и понятным. Однако он чуть менее гибкий, чем Axum.

#### А что с WebSocket?

В Rust работа с WebSocket обычно встроена в HTTP-фреймворки или подключается как легкое дополнение:

1. В Axum: Есть встроенный модуль axum::extract::ws. Вы просто добавляете WebSocketUpgrade в аргументы функции, и Axum берет на себя «рукопожатие» (handshake) и переключение протокола.
2. Библиотека tokio-tungstenite: Если вам нужен низкоуровневый контроль или вы пишете WebSocket-клиент. Это стандарт индустрии для работы с фреймами и потоками данных.
2. В Actix-web: Есть отличная поддержка через акторы, что удобно для управления состоянием комнат в чатах или игровых сессиях.

Почему это круто в Rust? Благодаря асинхронности (async/await), один сервер может держать десятки тысяч активных WebSocket-соединений, потребляя минимум оперативной памяти по сравнению с Node.js или Go.


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
[0402]: #q-0402