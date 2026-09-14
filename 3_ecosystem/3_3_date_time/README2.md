## Работа с датой и временем в RUST

## Задание

Реализовать `User::age()` и `User::is_adult()`. «Текущее время» — константа `NOW`, чтобы тесты были воспроизводимы. В README отдельно разбираются `std::time`, `time` vs `chrono`, `hifitime` и почему длительности меряют через `Instant`, а не `SystemTime`.

В коде используется **`chrono`**, хотя в теории курса для новых проектов чаще советуют `time`. Для календарных дат без пояса `NaiveDate` здесь уместен.

## Модель данных

```13:15:C:\Users\user\work\MyWorks\Rust\rust-incubator\3_ecosystem\3_3_date_time\src\main.rs
const NOW: &str = "2019-06-26";

struct User(NaiveDate) ;
```

`User` — обёртка над датой рождения. Календарь без часового пояса: возраст считается в годах по датам, не по UTC/локали.

`main` создаёт пользователя `2010-02-01` и печатает возраст и флаг совершеннолетия (относительно 2019-06-26 это 9 лет и не взрослый).

## Конструктор

`with_birthdate(year, month, day)` вызывает `NaiveDate::from_ymd_opt`. Невалидная дата (например 31 февраля) — `panic!`. Для шага это достаточно; в проде обычно `Result`.

## `age()`

1. Разбор `NOW` через `NaiveDate::parse_from_str(NOW, "%Y-%m-%d")`. Ошибка парсинга — panic (константа валидна, ветка страховка).
2. Черновые годы: `now.year() - birth.year()`. Если отрицательные (рождение в будущем) — `0`.
3. Если месяц и день «сейчас» ещё не дошли до дня рождения — вычитается 1 год (если годы ещё > 0). Сравнение кортежей `(month, day)` корректно учитывает день рождения в этот календарный день: на `2019-06-26` человеку с ДР `1990-06-26` уже исполнилось бы очередной год.

Итог — `u16`.

Пример из тестов при `NOW = 2019-06-26`:

| Дата рождения | Возраст | Почему |
|---|---|---|
| 1990-06-04 | 29 | ДР уже прошёл |
| 1990-07-04 | 28 | ДР ещё впереди |
| 0000-01-01 | 2019 | крайность календаря |
| 1970-01-01 | 49 | |
| 2019-06-25 | 0 | меньше года |
| 2032-06-25 и позже | 0 | «будущее» |

## `is_adult()`

Просто `self.age() >= 18`. Совершеннолетие считается в полных годах на дату `NOW`.

Доп. тесты: `2000-02-01` → взрослый (19 лет); `2010-02-01` → нет (9 лет).

## Тесты

Модуль `age_spec`:

- `counts_age` — таблица возрастов из исходного задания.
- `zero_if_birthdate_in_future` — дни рождения после `NOW` дают 0 (в т.ч. `2016-06-27`: год в прошлом, но день ещё не наступил относительно 26 июня… wait, 2016-06-27 vs 2019-06-26)

Wait: 2016-06-27 vs NOW 2019-06-26:
years = 2019-2016 = 3
(6,26) < (6,27) so subtract 1 → 2 years, not 0!

The test expects 0 for ((2016, 6, 27), 0). That's interesting - 2016 is in the past, age should be 2, not 0.

Let me recalculate:
- NOW: 2019-06-26
- Birth: 2016-06-27
- years = 3
- (6, 26) < (6, 27) → true, years -= 1 → 2

But the test expects 0. That's a bug in either the implementation or the test. The user asked to describe the project, I should mention this discrepancy if it's real.

Looking at the test name: `zero_if_birthdate_in_future` and the cases:
- 2032-06-25 - future year, age 0 ✓
- 2016-06-27 - NOT future! This is 3 years ago minus 1 day = 2 years
- 3000, 9999 - future ✓

The comment says future birthdates. 2016-06-27 is NOT in the future relative to 2019-06-26. Unless they consider "birthday hasn't occurred yet this year AND something else"? No that's wrong.

Unless... they intended that if the person isn't born yet in the sense of... no 2016 is clearly past.

This might be a leftover/wrong test case. If they run tests, `zero_if_birthdate_in_future` would FAIL for (2016, 6, 27).

I'll mention this: the case `(2016, 6, 27) → 0` contradicts the age formula (should be 2). Either a mistaken fixture or they meant a different NOW.

I'll include this as a notable inconsistency.

## Что проект иллюстрирует

- Календарная арифметика через `chrono::NaiveDate` и `Datelike` (`year`/`month`/`day`).
- Возраст = разница лет с поправкой на то, прошёл ли день рождения.
- Фиксированная точка отсчёта вместо `Utc::now()`, чтобы тесты не зависели от реальных часов.
- Практический разрыв с теорией README: для интервалов нужен `Instant`; здесь нужны календарные даты, поэтому `chrono`, не `std::time`.
