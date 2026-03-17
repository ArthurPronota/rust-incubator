Шаг 4.3: API-серверы, клиенты и инструменты
========================================

__Estimated time__: 1 day

Естественно, в [клиент-серверных][4] приложениях клиент и сервер взаимодействуют друг с другом через некоторый [API (интерфейс прикладного программирования - application programming interface)][API], который часто принимает форму [RPC (удаленный вызов процедур - remote procedure call)][RPC] для лучшей структуризации и стандартизации (из-за использования [IDL (язык определения интерфейса - interface definition language)][IDL]).


Экосистема [Rust] обеспечивает поддержку всех современных широко используемых и распространенных [RPC] технологий и даже включает в себя свои [уникальные технологии][`tarpc`].


## RESTful

Поскольку [REST] — это __скорее архитектурная конвенция/стиль__, чем [строгая спецификация][3] для [RPC], и [REST]ful [API] обычно _свободно основаны на методах [HTTP] напрямую, в [Rust] обычно нет необходимости в специальных фреймворках реализовывать [REST]ful [API]-сервер или запрашивать его. Подойдет любой [HTTP-сервер][101] или [HTTP-клиент][231].

REST (REpresentational State Transfer) — это архитектурный стиль для проектирования сетевых приложений.

Однако этот подход __страдает от отсутствия схемы [API]__, что затрудняет создание богатой экосистемы с готовыми инструментами (или подключение к существующим). К счастью, это легко решается с помощью конкретной спецификации [RPC][3] поверх соглашений [REST] и строгого следования ей.

Для получения более подробной информации о [REST] ознакомьтесь с материалами по ссылке:
- [Tyler Charboneau: What’s the Difference Between RPC and REST?][111]


### OpenAPI

[OpenAPI] (ранее [Swagger]) — это [спецификация][3] для [машинно-читаемого][102] [языка определения интерфейса][IDL], позволяющая описывать, создавать, использовать и визуализировать [REST]ful веб-[API]. В двух словах, [OpenAPI] — это __разновидность [RPC] на основе [REST]__.

> Спецификация OpenAPI (OAS) определяет стандартный, независимый от языка программирования интерфейс для HTTP API, который позволяет как людям, так и компьютерам обнаруживать и понимать возможности сервиса без доступа к исходному коду, документации или анализа сетевого трафика. При правильном определении потребитель может понимать и взаимодействовать с удаленным сервисом с минимальным количеством логики реализации.
>
> Определение OpenAPI затем может использоваться инструментами генерации документации для отображения API, инструментами генерации кода для создания серверов и клиентов на различных языках программирования, инструментами тестирования и во многих других случаях.


В экосистеме [Rust] большинство [OpenAPI]-крейтов следуют подходу __code-first__ (генерация [OpenAPI]-схемы из исходного кода). Наиболее известные крейты для этого — [`utoipa`], [`okapi`] и [`apistos`].


Для противоположной задачи (генерации исходного кода из схемы [OpenAPI]) в экосистеме [Rust] отсутствует собственная чистая реализация, и следует использовать оригинальный инструмент [OpenAPI] [`openapi-generator`] (работающий на основе крейта [`swagger`]).


