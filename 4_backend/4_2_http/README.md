Шаг 4.2: HTTP-серверы и клиенты
==================================

__Estimated time__: 1 day

Текущая ситуация с [HTTP] в экосистеме [Rust] хорошо описана в [разделе «Веб-программирование» книги «Awesome Rust»][1] и в [разделах «Веб-фреймворки»][2], [«HTTP-клиенты»][3] и [«Нижний веб-стек» книги «Мы уже веб?»][4]. Конечно, большинство из них используют [асинхронный ввод-вывод][5].

## Low-level

Существует несколько основных библиотек, предоставляющих универсальную реализацию [HTTP], которая обеспечивает работу всего многообразия [веб-фреймворков][21] и [HTTP]-клиентов в экосистеме [Rust].


Наиболее известным и зрелым является, конечно же, крейт [`hyper`] (созданный с использованием [`tokio`]). Почти все [веб-фреймворки][21] экосистемы [Rust] построены на его основе.

Основные альтернативы:
- [`async-h1`], обеспечивающий работу экосистемы [`async-std`] для [HTTP].
- [`actix-http`], обеспечивающий работу экосистемы [`actix-web`].

Для получения более подробной информации ознакомьтесь с:
- [Official `hyper` crate docs][`hyper`]
- [Official `async-h1` crate docs][`async-h1`]
- [Official `actix-http` crate docs][`actix-http`]




## Server

Хотя [`hyper`] предоставляет собственную серверную реализацию, его прямое использование может показаться довольно низкоуровневым и неэргономичным из-за его природы. Естественно, существует [многочисленное множество веб-фреймворков][2], построенных на основе [`hyper`], которые предоставляют высокоуровневый, эргономичный и удобный интерфейс. Наиболее известные из них:
- [`axum`] — [фреймворк для веб-приложений][21], ориентированный на эргономику и модульность, обеспечивающий маршрутизацию запросов без макросов (но эргономичный и декларативный), простую и предсказуемую обработку ошибок, а также в полной мере использующий преимущества экосистемы [`tower`] и [`tower-http`] [промежуточного ПО][22], сервисов и утилит.
- [`warp`] — сверхпростой, компонуемый [фреймворк веб-сервера][21] для сверхбыстрой работы, построенный на концепции «все — это [`Filter`]».
- [`rocket`] — [веб-фреймворк][21], цель которого — быть быстрым, простым и гибким, обеспечивая при этом гарантированную безопасность и защиту там, где это возможно, и, что важно, быть интересным (достигая этого за счет того, что вы пишете как можно меньше кода, необходимого для выполнения вашей задачи).
- [`poem`] — полнофункциональный и простой в использовании [веб-фреймворк][21], ориентированный на предоставление всех возможностей (например, [i18n]) «из коробки».
- [`salvo`] — мощный и простой [фреймворк веб-сервера][21], использующий реализацию [HTTP/3].

Для тех, кто предпочитает экосистему [`async-std`], окончательным выбором (и единственным на данный момент) является крейт [`tide`].


Все [веб-фреймворки][21], описанные выше, наследуют [кражу работы][23] от асинхронной среды выполнения, в которой они выполняются, и, следовательно, требуют надлежащей синхронизации (например, [`Send`]) от предоставляемых пользователем обработчиков запросов [HTTP], что может привести к ненужному или нежелательные накладные расходы. Вот почему __[`actix-web`] crate был разработан__ и реализован специально с учетом этого (__чтобы избежать [кражи работы][23]__), будучи созданным поверх [`actix-rt`] crate (используя [thread-модель для каждого ядра][24]), и, таким образом, не требует никакой синхронизации в своих обработчиках запросов (позволяя `!Send` [`Future`]s). Кроме того, [`actix-web`] на тот момент был первым зрелым и готовым к работе [веб-фреймворком][21] в экосистеме [Rust], который входил в [топ "Тестов веб-фреймворка TechEmpower"][25].


