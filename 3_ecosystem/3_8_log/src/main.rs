/*
    Общие принципы работы логгера:
        1) В логере создаётся Writer(s)
        2) В логгере создаётся subscriber(s).
        3) tracing::event!(...) - излучает событие.
        4) Subscriber обрабатывает событие.

    Созданы:
        1) Глобальный логгер: app_log
        2) Локальный логгер с targer:  access_log.
        3) Локальный логгер без targer:  access_log2

*/

/// Глобальный логгер
pub mod app_log {
    // Примитив синхронизации, в который номинально можно записать данные 
    // только один раз.
    use std::sync::OnceLock;

    use tracing::{
            Level,
        };

    use tracing_subscriber::{
            fmt::{
                self, 
                MakeWriter, 
                time::FormatTime,
            }, 
            prelude::*, 
            util::SubscriberInitExt,
    };

    use chrono::{
            Utc,
            SecondsFormat
        };

    // Трейты, помошники и типовые определения для ядра I/O функциональности.
    use std::io;

    /// Глобальный логгер
    static LOGGER: OnceLock<()> = OnceLock::new();

    /// Кастомный форматтер времени с наносекундами по RFC 3339
    struct Rfc3339Nanos;

    // Реализация FormatTime для структуры Rfc3339Nanos
    impl FormatTime for Rfc3339Nanos {
        fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
            // Возвращает объект DateTime<Utc>, соответствующий текущей дате и 
            // времени в формате UTC.
            let now = Utc::now() ;

