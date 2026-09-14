## **Сериализация и десериализация»** на deser

## Зависимости

| Крейт | Зачем |
|---|---|
| `serde` + `derive` | `Serialize` / `Deserialize` на типах |
| `serde_json` | JSON |
| `serde_yaml` | YAML |
| `toml` | TOML |
| `uuid` + `serde`, `v4` | поле `user_id` как UUID |
| `chrono` + `serde` | `debug.at` как `DateTime<Utc>` |

`serde` — только модель данных; форматы — отдельные бэкенды. README ещё рассказывает про `musli`, zero-copy и `rkyv`; в коде этого нет.

## Модель `Request`

Дерево структур зеркалит JSON:

```text
Request
├── type_request: TypeRequest     // JSON-ключ "type"
├── stream: Stream
│     ├── user_id: Uuid
│     ├── is_private, settings, shard_url
│     ├── public_tariff: PublicTariff   (id, price, duration, description)
│     └── private_tariff: PrivateTariff (client_price, duration, description)
├── gifts: Vec<Gift>
└── debug: Debug                    (duration: String, at: DateTime<Utc>)
```

Все типы: `Debug + Serialize + Deserialize + PartialEq` (сравнение в тестах).

Важные атрибуты:

- `#[serde(rename = "type")]` на `type_request` — `type` в Rust зарезервирован.
- `TypeRequest`: `Success` / `Error` → строки `"success"` / `"error"`.
- `rename` на `user_id`, `is_private`, `shard_url`, тарифах — имена как в JSON (snake_case). Комментарий про `clientPrice` — на будущее, в файле уже snake_case.
- `duration` везде **`String`** (`"1h"`, `"234ms"`), не `std::time::Duration`: это человекочитаемые метки, не ISO-duration.
- `at`: `"2019-06-28T08:35:46+00:00"` → `DateTime<Utc>` через serde chrono.

Файл `request.json` совпадает с тем, что зашито в `get_json_str()`; **с диска JSON не читается**.

## Поток в `main`

1. `serde_json::from_str::<Request>(get_json_str())`
2. снова в JSON (`to_string`) и печать
3. `serde_yaml::to_string` / `from_str`
4. `toml::to_string` / `from_str`

Одна структура, три формата — идея serde: типы не знают про JSON/YAML/TOML.

## Тесты

Три round-trip:

| Тест | Схема |
|---|---|
| `check_serde_json` | JSON → `Request` → JSON → `Request`, `assert_eq!` |
| `check_serde_yaml` | тот же JSON-текст кормят `serde_yaml::from_str` (YAML — надмножество JSON), затем YAML → структура |
| `check_toml` | JSON → структура → TOML → структура |

Нет сверки с эталонным YAML/TOML и нет негативных кейсов (битый UUID, неизвестный `"type"`).

## Что шаг показывает

- Derive вместо ручных `Serializer`/`Visitor`.
- Выбор типов: UUID, `DateTime<Utc>`, enum с `rename`, вложенные struct, `Vec`.
- Переименование полей под внешний контракт.
- Один `Request` сериализуется в разные текстовые форматы.

Ограничение: `request.json` не участвует в рантайме — дубль в исходнике. Для задания этого достаточно: статическая модель и печать YAML/TOML.