Чтобы ознакомиться с [OpenAPI] и использовать его в [Rust], прочтите следующее:
- [OpenAPI Initiative]
- [SwaggerHub Documentation: OpenAPI 3.0 Tutorial][122]
- [Official `utoipa` crate docs][`cynic`]
- [Official `okapi` crate docs][`okapi`]
- [Official `apistos` crate docs][`apistos`]
- [Twilio Docs: Generate a Rust client for Twilio's API][121]
- [Fabian Odenthal: Auto-Generating & Validating OpenAPI Docs in Rust: A Streamlined Approach with Utoipa and Schemathesis][123]
- [Olly Dixon: Auto-generating API service using Rust, to TypeScript & Dart][124]
- [Joshua Mo: Working with OpenAPI using Rust][125]




## GraphQL

[GraphQL] - это [гибкий][200] язык запросов для [API], позволяющий запрашивать данные частично и составлять несколько вложенных запросов как один, приправленный схемой, имеющей [выразительную][201] [систему типов][1] (по сравнению с другими [API] схемы) и [very strong][202] [introspection][2] возможности "из коробки".


Одной из самых сильных сторон [GraphQL] является его [вся экосистема][203], построенная вокруг языка, позволяющая автоматически генерировать код из схемы (или схему из кода), получать документацию непосредственно из интроспекции, интерактивно играть с любыми [API] на игровых площадках, легко имитировать их и гораздо, гораздо больше. __Как только вы построите свою схему [GraphQL], все остальное будет готово к работе.__


Еще одно важное преимущество [GraphQL] заключается в том, что его протокол не зависит от [транспорта][204]__, поэтому одна и та же схема и запросы, используемые через [HTTP], легко повторно используются через [WebSocket]__, что позволяет [stream data][205] практически без усилий.

Чтобы ознакомиться с [GraphQL], прочтите следующее:
- [GraphQL docs: Introduction to GraphQL][206]
- [The Fullstack Tutorial for GraphQL][207]


### Server

Для реализации [GraphQL]-сервера на [Rust] существуют два основных крейта в его экосистеме: [`juniper`] (предоставляет больше статических гарантий) и [`async-graphql`] (более функциональный). Оба используют подход __manifest code-to-schema__ (написание кода на [Rust] и последующая генерация [GraphQL]-схемы на его основе), поскольку система типов [Rust] гораздо более выразительна, чем система [GraphQL].


Однако библиотека [`juniper-from-schema`] пытается двигаться в противоположном направлении и в некоторой степени успешно __обеспечивает подход преобразования схемы в код__ (генерация кода [Rust] с использованием [`juniper`] из предоставленной схемы [GraphQL]).

Чтобы ознакомиться с реализацией [GraphQL]-сервера на [Russ], прочтите следующее:
- [Official `juniper` crate docs][`juniper`]
- [Juniper Book]
- [Official `juniper-from-schema` crate docs][`juniper-from-schema`]
- [Official `async-graphql` crate docs][`async-graphql`]
- [Async-graphql Book]


### Client

Для отправки запросов к существующим API [GraphQL][GraphQL] вам не обязательно нужен специальный крейт в [Rust] для тривиальных случаев, достаточно [любого HTTP-клиента][231], способного отправить [простой запрос/запрос на изменение][232].


Однако, если требуются дополнительные статические гарантии, можно использовать библиотеку [`graphql-client`], которая обеспечивает подход __запрос-в-код__ (код на [Rust] генерируется из файлов [GraphQL], определяющих запросы).

[`cynic`] crate использует __противоположный подход от кода к запросу__, генерируя [GraphQL]-запрос из [Rust]-кода и статически проверяя его на соответствие предоставленной [GraphQL]-схеме.

Чтобы ознакомиться с выполнением [GraphQL]-запросов в [Russ], прочтите следующее:
- [Official `graphql-client` crate description][`graphql-client`]
- [Official `cynic` crate docs][`cynic`]
- [Official `cynic` crate guide](https://cynic-rs.dev)



## gRPC

[gRPC] — это широко распространенная высокопроизводительная [RPC] платформа, имеющая __строгую схему__, с возможностью расширения для балансировки нагрузки, трассировки, проверки работоспособности и аутентификации, построенная на основе [HTTP/2] (и, следовательно, имеющая __обязательное шифрование__) и _активно использующая генерацию кода из схемы__.

Чтобы ознакомиться с [gRPC], прочтите следующее:
- [gRPC docs: Introduction to gRPC][301]
- [gRPC docs: Core concepts, architecture and lifecycle][302]


### Server and client

Для реализации [gRPC]-сервера на [Rust] в его экосистеме есть два основных готовых к использованию крейта: [`tonic`] (чистая реализация на [Rust], основанная на [`tokio`]) и [`grpcio`] (обертка вокруг реализации [gRPC core][311]).

В экосистеме [gRPC] реализация [gRPC]-клиента обычно мало чем отличается от реализации сервера, поскольку оба автоматически генерируются из одной и той же схемы `.proto`. Таким образом, для [Rust] одни и те же крейты [`tonic`] и [`grpcio`] выполняют работу по отправке [gRPC]-запросов.


Чтобы ознакомиться с использованием [gRPC] в [Rust], прочтите следующее:
- [Official `tonic` crate docs][`tonic`]
- [Official `grpcio` crate docs][`grpcio`]




## Task

Rework [the task from the previous step](../4_2_http/README.md#task) in a ["thick client" paradigm][41]:
- Server represents a [REST]ful [API] with separate endpoints for each operation.
- [CLI] client parses commands by itself and makes accurate requests to the server [REST]ful [API].

It should be possible to perform all the operations via [cURL] (or any other [HTTP]/[API] client) directly on the [REST]ful [API] server, without using the [CLI] client.

Additionally, implement generation of [OpenAPI] schema out of you server [REST]ful [API] code, and generate [HTML] documentation from the generated [OpenAPI] schema.

Avoid architecture [over-engineering][42] for this task, just use simple, straightforward and obvious solutions.




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- [Что такое API? Что такое RPC? Как они связаны?][4301]
- [Что означает подход «сначала код»? Что означает подход «сначала схема»? Каковы их преимущества и недостатки?][4302]
- [Что означает парадигма REST? Каковы основные характеристики RESTful API? В чём её сильные стороны? Чего ей не хватает?][4303]
- [Что такое OpenAPI? Что такое Swagger? Как они связаны? Почему они полезны для RESTful API?][4304]

- What is GraphQL? Which are strong sides of this technology? What problems does it bring in practice? 
- What is gRPC? What are its strengths? Which are good use-cases for it, and which are not? Why? 

<hr>

<a name="q-4301"><h3>Что такое API? Что такое RPC? Как они связаны?</h3></a>

В Rust эти понятия описывают, как программы общаются друг с другом, но на разных уровнях абстракции.

#### 1. Что такое API в Rust?

__API (Application Programming Interface)__ — это широкий термин, обозначающий «контракт» между кодом. В Rust его делят на два типа:

- Public API (библиотечное): Набор публичных (pub) структур, трейтов и функций в вашем крейте. Это то, что другие программисты видят, когда пишут use my_crate::*.
- Web API (сетевое): Набор endpoints (как ваш /api/command в Axum), через которые внешние программы (фронтенд, мобилки) общаются с вашим сервером.

#### 2. Что такое RPC?

__RPC (Remote Procedure Call)__ — это «удаленный вызов процедур».

- Суть: Вы вызываете функцию в коде так, будто она локальная, но на самом деле она выполняется на другом сервере.
- В Rust: Вы не возитесь с путями /api/user/create, вы просто вызываете метод client.create_user(args).
- Популярные решения:
   - gRPC (tonic): Самый мощный стандарт (использует Protobuf).
   - JSON-RPC: Часто используется в блокчейнах и простых инструментах.
   - tRPC: Позволяет иметь общие типы между фронтендом и бэкендом.

#### 3. Как они связаны?

__RPC__ — это подвид __API__.

Если классический REST API (как ваш Router в Axum) ориентирован на ресурсы (существительные: /users, /roles), то RPC ориентирован на действия (глаголы: GetUserInfo, CalculateTax).


|Характеристика|REST API (Axum)|RPC (Tonic/gRPC)|
|--------------|---------------|----------------|
|Стиль|HTTP-методы (GET, POST, PUT)|Вызов функции|
|Данные|Обычно JSON (текст)|Protobuf (бинарные, очень быстро)|
|Типизация|Ручная проверка JSON|Строгая генерация кода из .proto файлов|

#### Когда что выбирать?

- Web API (REST/Axum): Если ваш сервер должен отвечать браузерам, сторонним интеграциям или простому фронтенду. Это стандарт де-факто для интернета.
- RPC (gRPC): Если у вас много микросервисов на Rust, которые должны общаться друг с другом максимально быстро и с гарантией, что типы данных совпадают.

Ваш текущий код с handle_command — это Web API. Он принимает JSON-команду и выполняет её. Это очень близко к идее RPC, но реализовано поверх обычного HTTP-роутинга.

<hr>

<a name="q-4302"><h3>Что означает подход «сначала код»? Что означает подход «сначала схема»? Каковы их преимущества и недостатки?</h3></a>

Эти подходы описывают, с чего вы начинаете проектирование взаимодействия между клиентом и сервером. В Rust выбор между ними часто определяет, какие библиотеки вы будете использовать.

#### 1. Сначала код (Code-first)

Вы пишете обычные структуры и функции на Rust, а затем используете макросы, чтобы превратить их в API.

- Как это выглядит: Вы создаете struct User { ... }, добавляете #[derive(Serialize)] и подключаете к роутеру Axum. Спецификация (например, -OpenAPI/Swagger) генерируется автоматически из вашего кода.
- Инструменты: Axum, Actix-web, Rocket, крейт utaipa (для генерации Swagger).

__Преимущества:__

- Скорость: Вы сразу пишете логику, не отвлекаясь на сторонние файлы конфигурации.
- Гибкость: Легко менять структуру данных прямо в коде.
- Rust-way: Вы используете всю мощь системы типов Rust (Enums, Generics) без ограничений внешних схем.

__Недостатки:__

- Сложность для клиента: Если у вас нет автогенерации документации, фронтенд-разработчику придется гадать, какие поля ожидает ваш JSON.
- Риск поломки: Вы изменили имя поля в Rust — и у всех клиентов внезапно упало приложение.

#### 2. Сначала схема (Schema-first / Design-first)

Вы сначала описываете контракт (интерфейс) в специальном файле, а затем Rust-код генерируется на его основе.

- Как это выглядит: Вы пишете файл service.proto (для gRPC) или openapi.yaml. Специальный компилятор (например, prost или tonic-build) создает за вас структуры Rust.
- Инструменты: gRPC (Tonic), GraphQL (Juniper/Async-graphql), OpenAPI.

__Преимущества:__

- Строгий контракт: Клиент и сервер заранее договорились о форматах данных. Это исключает ошибки типа «ой, я забыл, что это поле теперь число».
- Мультиязычность: Вы написали одну схему, и на её основе сгенерировали код для Rust-сервера, Swift-приложения и TypeScript-фронтенда.
- Документация: Она уже встроена в процесс проектирования.

__Недостатки:__

- Лишний шаг: Любое изменение в коде требует изменения схемы и пересборки (генерации) проекта.
- Ограничения: Вы ограничены тем, что поддерживает формат схемы (например, в Protobuf сложнее выразить некоторые сложные перечисления Rust).

#### Сводная таблица выбора

|Ситуация|Рекомендуемый подход|Почему?|
|--------|--------------------|-------|
|Быстрый прототип / MVP|Code-first (Axum)|Минимум бюрократии, максимум кода.|
|Микросервисы (Polyglot)|Schema-first (gRPC)|Гарантия, что сервис на Go поймет сервис на Rust.|
|Публичное API для всех|Schema-first (OpenAPI)|Удобная документация и готовые SDK для пользователей.|
|Внутреннее API одного проекта|Code-first|Меньше сложностей с поддержкой синхронности схемы и кода.|

Ваш текущий проект с handle_command — это типичный Code-first. Вы определили структуру Commands прямо в Rust и используете её.

<hr>

<a name="q-4303"><h3>Что означает парадигма REST? Каковы основные характеристики RESTful API? В чём её сильные стороны? Чего ей не хватает?</h3></a>

В Rust парадигма REST (Representational State Transfer) реализуется через сопоставление HTTP-методов с функциями-обработчиками (handlers) в таких фреймворках, как Axum, Actix-web или Rocket.

#### Основные характеристики RESTful API

1. Ресурсо-ориентированность: Все сущности (пользователи, роли) имеют свои URL (например, /api/roles).
2. Использование методов HTTP:
   - GET — чтение.
   - POST — создание.
   - PUT/PATCH — обновление.
   - DELETE — удаление.
3. Stateless (Без сохранения состояния): Каждый запрос содержит всю информацию, необходимую для его обработки. Сервер не хранит «сессию» клиента между запросами.
4. Единообразие интерфейса: Ответы обычно приходят в стандартизированном формате (чаще всего JSON).

#### Сильные стороны в Rust

- Производительность: Благодаря Tokio и Hyper, Rust-серверы обрабатывают тысячи REST-запросов с минимальным потреблением памяти.
- Типобезопасность: С помощью крейта serde вы гарантируете, что если пришел некорректный JSON, Rust выдаст ошибку еще до того, как начнет работать бизнес-логика.
- Экосистема: Огромное количество готовых инструментов для логирования, аутентификации (JWT) и работы с базами данных (SQLx).

#### Чего ей не хватает (Недостатки)

- Избыточность данных (Over-fetching): REST часто возвращает весь объект, даже если вам нужно только одно поле.
- Множественные запросы: Чтобы собрать сложную страницу, клиенту часто приходится делать 5-10 запросов к разным эндпоинтам.
- Отсутствие строгой схемы: В чистом REST (без Swagger/OpenAPI) клиент не знает заранее, какие поля вернутся в JSON, что может привести к ошибкам при обновлении бэкенда.
- Слабая поддержка Real-time: REST не предназначен для мгновенной передачи данных от сервера (для этого нужны WebSockets или gRPC).

В вашем проекте использование Axum с post(handle_command) — это шаг в сторону REST, но с элементами RPC (так как вы передаете команду в теле запроса, а не манипулируете ресурсом через URL).

<hr>

<a name="q-4304"><h3>Что такое OpenAPI? Что такое Swagger? Как они связаны? Почему они полезны для RESTful API?</h3></a>

<hr>

[`apistos`]: https://docs.rs/apistos
[`async-graphql`]: https://docs.rs/async-graphql
[`cynic`]: https://docs.rs/cynic
[`graphql-client`]: https://github.com/graphql-rust/graphql-client
[`grpcio`]: https://docs.rs/crate/grpcio
[`juniper`]: https://docs.rs/juniper
[`juniper-from-schema`]: https://docs.rs/juniper-from-schema
[`okapi`]: https://github.com/GREsau/okapi
[`openapi-generator`]: https://github.com/OpenAPITools/openapi-generator
[`swagger`]: https://docs.rs/swagger
[`tarpc`]: https://docs.rs/tarpc
[`tonic`]: https://docs.rs/tonic
[`tokio`]: https://docs.rs/tokio
[`utoipa`]: https://docs.rs/utoipa
[API]: https://en.wikipedia.org/wiki/API
[Async-graphql Book]: https://async-graphql.github.io/async-graphql/en
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[cURL]: https://en.wikipedia.org/wiki/CURL
[GraphQL]: https://graphql.org
[gRPC]: https://grpc.io
[HTML]: https://en.wikipedia.org/wiki/HTML
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[HTTP/2]: https://en.wikipedia.org/wiki/HTTP/2
[IDL]: https://en.wikipedia.org/wiki/Interface_description_language
[Juniper Book]: https://graphql-rust.github.io/juniper/master
[OpenAPI]: https://en.wikipedia.org/wiki/OpenAPI_Specification
[OpenAPI Initiative]: https://learn.openapis.org
[REST]: https://en.wikipedia.org/wiki/Representational_state_transfer
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[Rust]: https://www.rust-lang.org
[Swagger]: https://en.wikipedia.org/wiki/Swagger_(software)
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[1]: https://en.wikipedia.org/wiki/Type_system
[2]: https://en.wikipedia.org/wiki/Type_introspection
[3]: https://en.wikipedia.org/wiki/Specification_(technical_standard)
[4]: https://en.wikipedia.org/wiki/Client%E2%80%93server_model
[101]: ../4_2_http/README.md#server
[102]: https://en.wikipedia.org/wiki/Machine-readable_medium_and_data
[111]: https://nordicapis.com/whats-the-difference-between-rpc-and-rest
[121]: https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api
[122]: https://support.smartbear.com/swaggerhub/docs/tutorials/openapi-3-tutorial.html
[123]: https://identeco.de/en/blog/generating_and_validating_openapi_docs_in_rust
[124]: https://www.polydelic.com/media/autogenerating-a-rust-api-to-typescript-and-dart
[125]: https://www.shuttle.rs/blog/2024/04/04/using-openapi-rust
[200]: https://graphql.org/learn/queries
[201]: https://graphql.org/learn/schema
[202]: https://graphql.org/learn/introspection
[203]: https://github.com/chentsulin/awesome-graphql#tools
[204]: https://en.wikipedia.org/wiki/Transport_layer
[205]: https://www.apollographql.com/docs/react/data/subscriptions
[206]: https://graphql.org/learn
[207]: https://www.howtographql.com
[231]: ../4_2_http/README.md#client
[232]: https://graphql.org/learn/serving-over-http
[301]: https://grpc.io/docs/what-is-grpc/introduction
[302]: https://grpc.io/docs/what-is-grpc/core-concepts
[311]: https://github.com/grpc/grpc
[41]: https://en.wikipedia.org/wiki/Rich_client
[42]: https://en.wikipedia.org/wiki/Overengineering

[4301]: #q-4301
[4302]: #q-4302
[4303]: #q-4303
[4304]: #q-4304