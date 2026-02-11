mod logger {
// Примитив синхронизации, в который номинально можно записать данные 
// только один раз.
use std::sync::OnceLock;

use tracing::{
        Level,
        Subscriber
    };

use tracing_subscriber::{
        fmt::{
            self,
            format::JsonFields,
            // Тип, позволяющий создавать экземпляры io::Write.
            MakeWriter,
            // Тип данных, способный измерять и форматировать текущее время.
            time::FormatTime
        },
        prelude::{
            // для .with_filter(filter)
            *,
            //__tracing_subscriber_SubscriberExt,
        },
        registry::LookupSpan,
        util::SubscriberInitExt,
        EnvFilter,
};

use chrono::{
        DateTime, 
        Utc,
        SecondsFormat
    };

// Трейты, помошники и типовые определения для ядра I/O функциональности.
use std::io;


/// Глобальный логгер
static LOGGER: OnceLock<()> = OnceLock::new();

/// Кастомный форматтер времени с наносекундами по RFC 3339
#[derive(Debug, Clone)]
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

/*
struct StdioWriterImpl {
    target: Option<io::Stdout>,
}
 */
/// Структура для реализации низового вывода в тот или иной поток
struct StdioWriterImpl {
    target: Option<OutputTarget>,
}

/// реализация io::Write для структуры StdioWriterImpl
impl io::Write for StdioWriterImpl {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &self.target {
            /*
            Some(w) => 
                w
                 // Блокирует этот обработчик к стандартному  выходному потоку,
                 // возвращая гаоантии пригодности к записи
                 .lock()
                 // Записывает буфер в этот write, возвращая 
                 // количество записанных байтов.
                 .write(buf),
             */
            Some(OutputTarget::Stdout(w)) => w.lock().write(buf),
            Some(OutputTarget::Stderr(w)) => w.lock().write(buf),
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
            /*
            Some(w) => 

               w
               // Блокирует этот обработчик к стандартному выходному потоку,
               // возвращая гаоантии пригодности к записи
                .lock()
               // Очищает этот выходной поток, гарантируя, что все промежуточно 
               // буферизованные данные достигнут места назначения.
                .flush(),
                */
            // вывод в Stdout
            Some(OutputTarget::Stdout(w)) => w.lock().flush(),
            // вывод в Stderr
            Some(OutputTarget::Stderr(w)) => w.lock().flush(),
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
struct StdioWriter;

impl<'a> MakeWriter<'a> for StdioWriter  {
    type Writer = StdioWriterImpl;

    // Возвращает экземпляр класса Writer.
    fn make_writer(&'a self) -> Self::Writer {
        StdioWriterImpl {
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
        let target = if *level <= Level::INFO {
            Some(OutputTarget::Stdout(io::stdout()))
        } else {
            Some(OutputTarget::Stderr(io::stderr()))
        };
         */
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
            Level::ERROR | Level::WARN => Some(OutputTarget::Stderr(io::stderr())),
            _ => Some(OutputTarget::Stdout(io::stdout())),
        };

        StdioWriterImpl{target}
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
            .with_writer(StdioWriter)
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
                    .flatten_event(
                            //false
                            true    // для выравнивания группы "fields"
                        )
            )
            // Объединяет себя с фильтром, возвращая отфильтрованный слой.
            .with_filter(EnvFilter::from_default_env())
            ;

        // Инициализируем подписчика
        tracing_subscriber::registry()
            // Оборачивает себя предоставленным слоем форматирования.
            .with(app_logger)
            // Пытается установить self в качестве глобального подписчика по 
            // умолчанию в текущей области
            .init();
    });
}

/// Макрос для логирования с автоматическим определением файла лога
#[macro_export]
macro_rules! log {
    ($level:expr, $msg:expr $(, $field:expr => $value:expr )*) => {
        {
            //let file = $crate::logger::get_log_filename(module_path!());
            //println!("level: {:?}", $level) ;
            tracing::event!(
                //target: module_path!(),
                $level,
                lvl = %$level, 
                file = "app.log",
                msg = $msg
                $(, $field = $value )*
            );
        }
    };
}

/// Удобные макросы для разных уровней логирования
#[macro_export]
macro_rules! log_error {
    ($msg:expr $(, $field:expr => $value:expr )*) => {
        log!(tracing::Level::ERROR, $msg $(, $field => $value )*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr $(, $field:expr => $value:expr )*) => {
        log!(tracing::Level::WARN, $msg $(, $field => $value )*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr $(, $field:expr => $value:expr )*) => {
        log!(tracing::Level::INFO, $msg $(, $field => $value )*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr $(, $field:expr => $value:expr )*) => {
        log!(tracing::Level::DEBUG, $msg $(, $field => $value )*);
    };
}

#[macro_export]
macro_rules! log_trace {
    ($msg:expr $(, $field:expr => $value:expr )*) => {
        log!(tracing::Level::TRACE, $msg $(, $field => $value )*);
    };
}
}

fn main() {
    use logger ;

    /*
    let mut v = vec![
                logger::Level::DEBUG,
                Level::ERROR,
                Level::INFO,
                Level::TRACE,
                Level::WARN,
                            ] ;
        println!("v: {:?}", {v.sort() ;v}) ;
    */

    logger::init_logger();

    //log_info!("Application started", "version" => "1.0.0") ;
    log_error!("Application started", "version" => "1.0.0") ;
}