Чтобы лучше понять и освоить HTTP-серверы в Rust, прочтите следующее:
- [Official `actix-web` crate docs][`actix-web`]
- [Official `actix-web` crate guides: Server](https://actix.rs/docs/server)
- [Official `axum` crate docs][`axum`]
- [Official `warp` crate docs][`warp`]
- [Official `rocket` crate docs][`rocket`]
- [Official `poem` crate docs][`poem`]
- [Official `salvo` guide](https://salvo.rs/guide)
- [Official `tide` crate docs][`tide`]
- [Official `hyper` crate guides: Server][26]




## Client

Подобно серверу, хотя [`hyper`] предоставляет собственную реализацию клиента, его прямое использование может показаться довольно низкоуровневым и неэргономичным. Поэтому «вариантом по умолчанию» (и наиболее часто используемым) [HTTP] клиентом в экосистеме [Rust] является крейт [`reqwest`], построенный на основе [`hyper`].

В качестве альтернативы, [`isahc`] crate представляет собой независимую от среды выполнения обертку (с основным упором на практичность и эргономичность) над известной библиотекой [cURL].

Для простых и тривиальных сценариев, __где асинхронная среда выполнения избыточна__ и/или предпочтительны низкие накладные расходы__, жизнеспособной альтернативой является крейт [`ureq`].

Для экосистемы [`async-std`] основной библиотекой является [`surf`], которая, однако, не ограничивается только [`async-std`] и может использовать альтернативные бэкенды: [cURL] (через [`isahc`]), [`hyper`], [WASM] (через [API `window.fetch` браузера][32]).

Для экосистемы [`actix-web`] наиболее подходящим вариантом будет крейт [`awc`], который поддерживает соединения [WebSocket] из коробки (в то время как большинство других [HTTP] клиентов этого не делают).

Чтобы лучше понять и освоить работу с HTTP-клиентами в Rust, прочтите следующее:
- [Official `reqwest` crate docs][`reqwest`]
- [Joshua Mo: Writing a Web Scraper in Rust using Reqwest][33]
- [Official `isahc` crate docs][`isahc`]
- [Official `ureq` crate docs][`ureq`]
- [Official `surf` crate docs][`surf`]
- [Official `awc` crate docs][`awc`]
- [Official `hyper` crate guides: Client][31]




## WebSocket

Многие [HTTP] клиенты и серверы в [Rust] не имеют встроенной реализации [WebSocket]. Поэтому был создан крейт [`tungstenite`], предоставляющий базовую и независимую от конкретного языка реализацию [WebSocket]. Такие крейты, как [`async-tungstenite`] и [`tokio-tungstenite`], предоставляют готовую к использованию реализацию клиента/сервера для желаемой экосистемы и асинхронной среды выполнения.

Для экосистемы [`actix-web`] идиоматическим решением является модуль [`actix-web-actors::ws`], обеспечивающий реализацию в виде [actor][41] (через [`actix`]).

Чтобы лучше понять и освоить реализацию [WebSocket] в [Rust], прочтите следующее:
- [Official `tungstenite` crate docs][`tungstenite`]
- [Official `async-tungstenite` crate docs][`async-tungstenite`]
- [Official `tokio-tungstenite` crate docs][`tokio-tungstenite`]
- [Official `actix-web-actors::ws` module docs][`actix-web-actors::ws`]




## Task

Rework [the task from the previous step](../4_1_db/README.md#task) in a [client-server architecture][51]. It should consist of a [CLI] client and a server [daemon][52], and utilize the ["thin client" approach][53]:
- [CLI] client does nothing except sending commands "as is" to the server and rendering its responses.
- Server [daemon][52], having a single [HTTP] endpoint, does all the parsing and executing of commands sent by the [CLI] client.




## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:
- [Что такое HTTP? Что подразумевает HTTP/2? Что подразумевает HTTP/3?][4201]
- How do work-stealing and thread-per-core paradigms affect programming a web server in practice? Which one is better and when? When does this question (choosing) become meaningful, in practice?
- [Как парадигмы «перераспределения задач» и «поток на ядро» влияют на практическое программирование веб-сервера? Какая из них лучше и когда? Когда этот вопрос (выбор) становится актуальным на практике?][4202]
- What are common crates for making HTTP requests in [Rust]? Which trade-offs do they have?
- What is WebSocket? How is it used and when? How does it work, in a nutshell?

<hr>

<a name="q-4201"><h3>Что такое HTTP? Что подразумевает HTTP/2? Что подразумевает HTTP/3?</h3></a>

__HTTP (HyperText Transfer Protocol)__ — это прикладной протокол передачи данных, основа интернета. Работает по модели «клиент-сервер»: браузер отправляет запрос, сервер возвращает ответ (HTML, картинки, JSON).

Вот основные отличия современных версий:

#### 1. HTTP/1.1 (Классика, 1997)

- Текстовый формат: Запросы передаются в виде обычного текста.
- Одна очередь: Чтобы загрузить 10 картинок, нужно либо открывать 10 соединений, либо качать их по очереди (проблема "Head-of-line blocking" — если первая картинка «зависла», остальные ждут).

#### 2. HTTP/2 (Эволюция, 2015)

Главная цель — скорость загрузки сложных сайтов.

- Бинарный протокол: Данные кодируются в нули и единицы, что компьютер обрабатывает быстрее.
- Мультиплексирование: По одному TCP-соединению можно передавать десятки файлов одновременно. Больше никакой очереди в рамках одного соединения.
- Сжатие заголовков (HPACK): Убирает избыточность в служебной информации.
- Server Push: Сервер может отправить стилевой файл (CSS) еще до того, как браузер его попросит.

#### 3. HTTP/3 (Революция, 2022)

Главная цель — стабильность на плохих каналах (Wi-Fi, мобильный интернет).

- Отказ от TCP: Вместо него используется протокол QUIC (Quick UDP Internet Connections) (работает поверх UDP).
- Устранение блокировки на уровне пакетов: В HTTP/2, если потеряется один пакет данных, всё соединение TCP "замирает", пока пакет не дойдет. В HTTP/- 3 (QUIC) потеря одного пакета блокирует только его поток данных, остальные продолжают качаться.
- Быстрое переподключение (0-RTT) (Zero Round Trip Time Resumption (возобновление сеанса с нулевым временем ожидания)): Если вы перешли с Wi-Fi на LTE, соединение не разрывается, а мгновенно продолжается благодаря ID сессии, а не IP-адресу.
- Шифрование по умолчанию: TLS 1.3 уже встроен в сам протокол QUIC, его нельзя отключить.

__Итог:__ HTTP/2 ускорил загрузку за счет параллельности, а HTTP/3 сделал эту параллельность неубиваемой даже при плохой связи.

<hr>

<a name="q-4202"><h3>Как парадигмы «перераспределения задач» и «поток на ядро» влияют на практическое программирование веб-сервера? Какая из них лучше и когда? Когда этот вопрос (выбор) становится актуальным на практике?</h3></a>

Эти парадигмы определяют, как ваш сервер распоряжается ресурсами процессора. В Rust выбор обычно стоит между __Tokio__ (Work Stealing) и __Glommio/Monoio__ (Thread-per-core).

#### 1. Перераспределение задач (Work Stealing)

Используется в Tokio (стандарт индустрии).

- Как работает: Есть общий пул потоков (worker threads). Если один поток разгреб свои задачи, он «ворует» задачи из очереди другого, чтобы никто не простаивал.
- Влияние на код: Вы пишете обычный ___async/await___. Данные между потоками должны быть ___Send + Sync___. Вам часто нужны ___Arc<Mutex<T>>___ или ___Arc<RwLock<T>>___, так как задача может начаться на одном ядре, а продолжиться на другом.
- Плюсы: Отличная утилизация CPU «из коробки». Вам не нужно думать о балансировке нагрузки вручную.

#### 2. Поток на ядро (Thread-per-core / Share-nothing)

Используется в Glommio, Monoio или кастомных решениях на io_uring.

- Как работает: Каждое ядро CPU имеет свой изолированный исполнитель (executor). Задачи жестко привязаны к ядру и никогда не переезжают.
- Влияние на код: Можно использовать !Send типы (например, Rc вместо Arc). Минимум блокировок (Mutex почти не нужен), так как данные живут только на одном ядре. Однако взаимодействие между ядрами становится сложным — нужно явно пересылать сообщения через каналы.
- Плюсы: Экстремально низкие задержки (latency) и отсутствие накладных расходов на синхронизацию кэшей процессора.


#### Что лучше и когда?

|Характеристика|Work Stealing (Tokio)|Thread-per-core (Glommio)|
|--------------|---------------------|-------------------------|
|Для чего|Обычные API, микросервисы, бизнес-логика.|	Высокопроизводительные БД, прокси, хранилища.|
|Сложность|Низкая (стандарт Rust).|Высокая (специфичное API, io_uring).|
|Масштабирование|Автоматическое.|Требует ручной балансировки входящих соединений.|
|Эффективность|Средняя (есть накладные расходы на lock-free очереди).|	Максимальная (близко к железу).|

#### Когда выбор становится актуальным?

На практике вопрос встает, когда вы упираетесь в "ceiling" (потолок) производительности:

1. Задержки (P99 Latency): Если вам нужно отвечать за микросекунды, переключение контекста и синхронизация в Work Stealing могут стать узким местом.
2. Работа с диском: Если сервер активно пишет/читает (БД), Thread-per-core в связке с io_uring даст колоссальный прирост.
3. Огромное количество ядер: На системах с 64+ ядрами конкуренция за общие очереди в Tokio может начать снижать эффективность.

__Итог:__ В 95% случаев для веб-сервера на Rust выбирают Work Stealing (Tokio), так как это проще и надежнее. К Thread-per-core переходят только при создании системных компонентов (баз данных или очень нагруженных сетевых шлюзов).

#### Особенности работы Tokio 

1. Как Tokio распределяет нагрузку

Когда вы используете аннотацию __#[tokio::main]__, по умолчанию запускается планировщик типа Work-Stealing:

   - Количество потоков: Tokio создает столько системных потоков (Worker Threads), сколько у вас логических ядер CPU (через num_cpus).
   - Привязка к ядрам (Affinity): По умолчанию Tokio не привязывает (не "прибивает" гвоздями) поток к конкретному номеру ядра. Это оставляет операционной системе (OS) право решать, на каком физическом ядре выполнять поток в данный момент.

2. Механизм Work-Stealing

Распределение задач происходит динамично:

   - У каждого потока Tokio есть своя локальная очередь задач.
   - Если один поток освободился, а у другого в очереди еще 100 задач, свободный поток «крадет» (steals) часть задач у соседа.
   - Это гарантирует, что все ядра будут загружены равномерно, даже если некоторые задачи выполняются дольше других.

3. Можно ли «прибить» поток к ядру?

Сам Tokio из коробки не делает жесткую привязку (CPU Affinity), так как это может мешать работе других программ в системе. Однако, если вам нужна экстремальная производительность, вы можете сделать это вручную:

   - Использовать крейт tokio-util или core_affinity.
   - Настроить on_thread_start при создании рантайма через Builder:

```rust
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .on_thread_start(|| {
        // Здесь можно вызвать код для привязки текущего потока к ядру
        println!("Поток запущен на ядре...");
    })
    .build()
    .unwrap();
```


Atomic Types in rust
March 8, 2026

Atomic types in Rust are low-level primitives found in the std::sync::atomic module that provide thread-safe, lock-free access to shared memory. Unlike standard types, they allow multiple threads to read and write to the same memory location without data races. 
Rust Documentation
Rust Documentation
 +3
Core Atomic Types
Rust provides atomic versions of several primitive types: 
Booleans: AtomicBool for flags.
Integers: Signed and unsigned variants including AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, and AtomicUsize.
Pointers: AtomicPtr for thread-safe raw pointer operations. 
Rust Documentation
Rust Documentation
 +2
Key Characteristics
Interior Mutability: You can modify an atomic value through a shared reference (&T), which is essential for sharing data between threads.
Memory Ordering: Every atomic operation requires an Ordering argument (e.g., Relaxed, Acquire, Release, SeqCst) to define how the CPU and compiler should synchronize memory access.
Static Initialization: Atomics can be stored in static variables and initialized at compile-time using const constructors.
Thread Safety: They implement the Sync trait, meaning they are safe to share across thread boundaries. 
Rust Documentation
Rust Documentation
 +5
Common Operations
load/store: Atomically reading or writing the value.
fetch_add/fetch_sub: Atomically incrementing or decrementing integers, often used for counters.
swap: Replacing the current value and returning the old one.
compare_exchange: Only updating the value if it matches an expected "current" value, forming the basis for many lock-free algorithms. 
Reddit
Reddit
 +6
Would you like to see a code example of how to use AtomicUsize for a thread-safe counter?





undefined
undefined
undefined
16 sites
std::sync::atomic - Rust
Module atomic Copy item path. ... Atomic types provide primitive shared-memory communication between threads, and are the building...

Rust Documentation
Crust of Rust: Atomics and Memory Ordering
The video introduces atomics and memory ordering in Rust, explaining that these concepts are frequently requested topics. This top...

YouTube
·
Jon Gjengset

9m
Fearless Concurrency in Rust: A Practical Guide to Atomic Types
What Are Atomic Types in Rust? Atomic types are low-level primitives that allow safe, lock-free operations on shared data. Rust pr...

Dev Genius

Show all
Что будет с транзакцией trans если возникнет ошибка и функция create_role завершится с ошибкой для базы MySql: /// Создать роль pub async fn create_role( db: &Database, slug: &str, name: &str, permissions: &str, ) ->Result<Role> { // сформировать роль по умалчанию let mut new_role = Role::default() ; new_role.set_slug(slug)? ; new_role.set_name(name)? ; new_role.set_permissions(permissions)? ; let mut trans = db .pool // Устанавливает соединение и немедленно начинает новую транзакцию. .begin() .await? ; // Проверка наличия создаваемой роли match sqlx::query_as::<_, Role>( r#" select * from roles where slug = ? "# ) .bind(new_role.slug()) .fetch_one(&mut *trans) .await { Ok(r) => { // данные по роли извлечены r.validate()? ; // проверка роли Ok(r) // возврат найденной роли }, Err(sqlx::Error::RowNotFound) => { // роль не найдена // создание роли sqlx::query( r#" insert into roles (slug, name, permissions) values (?, ?, ?) "# ) .bind(new_role.slug()) .bind(new_role.name()) .bind(new_role.permissions()) .execute(&mut *trans) .await? ; // получить данные по роли new_role = sqlx::query_as::<_, Role>( r#" select * from roles where slug = ? "# ) .bind(new_role.slug()) .fetch_one(&mut *trans) .await? ; new_role .validate()? ; trans .commit() .await? ; Ok(new_role) }, Err(err) => Err(err.into()), // ошибка запроса } }
March 8, 2026

В случае возникновения ошибки в MySQL при использовании библиотеки sqlx, судьба транзакции trans определяется механизмом RAII (Resource Acquisition Is Initialization):
1. Автоматический Rollback
Если функция завершается до вызова trans.commit().await? (например, через оператор ? или при возврате Err), объект trans выходит из области видимости и уничтожается (dropped).
В sqlx деструктор транзакции автоматически отправляет команду ROLLBACK в базу данных, если транзакция не была явно зафиксирована. Это гарантирует, что незавершенные изменения не сохранятся.
2. Состояние в MySQL
Поскольку вы используете &mut *trans для каждого запроса, любая ошибка внутри блока (например, ошибка уникальности slug при вставке или ошибка валидации) приведет к следующему:
Если ошибка произошла в execute или fetch_one, оператор ? прервет выполнение функции.
Соединение вернется в пул, но перед этим драйвер отправит сигнал отмены транзакции.
Результат: Все действия внутри этой транзакции будут отменены, база останется в исходном состоянии.
3. Нюанс с логикой fetch_one
В вашем коде есть ветка Ok(r), где роль найдена. В этой ветке вы возвращаете Ok(r), но не вызываете trans.commit().
В данном конкретном случае (только SELECT) это не критично, так как изменений нет.
Однако технически транзакция все равно будет "откачена" при выходе из функции. Для чистоты кода, если вы открыли транзакцию, ее лучше завершать явно, даже если вы только читали данные, либо открывать транзакцию после того, как убедились, что роль нужно создавать.
Рекомендация
Для MySQL лучше сначала проверить существование роли обычным запросом через db.pool, и только если её нет — открывать транзакцию для вставки. Это сэкономит ресурсы пула соединений.
Хотите разобрать, как оптимизировать этот код, чтобы избежать лишнего открытия транзакции при обычном чтении?




да
March 8, 2026

Чтобы оптимизировать этот код, лучше всего использовать подход "проверь, затем действуй" вне транзакции, либо использовать специфичную для MySQL конструкцию INSERT IGNORE или ON DUPLICATE KEY UPDATE.
Вот наиболее чистый и эффективный вариант на Rust с использованием sqlx:
Оптимизированный вариант
Мы выносим первичный поиск за пределы транзакции. Это освобождает соединение и не блокирует ресурсы базы данных, если роль уже существует.
rust
pub async fn create_role(
    db: &Database,
    slug: &str,
    name: &str,
    permissions: &str,
) -> Result<Role> {
    let mut new_role = Role::default();
    new_role.set_slug(slug)?;
    new_role.set_name(name)?;
    new_role.set_permissions(permissions)?;

    // 1. Пытаемся найти роль без открытия транзакции
    let existing = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE slug = ?"
    )
    .bind(new_role.slug())
    .fetch_optional(&db.pool) // Используем fetch_optional вместо match
    .await?;

    if let Some(role) = existing {
        role.validate()?;
        return Ok(role);
    }

    // 2. Если роли нет, открываем транзакцию для создания
    let mut trans = db.pool.begin().await?;

    // Используем INSERT IGNORE или просто INSERT, так как мы уже проверили существование
    // Но в многопоточной среде лучше быть готовым к ошибке дубликата
    sqlx::query(
        "INSERT INTO roles (slug, name, permissions) VALUES (?, ?, ?)"
    )
    .bind(new_role.slug())
    .bind(new_role.name())
    .bind(new_role.permissions())
    .execute(&mut *trans)
    .await?;

    // Получаем созданную роль (в MySQL можно использовать LAST_INSERT_ID, 
    // но по slug надежнее, если это первичный ключ)
    let created_role = sqlx::query_as::<_, Role>(
        "SELECT * FROM roles WHERE slug = ?"
    )
    .bind(new_role.slug())
    .fetch_one(&mut *trans)
    .await?;

    created_role.validate()?;
    
    // Явно фиксируем изменения
    trans.commit().await?;

    Ok(created_role)
}
Use code with caution.

Что изменилось и почему это лучше:
fetch_optional: Вместо сложного match с обработкой RowNotFound, этот метод возвращает Option<Role>. Это делает код короче и понятнее.
Экономия ресурсов: Транзакция (begin()) вызывается только тогда, когда запись в базу данных действительно необходима. Если роль уже есть, мы вообще не тратим ресурсы на управление транзакцией.
Безопасность: Если между первым SELECT и INSERT другой поток успеет вставить роль с таким же slug, MySQL вернет ошибку уникальности (Unique Constraint violation), оператор ? поймает её, и транзакция корректно откатится.
Еще более быстрый путь (MySQL-way):
Если вам не нужно выполнять сложную логику валидации в Rust перед вставкой, можно использовать INSERT ... ON DUPLICATE KEY UPDATE или INSERT IGNORE, а затем сразу запрашивать данные. Это сократит количество сетевых запросов к БД.
Хотите рассмотреть вариант с одним SQL-запросом, который делает всё сразу на стороне MySQL?




Что такое HTTP? Что подразумевает HTTP/2? Что подразумевает HTTP/3?
HTTP (HyperText Transfer Protocol) — это прикладной протокол передачи данных, основа интернета. Работает по модели «клиент-сервер»: браузер отправляет запрос, сервер возвращает ответ (HTML, картинки, JSON).
Вот основные отличия современных версий:
1. HTTP/1.1 (Классика, 1997)
Текстовый формат: Запросы передаются в виде обычного текста.
Одна очередь: Чтобы загрузить 10 картинок, нужно либо открывать 10 соединений, либо качать их по очереди (проблема "Head-of-line blocking" — если первая картинка «зависла», остальные ждут).
2. HTTP/2 (Эволюция, 2015)
Главная цель — скорость загрузки сложных сайтов.
Бинарный протокол: Данные кодируются в нули и единицы, что компьютер обрабатывает быстрее.
Мультиплексирование: По одному TCP-соединению можно передавать десятки файлов одновременно. Больше никакой очереди в рамках одного соединения.
Сжатие заголовков (HPACK): Убирает избыточность в служебной информации.
Server Push: Сервер может отправить стилевой файл (CSS) еще до того, как браузер его попросит.
3. HTTP/3 (Революция, 2022)
Главная цель — стабильность на плохих каналах (Wi-Fi, мобильный интернет).
Отказ от TCP: Вместо него используется протокол QUIC (работает поверх UDP).
Устранение блокировки на уровне пакетов: В HTTP/2, если потеряется один пакет данных, всё соединение TCP "замирает", пока пакет не дойдет. В HTTP/3 (QUIC) потеря одного пакета блокирует только его поток данных, остальные продолжают качаться.
Быстрое переподключение (0-RTT): Если вы перешли с Wi-Fi на LTE, соединение не разрывается, а мгновенно продолжается благодаря ID сессии, а не IP-адресу.
Шифрование по умолчанию: TLS 1.3 уже встроен в сам протокол QUIC, его нельзя отключить.
Итог: HTTP/2 ускорил загрузку за счет параллельности, а HTTP/3 сделал эту параллельность неубиваемой даже при плохой связи.
Хотите узнать, как настроить поддержку HTTP/3 на сервере Nginx или в приложении на Rust?




расшифровка QUIC
QUIC stands for Quick UDP Internet Connections.
It was originally developed by Google in 2012 and later standardized by the IETF. The name reflects its core mission: making internet connections faster by using UDP instead of the traditional, slower TCP.
Key technical "decoding":
Quick: It reduces the time needed to establish a secure connection (handshake). While TCP + TLS takes 2–3 round trips, QUIC can often do it in zero or one (0-RTT).
UDP: Unlike HTTP/1.1 and HTTP/2 which rely on TCP, QUIC sits on top of UDP. This allows it to handle packet loss much more gracefully without freezing the entire data stream.
Internet Connections: It acts as a transport layer that bundles transport (like TCP) and encryption (like TLS 1.3) into one single, tight package.
In short, QUIC is the "engine" that powers HTTP/3.
Do you want to see how QUIC handles a switch from Wi-Fi to 5G without dropping your active downloads?




Расшифровка 0-RTT
0-RTT расшифровывается как Zero Round Trip Time Resumption (возобновление сеанса с нулевым временем ожидания).
В обычном интернете (TCP + TLS), прежде чем отправить первый байт данных (например, запрос GET /), браузер и сервер должны «пожать друг другу руки»: обменяться пакетами для установки соединения и ключами шифрования. Это занимает 1–3 полных цикла (Round Trips) ожидания.
В чем суть 0-RTT:
Если браузер уже подключался к этому серверу ранее, он сохраняет секретный «билет» (session ticket). При повторном заходе:
Браузер отправляет запрос данных сразу же, в самом первом пакете вместе с этим билетом.
Сервер расшифровывает запрос и тут же присылает ответ.
Результат: Задержка на установку связи равна 0, загрузка страницы начинается мгновенно.
Главный минус:
Уязвимость к атакам повтора (Replay Attacks): Злоумышленник может перехватить этот первый пакет с данными и отправить его серверу еще раз. Если это был запрос на «купить товар» или «перевести деньги», сервер может выполнить его дважды.
Решение: 0-RTT обычно разрешают только для «безопасных» запросов типа GET, которые не меняют состояние данных на сервере.
Хотите узнать, как включить поддержку 0-RTT в конфигурации вашего веб-сервера (например, Cloudflare или Nginx)?




Как парадигмы «перераспределения задач» и «поток на ядро» влияют на практическое программирование веб-сервера на rust? Какая из них лучше и когда? Когда этот вопрос (выбор) становится актуальным на практике?
Эти парадигмы определяют, как ваш сервер распоряжается ресурсами процессора. В Rust выбор обычно стоит между Tokio (Work Stealing) и Glommio/Monoio (Thread-per-core).
1. Перераспределение задач (Work Stealing)
Используется в Tokio (стандарт индустрии).
Как работает: Есть общий пул потоков (worker threads). Если один поток разгреб свои задачи, он «ворует» задачи из очереди другого, чтобы никто не простаивал.
Влияние на код: Вы пишете обычный async/await. Данные между потоками должны быть Send + Sync. Вам часто нужны Arc<Mutex<T>> или Arc<RwLock<T>>, так как задача может начаться на одном ядре, а продолжиться на другом.
Плюсы: Отличная утилизация CPU «из коробки». Вам не нужно думать о балансировке нагрузки вручную.
2. Поток на ядро (Thread-per-core / Share-nothing)
Используется в Glommio, Monoio или кастомных решениях на io_uring.
Как работает: Каждое ядро CPU имеет свой изолированный исполнитель (executor). Задачи жестко привязаны к ядру и никогда не переезжают.
Влияние на код: Можно использовать !Send типы (например, Rc вместо Arc). Минимум блокировок (Mutex почти не нужен), так как данные живут только на одном ядре. Однако взаимодействие между ядрами становится сложным — нужно явно пересылать сообщения через каналы.
Плюсы: Экстремально низкие задержки (latency) и отсутствие накладных расходов на синхронизацию кэшей процессора.
Что лучше и когда?
Характеристика	Work Stealing (Tokio)	Thread-per-core (Glommio)
Для чего	Обычные API, микросервисы, бизнес-логика.	Высокопроизводительные БД, прокси, хранилища.
Сложность	Низкая (стандарт Rust).	Высокая (специфичное API, io_uring).
Масштабирование	Автоматическое.	Требует ручной балансировки входящих соединений.
Эффективность	Средняя (есть накладные расходы на lock-free очереди).	Максимальная (близко к железу).
Когда выбор становится актуальным?
На практике вопрос встает, когда вы упираетесь в "ceiling" (потолок) производительности:
Задержки (P99 Latency): Если вам нужно отвечать за микросекунды, переключение контекста и синхронизация в Work Stealing могут стать узким местом.
Работа с диском: Если сервер активно пишет/читает (БД), Thread-per-core в связке с io_uring даст колоссальный прирост.
Огромное количество ядер: На системах с 64+ ядрами конкуренция за общие очереди в Tokio может начать снижать эффективность.
Итог: В 95% случаев для веб-сервера на Rust выбирают Work Stealing (Tokio), так как это проще и надежнее. К Thread-per-core переходят только при создании системных компонентов (баз данных или очень нагруженных сетевых шлюзов).
Хотите сравнить производительность конкретных фреймворков (например, Axum на Tokio против решений на io_uring)?




Крейт Tokio автоматически распределяет потоко по ядрам CPU?
Короткий ответ: Да, но с важной оговоркой.
Tokio делает это не напрямую через магию процессора, а через многопоточный планировщик (Multi-threaded Scheduler). Вот как это работает на практике:
1. Как Tokio распределяет нагрузку
Когда вы используете аннотацию #[tokio::main], по умолчанию запускается планировщик типа Work-Stealing:
Количество потоков: Tokio создает столько системных потоков (Worker Threads), сколько у вас логических ядер CPU (через num_cpus).
Привязка к ядрам (Affinity): По умолчанию Tokio не привязывает (не "прибивает" гвоздями) поток к конкретному номеру ядра. Это оставляет операционной системе (OS) право решать, на каком физическом ядре выполнять поток в данный момент.
2. Механизм Work-Stealing
Распределение задач происходит динамично:
У каждого потока Tokio есть своя локальная очередь задач.
Если один поток освободился, а у другого в очереди еще 100 задач, свободный поток «крадет» (steals) часть задач у соседа.
Это гарантирует, что все ядра будут загружены равномерно, даже если некоторые задачи выполняются дольше других.
3. Можно ли «прибить» поток к ядру?
Сам Tokio из коробки не делает жесткую привязку (CPU Affinity), так как это может мешать работе других программ в системе. Однако, если вам нужна экстремальная производительность, вы можете сделать это вручную:
Использовать крейт tokio-util или core_affinity.
Настроить on_thread_start при создании рантайма через Builder:

```rust
rust
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .on_thread_start(|| {
        // Здесь можно вызвать код для привязки текущего потока к ядру
        println!("Поток запущен на ядре...");
    })
    .build()
    .unwrap();
```

#### Когда Tokio работает в ОДНОМ потоке?

Если вы укажете __#[tokio::main(flavor = "current_thread")]__, никакого распределения по ядрам не будет. Весь код (и все async задачи) будет крутиться строго в одном системном потоке. Это полезно для экономии ресурсов в маленьких контейнерах или там, где не нужна параллельность.

__Итог:__ Tokio автоматически создает пул потоков по числу ядер и эффективно перекидывает задачи между ними, но финальное решение, на каком именно физическом ядре сейчас работает поток, обычно остается за ОС.

<hr>

[`actix`]: https://docs.rs/actix
[`actix-http`]: https://docs.rs/actix-http
[`actix-rt`]: https://docs.rs/actix-rt
[`actix-web`]: https://docs.rs/actix-web
[`actix-web-actors::ws`]: https://docs.rs/actix-web-actors/latest/actix_web_actors/ws/index.html
[`async-h1`]: https://docs.rs/async-h1
[`async-std`]: https://docs.rs/async-std
[`async-tungstenite`]: https://docs.rs/crate/async-tungstenite
[`awc`]: https://docs.rs/awc
[`axum`]: https://docs.rs/axum
[`Filter`]: https://docs.rs/warp/latest/warp/trait.Filter.html
[`Future`]: https://doc.rust-lang.org/stable/std/future/trait.Future.html
[`hyper`]: https://docs.rs/hyper
[`isahc`]: https://docs.rs/isahc
[`poem`]: https://docs.rs/poem
[`reqwest`]: https://docs.rs/reqwest
[`rocket`]: https://docs.rs/rocket
[`salvo`]: https://docs.rs/salvo
[`surf`]: https://docs.rs/surf
[`tower`]: https://docs.rs/tower
[`tower-http`]: https://docs.rs/tower-http
[`tungstenite`]: https://docs.rs/crate/tungstenite
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`tide`]: https://docs.rs/tide
[`tokio`]: https://docs.rs/tokio
[`tokio-tungstenite`]: https://docs.rs/crate/tokio-tungstenite
[`ureq`]: https://docs.rs/ureq
[`warp`]: https://docs.rs/warp
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[cURL]: https://en.wikipedia.org/wiki/CURL
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[HTTP/3]: https://en.wikipedia.org/wiki/HTTP/3
[i18n]: https://en.wikipedia.org/wiki/Internationalization_and_localization
[Rust]: https://www.rust-lang.org
[WASM]: https://en.wikipedia.org/wiki/WebAssembly
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[1]: https://github.com/rust-unofficial/awesome-rust#web-programming
[2]: https://www.arewewebyet.org/topics/frameworks
[3]: https://www.arewewebyet.org/topics/http-clients
[4]: https://www.arewewebyet.org/topics/lower-web-stack
[5]: ../../3_ecosystem/3_11_async
[21]: https://en.wikipedia.org/wiki/Web_framework
[22]: https://en.wikipedia.org/wiki/Middleware
[23]: https://en.wikipedia.org/wiki/Work_stealing
[24]: https://www.datadoghq.com/blog/engineering/introducing-glommio
[25]: https://www.techempower.com/benchmarks#hw=ph&test=plaintext&section=data-r18
[26]: https://hyper.rs/guides/server/hello-world
[31]: https://hyper.rs/guides/client/basic
[32]: https://developer.mozilla.org/docs/Web/API/Fetch_API
[33]: https://www.shuttle.rs/blog/2023/09/13/web-scraping-rust-reqwest
[41]: https://en.wikipedia.org/wiki/Actor_model
[51]: https://en.wikipedia.org/wiki/Client%E2%80%93server_model
[52]: https://en.wikipedia.org/wiki/Daemon_(computing)
[53]: https://en.wikipedia.org/wiki/Thin_client

[4201]: #q-4201
[4202]: #q-4202