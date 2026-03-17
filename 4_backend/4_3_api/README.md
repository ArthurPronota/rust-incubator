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

For making request to existing [GraphQL][GraphQL] [API]s, you don't necessarily need a special crate in [Rust] for trivial cases, just [any HTTP client][231] is capable to send a [simple query/mutation request][232].

However, if more static guarantees is needed, then the [`graphql-client`] crate may be used, providing the __query-to-code approach__ ([Rust] code is generated from [GraphQL] files defining queries).

[`cynic`] crate takes the __opposite code-to-query approach__ of generating a [GraphQL] query out of [Rust] code and validating it statically against a provided [GraphQL] schema.

To be familiar with making [GraphQL] requests in [Rust], read through:
- [Official `graphql-client` crate description][`graphql-client`]
- [Official `cynic` crate docs][`cynic`]
- [Official `cynic` crate guide](https://cynic-rs.dev)




## gRPC

[gRPC] is a widely-adopted high performance [RPC] framework, having a __strict schema__, powered with pluggable support for load balancing, tracing, health checking and authentication, built on top of [HTTP/2] (and so, having a __mandatory encryption__), and __heavily using code-from-schema generation__.

To be familiar with [gRPC], read through:
- [gRPC docs: Introduction to gRPC][301]
- [gRPC docs: Core concepts, architecture and lifecycle][302]


### Server and client

For implementing a [gRPC] server in [Rust], there are two main production-ready crates in its ecosystem: [`tonic`] (pure [Rust] implementation, based on [`tokio`]) and [`grpcio`] (wrapper around [gRPC core][311] implementation).

In [gRPC] ecosystem, usually, implementing a [gRPC] client doesn't differ much from implementing a server, since both are auto-generated from the same `.proto` schema. So, for [Rust], the same [`tonic`] and [`grpcio`] crates do the job when it comes to making [gRPC] requests. 

To be familiar with using [gRPC] in [Rust], read through:
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

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is API? What is RPC? How do they relate?
- What does "code-first" approach mean? What does "schema-first" approach mean? Which advantages and disadvantages do they have?
- What does REST paradigm mean? What are essentials of RESTful API? Which strengths does it have? What does it lack?  
- What is OpenAPI? What is Swagger? How do they relate? Why are they beneficial for RESTful API?
- What is GraphQL? Which are strong sides of this technology? What problems does it bring in practice? 
- What is gRPC? What are its strengths? Which are good use-cases for it, and which are not? Why? 




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
