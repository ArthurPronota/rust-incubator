## Глобальный и Локальный логгеры.

## Задание

1. **Глобальный** «app.log»: всё в **stdout**, уровни **WARN и выше** — в **stderr**.
2. **Локальный** `access.log`: всё в файл `access.log`.
3. Структурированный **JSON**, время **RFC 3339 с наносекундами**.

В коде фактически **три** варианта: глобальный `app_log` и два файловых (`access_log` с фильтром по `target`, `access_log2` без него). Есть черновики `main.rs.old*`. Тестов нет. В `Cargo.toml` ещё `once_cell` и `serde_json` — в текущем `main.rs` не используются (`OnceLock` из std).

## Общая схема tracing

Как в комментарии в начале файла:

1. Writer (куда писать)
2. Subscriber / Layer (как форматировать и фильтровать)
3. `tracing::event!` излучает событие
4. Subscriber обрабатывает его

Формат везде похож: `fmt::layer().json()`, без span/file/line/target в JSON, **`flatten_event(true)`** (поля события на верхнем уровне, не вложенный объект `fields`), таймер **`Rfc3339Nanos`** через `chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true)`.

Имена полей в реальном JSON — дефолты tracing: `timestamp`, `level`, `msg`/`message`, а не `time` / `lvl` из примера в README.

## 1. Глобальный `app_log`

Инициализация один раз: `OnceLock` + `tracing_subscriber::registry().with(layer).init()` — **глобальный** default subscriber.

Маршрутизация потоков — кастомный **`MakeWriter`**:

- `make_writer_for(metadata)` смотрит `meta.level()`
- `ERROR` | `WARN` → `stderr().lock()`
- остальное (`INFO`/`DEBUG`/`TRACE`) → `stdout()`

`StdMixWriterImpl` реализует `io::Write` и пишет в выбранный handle.

Макросы `glb_log_error!` … `glb_log_trace!` — обёртки над `tracing::event!` с полями `msg = ...` и произвольными `key => value`.

## 2. Локальный `access_log` (с `target`)

`LocalLogger { target, path, guard }`:

- файл `create + append`
- **`tracing_appender::non_blocking(file)`** — очередь и отдельный поток I/O; **`WorkerGuard`** хранится в структуре, иначе буфер при drop потеряется
- слой с **`filter_fn`**: в файл попадают только события, у которых `metadata.target() == "access_log"`
- subscriber ставится через **`set_default`**, не `set_global_default` → действует только в **текущем потоке**, пока жив `DefaultGuard` (`_gd`)

Поэтому вызов `loc_log_debug!(...)` **после** закрытия блока в `main` «не работает» — это специально показано в комментарии.

Макросы `loc_log_*!` передают `target:` в `event!`, чтобы пройти фильтр.

## 3. `access_log2` (без target)

Тот же файловый non-blocking writer (`access2.log`), но **без фильтра**. Любое событие в зоне `_gd` уходит в этот файл. Макросы `loc_log2_*!` без параметра target.

## `main`

1. `app_log::init_logger()` и несколько `glb_log_*` (консоль, JSON, WARN/ERROR → stderr).
2. Блок: `LocalLogger::new("access_log", "./access.log")`, события с `target: "access_log"` и макросы → дописывается `access.log`.
3. Событие вне блока — глобальный subscriber, не файл.
4. Блок `access_log2` → `access2.log`.
5. Снова `glb_log_debug!` в консоль.

`access.log` / `access2.log` в репозитории — накопленный вывод прошлых запусков.

## Что шаг показывает

- Фасад vs реализация: в приложении `tracing::event!`, бэкенд — subscriber.
- Структурированные поля (`method`, `path`, …), не только строка как у `println!`.
- `MakeWriter` для развода stdout/stderr по уровню.
- Потоковый файловый лог + `WorkerGuard`.
- Глобальный subscriber vs thread-local `set_default` + RAII.
- Фильтрация по `target` как способ нескольких «каналов» логов.

Отклонения от примера README: ключи `timestamp`/`level` вместо `time`/`lvl`; поле `file` не подставляется автоматически (его кладут руками, как `"file" => "app.log"`). `env-filter` в зависимостях объявлен, в коде закомментирован.
