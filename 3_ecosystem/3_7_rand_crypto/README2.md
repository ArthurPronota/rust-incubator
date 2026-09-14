## Случайность и криптография SHA-3 и Argon2

Теория README шире кода (`ring`, dalek, TLS, constant-time). В исходнике — генерация случайных строк, SHA-3 файла и Argon2 пароля.

## Задание

1. `generate_password` — пароль заданной длины из charset  
2. `select_rand_val` — случайный элемент среза  
3. `new_access_token` — 64 символа `a-zA-Z0-9`, криптостойко  
4. `get_file_hash` — SHA-3 файла по пути  
5. `hash_password` — Argon2 от пароля  

## Выбор RNG (главный компромисс шага)

В `rand 0.8`:

- **`thread_rng()`** — CSPRNG потока (ChaCha + энтропия ОС). Нужен для паролей и токенов.
- **`SmallRng`** — быстрый некриптографический PRNG (feature `small_rng`). Для игр, сэмплинга, не для секретов.

В коде оба варианта оставлены в комментариях. **Фактически:**

| Функция | Что используется | Соответствие задаче |
|---|---|---|
| `generate_password` | `SmallRng::from_entropy()` | для пароля лучше `thread_rng` |
| `select_rand_val` | новый `SmallRng::from_entropy()` на каждый вызов | для «просто случайный элемент» ок, но seed каждый раз избыточен |
| `new_access_token` | `thread_rng()` | как в формулировке «cryptographically secure» |

`from_entropy()` тянет `getrandom` (ОС). Дальше `SmallRng` уже предсказуем по состоянию — энтропия только на старте.

## Функции

**`generate_password(len, charset)`**  
Пустые длина/charset → `Err`. Charset → `Vec<char>` (Unicode, не байты). Цикл `0..len`, индекс `gen_range(0..len)`, сборка `String`.

**`select_rand_val<T>(slice)`**  
Пустой срез → ошибка, иначе `&T` по случайному индексу. Дженерик: числа, `char`, `&str`, кортежи — это видно в `main`.

**`new_access_token()`**  
Фиксированный ASCII-алфавит `&[u8]`, 64 раза `thread_rng().gen_range`. «Уникальность» статистическая, не UUID: коллизия маловероятна, отдельной проверки нет. На каждый символ снова берётся `thread_rng()` (дешевле, чем новый `SmallRng` каждый раз).

**`get_file_hash(path)`**  
Пустой путь / ошибка `open` → `Err`. `BufReader` + буфер 8 KiB, инкрементальный **`Sha3_256`**: `update` кусками, `finalize`, hex `{:x}`. Потоковое чтение — файл целиком в память не грузится. В `main` хешируется `Cargo.toml`. SHA-3 здесь как **сырой** хеш файла, не KDF для паролей (это отдельно сказано в README).

**`hash_password(password)`**  
Пустой пароль → ошибка. Соль: `SaltString::generate(&mut OsRng)` (криптостойко). **Argon2id** v0x13, параметры `19456` KiB памяти, 2 итерации, параллелизм 1, выход 32 байта. Результат — **PHC-строка** (`$argon2id$v=19$...`), её обычно кладут в БД. Обычный SHA для паролей не используют: быстро и без соли/стоимости.

## `main`

Демо всех пяти функций: пароль из букв/цифр/спецсимволов, выбор из срезов, токен, хеш `Cargo.toml`, Argon2 от `"password"`. Модуля `#[cfg(test)]` нет.

## Что шаг показывает

- CSPRNG vs быстрый PRNG и зачем не мешать их для секретов.
- Потоковый SHA-3 (`Digest`: `new` / `update` / `finalize`).
- Password hashing: соль + медленный KDF (Argon2id), не SHA.
- API `rand 0.8`: `Rng::gen_range`, `SeedableRng::from_entropy`.

Замечание по безопасности учебной реализации: пароли через `SmallRng` слабее требования «случайный пароль»; токен сделан правильно через `thread_rng`.
