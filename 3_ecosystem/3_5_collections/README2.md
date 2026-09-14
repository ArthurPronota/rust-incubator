## Использование итераторов для работы с коллекциями в RUST.

## Задание

Трейт `UsersRepository` с тремя операциями:

1. один `User` по id;
2. несколько `User` по списку id;
3. id пользователей, чей `nickname` содержит подстроку.

Реализация должна опираться на immutable-коллекцию (`im`), плюс тесты.

Теория в README шире кода: `std::collections`, ленивые итераторы, persistent structures (`im` / `rpds`), concurrent-коллекции (`crossbeam` и др.). В `main.rs` из этого взяты только `im::HashMap` и обычный `HashSet` для результата поиска.

## Модель

```rust
pub struct User {
    id: usize,
    nickname: String,
}

struct Users(HashMap<usize, User>); // im::HashMap
```

`User` — `Clone` + `Debug` + `PartialEq` (нужно и `im` для копирования узлов, и тестам). Репозиторий — newtype вокруг **`im::HashMap<usize, User>`**: ключ — id, значение — пользователь. Для lookup по id это естественный выбор (`O(log n)` у HAMT, не `O(1)` как у `std::HashMap`).

`new` собирает карту итератором: `into_iter().map(|u| (u.id, u)).collect()`. Дубликаты id перезапишутся последним — отдельной проверки нет.

## Методы трейта

**`get_user_by_id`** — `get_key_value`, при нахождении отдаёт `&User`. Эквивалент `self.0.get(&id)`.

**`get_users_by_ids`** — по срезу `&[usize]`: `iter().filter_map(|i| self.0.get(i)).collect()`. Пропущенные id тихо отбрасываются; порядок как во входном срезе.

**`get_users_by_nickname`** — полный обход карты:

- фраза один раз в `to_lowercase()`;
- `filter` по `nickname.to_lowercase().contains(...)`;
- `map` в `*id`;
- `collect` в **`std::collections::HashSet<usize>`**.

Поиск без индекса по никнейму: каждый раз `O(n)`. `HashSet` в сигнатуре — множество без порядка (для «какие id», не «в каком порядке»). Регистр не учитывается.

`im::HashMap` здесь в основном как **хранилище с семантикой persistent/HAMT**. Код его не клонирует и не ветвит версии (нет undo). Для учебного шага достаточно «backed by immutable collection».

## `main` и тесты

Демо на трёх пользователях: `user1`, `user2`, `user23`. Печатаются lookup по id `1`, список `[1, 2]`, поиск `"er2"` → id 2 и 3.

Тесты (`cargo test`):

| Тест | Что проверяет |
|---|---|
| `check_users_by_ids` | id 1 → `user1`; id 0 → `None` |
| `test_users_by_ids` | `[3, 2]` — длина и что все найденные id из запроса |
| `check_users_by_nickname` | `"er2"` даёт 2 id, никнеймы содержат фразу |

Нет тестов на отсутствующие id в пакетном запросе, пустую фразу, разный регистр (`User2` vs `er2`) и клонирование `im::HashMap`.

## Что шаг иллюстрирует

- Выбор коллекции под операцию: карта по id, `HashSet` для множества найденных id, слайс на входе пакетного запроса.
- Идиома итераторов: `filter_map` / `filter` / `map` / `collect` вместо ручных циклов.
- Persistent `HashMap` из `im` вместо `std`, даже если API репозитория только читает данные.
