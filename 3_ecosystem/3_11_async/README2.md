## Асинхронное чтение канала HTML на основе представленных URL.

## Задание

```bash
cargo run -p step_3_11 -- [--max-threads=<number>] <file>
```

Прочитать ссылки из `<file>`, скачать каждую в `.html` с именем от URL. `--max-threads` — максимум одновременных загрузок; по умолчанию `num_cpus::get()`. В задании сказано «потоки»; в коде это **число tokio-задач**, не пул OS-threads (Tokio сам мультиплексирует их).

Запуск из каталога шага: `cargo run list_links.txt`, справка: `cargo run -- -h`.

## Зависимости

| Крейт | Роль |
|---|---|
| `clap` derive | CLI |
| `num_cpus` | дефолт `--max-threads` |
| `tokio` full | `#[tokio::main]`, `spawn`, `tokio::fs::write` |
| `futures` | `StreamExt::buffer_unordered` |
| `reqwest` | async HTTP GET |

## CLI

`Args`: обязательный позиционный `file: PathBuf`; опция `--max-threads` (`usize`, default = число CPU).

## `main`

1. `Args::parse()`
2. Файл со ссылками должен существовать
3. `content_pages/` создаётся синхронно (`std::fs::create_dir_all`)
4. Файл читается целиком (`read_to_string`), строки: непустые, `trim`, только `http://` / `https://`
5. Пустой список URL → ошибка
6. `reqwest::Client` с timeout **30 с**
7. `download_pages(...).await`

Ошибки — `Box<dyn Error + Send + Sync>` (удобно в async между задачами).

## Имя файла

Из URL выкидываются `http://`/`https://`, прочие не-буквы/цифры кроме `.` и `-` заменяются на `_` (через `str::replace` с предикатом на `char`). Суффикс `.html`. Пример:  
`https://doc.rust-lang.org/edition-guide/rust-2024/index.html` →  
`content_pages/doc.rust-lang.org_edition-guide_rust-2024_index.html.html`.

## `download_page`

GET → проверка `status().is_success()` → `response.text().await` → `tokio::fs::write`. Успех: `(url, file_path)`.

## Параллелизм: `download_pages`

```text
stream::iter(urls)
  .map(|url| spawn(download_page(client.clone(), url)))
  .buffer_unordered(max_concurrents)
  .collect::<Vec<_>>()
  .await
```

- `Client` клонируется дёшево (внутри `Arc`).
- `spawn` — отдельная задача на URL.
- **`buffer_unordered(N)`** держит не больше N незавершённых загрузок; новые стартуют по мере завершения; порядок результатов ≠ порядок URL.
- Двойной `Result`: `JoinError` от spawn и ошибка загрузки. Успех — `println!`, сбой — `eprintln!`; общий `Result` функции всё равно `Ok(())`, если runtime не упал.

## Модель

Типичный **I/O-bound** async: пока одна загрузка ждёт сеть, рантайм качает другие. Это не Rayon/потоки из шага 3.10. Флаг называется `max-threads`, но ограничивает **concurrency задач**, а не `std::thread`.

Чтение списка URL и создание каталога синхронные; сеть и запись HTML — async. Тестов нет.