            write!(w, 
                   "{}",
                // Возвращает строку даты и времени, соответствующую RFC 3339 и 
                // ISO 8601, с субсекундами, отформатированными в соответствии с 
                // функцией SecondsFormat.
                now.to_rfc3339_opts(
                        SecondsFormat::Nanos, // формат времени в наносекундах
                        true    // использовать TZ UTC
                    )
                )
        }
    }

    /// перечесление возможных потоков вывода
    enum OutputTarget {
        Stdout(io::Stdout),
        Stderr(io::Stderr),
    }

    /// Структура для реализации низового вывода в тот или иной поток
    struct StdMixWriterImpl {
        target: Option<OutputTarget>,
    }

    /// реализация io::Write для структуры StdMixWriterImpl
    impl io::Write for StdMixWriterImpl {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            match &self.target {
                Some(OutputTarget::Stdout(w)) => 
                        w
                         // Блокирует этот обработчик к стандартному  выходному потоку,
                         // возвращая гаоантии пригодности к записи
                         .lock()
                         // Записывает буфер в этот write, возвращая
                         // количество записанных байтов.
                         .write(buf),
                Some(OutputTarget::Stderr(w)) => 
                        w
                         // Блокирует этот обработчик к стандартному  выходному потоку,
                         // возвращая гаоантии пригодности к записи
                         .lock()
                         // Записывает буфер в этот write, возвращая
                         // количество записанных байтов.
                         .write(buf),
                None => 
                    io::stdout()
                    // Блокирует этот обработчик к стандартному выходному потоку,
                    // возвращая гаоантии пригодности к записи
                    .lock()
                    // Записывает буфер в этот write, возвращая 
                    // количество записанных байтов.                 
                    .write(buf),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            match &self.target {
                // вывод в Stdout
                Some(OutputTarget::Stdout(w)) => 
                    w
                     // Блокирует этот обработчик к стандартному выходному потоку,
                     // возвращая гаоантии пригодности к записи
                     .lock()
                     // Очищает этот выходной поток, гарантируя, что все промежуточно 
                     // буферизованные данные достигнут места назначения.
                     .flush(),
                // вывод в Stderr
                Some(OutputTarget::Stderr(w)) => 
                    w
                     // Блокирует этот обработчик к стандартному выходному потоку,
                     // возвращая гаоантии пригодности к записи                     
                     .lock()
                     // Очищает этот выходной поток, гарантируя, что все промежуточно
                     // буферизованные данные достигнут места назначения.
                     .flush(),
                // для всего остального вывод в Stdout
                None => io::stdout()
                // Блокирует этот обработчик к стандартному выходному потоку,
                // возвращая гаоантии пригодности к записи
                .lock()
                // Очищает этот выходной поток, гарантируя, что все промежуточно 
                // буферизованные данные достигнут места назначения.                     
                .flush(),
            }    
        }
    }

    /// Кастомный writer, который направляет WARN и ERROR в stderr,
    /// остальное в stdout
    struct StdMixWriter;

    impl<'a> MakeWriter<'a> for StdMixWriter  {
        type Writer = StdMixWriterImpl;

        // Возвращает экземпляр класса Writer.
        fn make_writer(&'a self) -> Self::Writer {
            StdMixWriterImpl {
                target: None,
            }
        }

        // Возвращает объект Writer для записи данных из диапазона или события,
        // описанного предоставленными метаданными.
        fn make_writer_for(&'a self, meta: &tracing::Metadata<'_>) -> Self::Writer {
            // Возвращает уровень детализации описываемого фрагмента текста или 
            // события.
            let level = meta.level() ;

            /*
            Отсортированный список значений перечисления Level
            в порядке убывания важности
                [Level(Error), 
                Level(Warn),
                Level(Info),
                Level(Debug),
                Level(Trace)]
            */
            let target = match *level {
                Level::ERROR | Level::WARN => {
                        Some(OutputTarget::Stderr(io::stderr()))
                    },
                _ => Some(OutputTarget::Stdout(io::stdout())),
            };

            StdMixWriterImpl{target}
        }
    }


    /// Инициализация глобального логгера
    pub fn init_logger() {
    
        LOGGER
        // Получает содержимое ячейки, инициализируя его значением f(), 
        // если ячейка не была инициализирована.
          .get_or_init(
            || {
            // Создаем слой форматирования для app.log (основной логгер)
            let app_logger = 
              fmt::layer()
                // Задает объект MakeWriter, который будет использоваться создаваемым
                // слоем для записи событий.
                .with_writer(StdMixWriter)
                // Задает форматтер событий, который будет использоваться создаваемым
                // слоем для форматирования событий.
                .event_format(
                    // Возвращает конфигурацию по умолчанию для форматировщика событий.
                    fmt::format()
                        // Используйте полный формат JSON.
                        .json()
                        // Определяет, будет ли форматтер включать текущий элемент <span> 
                        // в форматируемые события.
                        .with_current_span(false)
                        // Определяет, будет ли форматтер включать список (от корня до листа)
                        // всех введенных в данный момент тегов <span> в форматируемые события.
                        .with_span_list(false)
                        // Определяет, отображается ли путь к файлу исходного кода события.
                        .with_file(false)
                        // Определяет, отображается ли номер строки исходного кода события.
                        .with_line_number(false)
                        // Определяет, отображается ли целевой объект события.
                        .with_target(false)
                        // Используйте указанный таймер для меток времени сообщений журнала.
                        .with_timer(Rfc3339Nanos)
                        // Используйте полный формат JSON, но с преобразованными в плоский 
                        // формат полями события.
                        // для выравнивания группы "fields"
                        .flatten_event(true)
                        //.format_event(ctx, writer, event) 
                )
                // Объединяет себя с фильтром, возвращая отфильтрованный слой.
                //.with_filter(EnvFilter::from_default_env())
                ;

            // Инициализируем подписчика
            // Создает реестр, который может объединять несколько слоев.
            tracing_subscriber::registry()
                // Оборачивает себя предоставленным слоем форматирования (Добавляет слой в реестр.).
                .with(app_logger)
                // Пытается установить self в качестве глобального подписчика по 
                // умолчанию в текущей области
                .init()
                ;
        });
    }


    /// Базовый макрос для логирования (без экспорта)
    macro_rules! log {
        ($level:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            {
                tracing::event!(
                    $level,
                    msg = $msg
                    $(, $field = $value )*
                );
            }
        };
    }

    // Экспортируем макрос log для использования внутри крейта по пути
    pub(crate) use log;

    /// Макрос для вывода ошибок
    #[macro_export]
    macro_rules! glb_log_error {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::app_log::log!(tracing::Level::ERROR, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода предупреждений
    #[macro_export]
    macro_rules! glb_log_warn {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::app_log::log!(tracing::Level::WARN, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода информации
    #[macro_export]
    macro_rules! glb_log_info {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::app_log::log!(tracing::Level::INFO, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода отдадочной информации
    #[macro_export]
    macro_rules! glb_log_debug {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::app_log::log!(tracing::Level::DEBUG, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода трассировочной информации
    #[macro_export]
    macro_rules! glb_log_trace {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::app_log::log!(tracing::Level::TRACE, $msg $(, $field => $value )*);
        };
    }
}


/// Локальный логгер с targer
pub mod access_log {

    use tracing_subscriber::{
                fmt::{
                    self,
                    time::FormatTime
                }, 
                layer::SubscriberExt, 
                Layer
            };

    use tracing_appender::non_blocking::WorkerGuard;

    use chrono::{
            Utc,
            SecondsFormat
        };

    /// Кастомный форматтер времени с наносекундами по RFC 3339
    struct Rfc3339Nanos;

    // Реализация FormatTime для структуры Rfc3339Nanos
    impl FormatTime for Rfc3339Nanos {
        fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
            // Возвращает объект DateTime<Utc>, соответствующий текущей дате и 
            // времени в формате UTC.
            let now = Utc::now() ;

            write!(w, 
                   "{}",
                // Возвращает строку даты и времени, соответствующую RFC 3339 и 
                // ISO 8601, с субсекундами, отформатированными в соответствии с 
                // функцией SecondsFormat.
                now.to_rfc3339_opts(
                        SecondsFormat::Nanos, // формат времени в наносекундах
                        true    // использовать TZ UTC
                    )
                )
        }
    }


    /// Структура локального логгера
    pub struct LocalLogger<'a> {
        target:     &'a str,
        path:       &'a str,
        guard:      Option::<WorkerGuard>,
    }

    // Реализация локального логгера
    impl<'a> LocalLogger<'a> {

        // Cоздание нового логгера
        pub fn new(target: &'a str, path: &'a str) -> Self {
            Self { 
                target: target,     // target.to_string(),
                path:   path,       // path.to_string(),
                guard:  Option::<WorkerGuard>::None,
            }
        }

        /// Инициализация нового логгер
        pub fn init(
                    &mut self,
                ) -> Result<tracing::subscriber::DefaultGuard, std::io::Error> {
            // Создаем файловый writer
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(
                    self.path
                )?;
        
            let (
                // (writer): Он реализует интерфейсы std::io::Write и 
                // tracing_subscriber::fmt::writer::MakeWriter, что позволяет 
                // использовать его с FmtSubscriber или Layer из tracing_subscriber.
                // При записи данные ставятся в очередь для рабочего потока.
                non_blocking,
                // (guard): Это крайне важно. Он гарантирует, что все буферизованные 
                // логи будут выведены в выходной поток при завершении программы.
                // guard должен оставаться активным до завершения программы; 
                // в противном случае буферизованные логи могут быть немедленно 
                // отброшены, и никакие записи в журнал не будут сделаны.
                guard
            ) = 
                // Создает неблокирующий поток для записи данных, запуская отдельный 
                // поток логирования для обработки операций ввода-вывода. 
                // Это предотвращает блокировку основных потоков приложения при 
                // записи логов, повышая производительность.        
                tracing_appender::non_blocking(file);

            // сохраняем guard в структуре до завершения работы программы
            // для нормального завершения записи всех логов         
            self.guard = Some(guard) ;

            // создаём строку на основе target
            let target_copy = self.target.to_string() ;


            // Создаем слой с фильтром по target
            let layer =
            // создаёт слой форматирования
            fmt::layer()
                // Используйте полный формат JSON.
                .json()
                // Определяет, отображается ли целевой объект события.
                .with_target(false)
                // Определяет, будет ли форматтер включать текущий элемент <span> 
                // в форматируемые события.                
                .with_current_span(false)
                // Определяет, будет ли форматтер включать список (от корня до листа)
                // всех введенных в данный момент тегов <span> в форматируемые события.                
                .with_span_list(false)
                // Определяет, отображается ли путь к файлу исходного кода события.
                .with_file(false)
                // Определяет, отображается ли номер строки исходного кода события.
                .with_line_number(false)
                // Используйте указанный таймер для меток времени сообщений журнала.
                .with_timer(Rfc3339Nanos)
                // Используйте полный формат JSON, но с преобразованными в плоский 
                // формат полями события.
                // для выравнивания группы "fields"                
                .flatten_event(true)
                // Задает объект MakeWriter, который будет использоваться создаваемым
                // слоем для записи событий.            
                .with_writer(non_blocking)
                // Условие фильтрации (по target)
                .with_filter(tracing_subscriber::filter::filter_fn(move |metadata| {
                    metadata.target() == &target_copy
                }));

            // полписчик событий
            let subscriber = 
                    // Функция создаёт подписчика добавлая слой layer к реестру.
                    // Она является основой, на которую добавляются различные 
                    // уровни функциональности трассировки
                    tracing_subscriber::registry()
                        // Оборачивает себя предоставленным слоем.
                        .with(layer)
                        ;

            /* Установка подписчика как глобального по умолчанию
            tracing::subscriber::set_global_default(subscriber) ;
            */

            // Функция tracing::subscriber::set_default в Rust устанавливает 
            // подписчика в качестве значения по умолчанию только для текущего
            // потока на время жизни возвращаемого DefaultGuard.
            // До конца scope (RAII)
            Ok(tracing::subscriber::set_default(subscriber))
        }
    }

    /// Базовый макрос для логирования (без экспорта)
    macro_rules! log {
        ($target:expr, $level:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            {
                tracing::event!(
                    target: $target,    // "target:" это имя параметра, устанавливаентся в меиаданных
                    $level,
                    msg = $msg
                    $(, $field = $value )*
                );
            }
        };
    }

    // Экспортируем макрос log для использования внутри крейта по пути
    pub(crate) use log;

    /// Макрос для вывода ошибок
    #[macro_export]
    macro_rules! loc_log_error {
        ($target:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log::log!($target, tracing::Level::ERROR, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода предупреждений
    #[macro_export]
    macro_rules! loc_log_warn {
        ($target:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log::log!($target, tracing::Level::WARN, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода информации
    #[macro_export]
    macro_rules! loc_log_info {
        ($target:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log::log!($target, tracing::Level::INFO, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода отдадочной информации
    #[macro_export]
    macro_rules! loc_log_debug {
        ($target:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log::log!($target, tracing::Level::DEBUG, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода трассировочной информации
    #[macro_export]
    macro_rules! loc_log_trace {
        ($target:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log::log!($target, tracing::Level::TRACE, $msg $(, $field => $value )*);
        };
    }    
}

/// Локальный логгер без targer
pub mod access_log2 {

    use tracing_subscriber::{
                fmt::{
                    self,
                    time::FormatTime
                }, 
                layer::SubscriberExt, 
                //Layer
            };

    use tracing_appender::non_blocking::WorkerGuard;

    use chrono::{
            Utc,
            SecondsFormat
        };

    /// Кастомный форматтер времени с наносекундами по RFC 3339
    struct Rfc3339Nanos;

    // Реализация FormatTime для структуры Rfc3339Nanos
    impl FormatTime for Rfc3339Nanos {
        fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
            // Возвращает объект DateTime<Utc>, соответствующий текущей дате и 
            // времени в формате UTC.
            let now = Utc::now() ;

            write!(w, 
                   "{}",
                // Возвращает строку даты и времени, соответствующую RFC 3339 и 
                // ISO 8601, с субсекундами, отформатированными в соответствии с 
                // функцией SecondsFormat.
                now.to_rfc3339_opts(
                        SecondsFormat::Nanos, // формат времени в наносекундах
                        true    // использовать TZ UTC
                    )
                )
        }
    }

    /// Структура локального логгера
    pub struct LocalLogger<'a> {
        path:       &'a str,
        guard:      Option::<WorkerGuard>,
    }

    // Реализация локального логгера
    impl<'a> LocalLogger<'a> {

        // Cоздание нового логгера
        pub fn new(path: &'a str) -> Self {
            Self { 
                path:   path,       // path.to_string(),
                guard:  Option::<WorkerGuard>::None,
            }
        }

        /// Инициализация нового логгер
        pub fn init(
                    &mut self,
                ) -> Result<tracing::subscriber::DefaultGuard, std::io::Error> {
            // Создаем файловый writer
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(
                    self.path
                )?;
        
            let (
                // (writer): Он реализует интерфейсы std::io::Write и 
                // tracing_subscriber::fmt::writer::MakeWriter, что позволяет 
                // использовать его с FmtSubscriber или Layer из tracing_subscriber.
                // При записи данные ставятся в очередь для рабочего потока.
                non_blocking,
                // (guard): Это крайне важно. Он гарантирует, что все буферизованные 
                // логи будут выведены в выходной поток при завершении программы.
                // guard должен оставаться активным до завершения программы; 
                // в противном случае буферизованные логи могут быть немедленно 
                // отброшены, и никакие записи в журнал не будут сделаны.
                guard
            ) = 
                // Создает неблокирующий поток для записи данных, запуская отдельный 
                // поток логирования для обработки операций ввода-вывода. 
                // Это предотвращает блокировку основных потоков приложения при 
                // записи логов, повышая производительность.        
                tracing_appender::non_blocking(file);

            // сохраняем guard в структуре до завершения работы программы
            // для нормального завершения записи всех логов         
            self.guard = Some(guard) ;

            // Создаем слой с фильтром по target
            let layer =
            // создаёт слой форматирования
            fmt::layer()
                // Используйте полный формат JSON.
                .json()
                // Определяет, отображается ли целевой объект события.
                .with_target(false)
                // Определяет, будет ли форматтер включать текущий элемент <span> 
                // в форматируемые события.                
                .with_current_span(false)
                // Определяет, будет ли форматтер включать список (от корня до листа)
                // всех введенных в данный момент тегов <span> в форматируемые события.                
                .with_span_list(false)
                // Определяет, отображается ли путь к файлу исходного кода события.                
                .with_file(false)
                // Определяет, отображается ли номер строки исходного кода события.
                .with_line_number(false)
                // Используйте указанный таймер для меток времени сообщений журнала.
                .with_timer(Rfc3339Nanos)
                // Используйте полный формат JSON, но с преобразованными в плоский 
                // формат полями события.
                // для выравнивания группы "fields"                
                .flatten_event(true)
                // Задает объект MakeWriter, который будет использоваться создаваемым
                // слоем для записи событий.
                .with_writer(non_blocking)
                /*
                .with_filter(tracing_subscriber::filter::filter_fn(move |metadata| {
                    metadata.target() == &target_copy
                }))
                 */
                ;

            // полписчик событий
            let subscriber = 
                    // Функция создаёт подписчика добавлая слой layer к реестру.
                    // Она является основой, на которую добавляются различные 
                    // уровни функциональности трассировки
                    tracing_subscriber::registry()
                        // Оборачивает себя предоставленным слоем.
                        .with(layer)
                        ;

            // Функция tracing::subscriber::set_default в Rust устанавливает 
            // подписчика в качестве значения по умолчанию только для текущего
            // потока на время жизни возвращаемого DefaultGuard.
            // До конца scope (RAII)
            Ok(tracing::subscriber::set_default(subscriber))
        }
    }

    /// Базовый макрос для логирования (без экспорта)
    macro_rules! log {
        ($level:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
            {
                tracing::event!(
                    $level,
                    msg = $msg
                    $(, $field = $value )*
                );
            }
        };
    }

    // Экспортируем макрос log для использования внутри крейта по пути
    pub(crate) use log;

    /// Макрос для вывода ошибок
    #[macro_export]
    macro_rules! loc_log2_error {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log2::log!(tracing::Level::ERROR, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода предупреждений
    #[macro_export]
    macro_rules! loc_log2_warn {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log2::log!(tracing::Level::WARN, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода информации
    #[macro_export]
    macro_rules! loc_log2_info {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log2::log!(tracing::Level::INFO, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода отдадочной информации
    #[macro_export]
    macro_rules! loc_log2_debug {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log2::log!(tracing::Level::DEBUG, $msg $(, $field => $value )*);
        };
    }

    /// Макрос для вывода трассировочной информации
    #[macro_export]
    macro_rules! loc_log2_trace {
        ($msg:expr $(, $field:expr => $value:expr )*) => {
            crate::access_log2::log!(tracing::Level::TRACE, $msg $(, $field => $value )*);
        };
    }    
}


fn main() {

    // импорт глобального логгера
    use app_log ;

    // инициализация глобального логгера
    app_log::init_logger();

    // вызовы глобального логгера с разными уровнями отслеживания
    glb_log_info!("http", "file" => "app.log", "method" => "POST", "path" => "/some") ;
    glb_log_error!("Error occurred") ;
    glb_log_warn!("Application started", "version" => "1.0.0") ;
    glb_log_trace!("Application started", "version" => "1.0.0") ;
    glb_log_debug!("Application started", "version" => "1.0.0") ;

    // Локальный логгер с target
    {
        // импорт локального логгера
        use access_log ;
    
        // импорт для обобщённого вызова событий
        use tracing::Level;
    

        // Создание нового локального логгера
        let mut logger = 
                    access_log::LocalLogger::new(
                                "access_log",
                                "./access.log"
                            );

        // Устанавливаем подписчика для текущей области видимости
        let _gd = logger.init().unwrap() ;

        let v = "a".to_string() ;

        // вызовы локального логгера с разными уровнями отслеживания
        tracing::event!(target: "access_log", Level::INFO, method = "DELETE", path = "/", ans = v);
        tracing::event!(target: "access_log", Level::INFO, method = "POST", path = "/");
        tracing::event!(target: "access_log", Level::INFO, "abc");

        let mess = "my mess".to_owned() ;
        let val = "access_log".to_owned() ;
        let num = 10 ;

        // вызовы локального логгера с разными уровнями отслеживания
        loc_log_info!("access_log", 
                       mess,
                       "path" => "/some",
                       "file" => val,
                       "file2" => num
                    ) ;
        loc_log_error!("access_log", "Error occurred") ;
        loc_log_warn!("access_log", "Application started", "version" => "1.0.0") ;
        loc_log_trace!("access_log", "Application started", "version" => "1.0.0") ;
        loc_log_debug!("access_log", "Application started", "version" => "1.0.0") ;
    }

    // этот код не работает, т.к. он вне зоны видимости DefaultGuard
    loc_log_debug!("access_log", "NewApplication started", "version" => "1.0.0") ;

    // Локальный логгер без target
    {
        // импорт локального логгера
        use access_log2 ;
    
        // импорт для обобщённого вызова событий
        use tracing::Level;
    
        // Создание нового локального логгера
        let mut logger = 
                    access_log2::LocalLogger::new(
                                "./access2.log"
                            );

        // Устанавливаем подписчика для текущей области видимости
        let _gd = logger.init().unwrap() ;

        let v = "a".to_string() ;

        // вызовы локального логгера с разными уровнями отслеживания
        tracing::event!(target: "access_log", Level::INFO, method = "DELETE", path = "/", ans = v);
        tracing::event!(target: "access_log", Level::INFO, method = "POST", path = "/");
        tracing::event!(target: "access_log", Level::INFO, "abc");

        let mess = "my mess".to_owned() ;
        let val = "access_log".to_owned() ;
        let num = 10 ;

        // вызовы локального логгера с разными уровнями отслеживания
        loc_log2_info!(mess,
                       "path" => "/some",
                       "file" => val,
                       "file2" => num
                    ) ;
        loc_log2_error!("Error occurred") ;
        loc_log2_warn!("Application started", "version" => "1.0.0") ;
        loc_log2_trace!("Application started", "version" => "1.0.0") ;
        loc_log2_debug!("Application started", "version" => "1.0.0") ;
    }

    glb_log_debug!("Process started", "version" => "2.0.0") ;
}