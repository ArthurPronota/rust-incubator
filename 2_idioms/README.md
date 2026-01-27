Шаг 2: Идиомы
==============

__Estimated time__: 2 days

Эти шаги описывают распространенные идиомы, необходимые для написания хорошо продуманного и идиоматического кода на [Rust].

> ❗️Перед завершением этого шага необходимо выполнить все его подшаги.

After doing them you should be able to answer the following questions:
После выполнения этих заданий вы сможете ответить на следующие вопросы:

- [`Почему меня должны волновать типы и способ выражения информации с помощью типов? Как типы помогают повысить гарантии корректности программы?`](#почему-меня-должны-волновать-типы-и-способ-выражения-информации-с-помощью-типов-как-типы-помогают-повысить-гарантии-корректности-программы)

- [`Что необходимо для написания хорошо спроектированных и эргономичных API на [Rust] и почему?`](#%D1%87%D1%82%D0%BE-%D0%BD%D0%B5%D0%BE%D0%B1%D1%85%D0%BE%D0%B4%D0%B8%D0%BC%D0%BE-%D0%B4%D0%BB%D1%8F-%D0%BD%D0%B0%D0%BF%D0%B8%D1%81%D0%B0%D0%BD%D0%B8%D1%8F-%D1%85%D0%BE%D1%80%D0%BE%D1%88%D0%BE-%D1%81%D0%BF%D1%80%D0%BE%D0%B5%D0%BA%D1%82%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%BD%D1%8B%D1%85-%D0%B8-%D1%8D%D1%80%D0%B3%D0%BE%D0%BD%D0%BE%D0%BC%D0%B8%D1%87%D0%BD%D1%8B%D1%85-api-%D0%BD%D0%B0-rust-%D0%B8-%D0%BF%D0%BE%D1%87%D0%B5%D0%BC%D1%83)

- [Зачем существует `mem::replace` и какую цель он преследует? Когда и почему он действительно полезен?](#%D0%B7%D0%B0%D1%87%D0%B5%D0%BC-%D1%81%D1%83%D1%89%D0%B5%D1%81%D1%82%D0%B2%D1%83%D0%B5%D1%82-memreplace-%D0%B8-%D0%BA%D0%B0%D0%BA%D1%83%D1%8E-%D1%86%D0%B5%D0%BB%D1%8C-%D0%BE%D0%BD-%D0%BF%D1%80%D0%B5%D1%81%D0%BB%D0%B5%D0%B4%D1%83%D0%B5%D1%82-%D0%BA%D0%BE%D0%B3%D0%B4%D0%B0-%D0%B8-%D0%BF%D0%BE%D1%87%D0%B5%D0%BC%D1%83-%D0%BE%D0%BD-%D0%B4%D0%B5%D0%B9%D1%81%D1%82%D0%B2%D0%B8%D1%82%D0%B5%D0%BB%D1%8C%D0%BD%D0%BE-%D0%BF%D0%BE%D0%BB%D0%B5%D0%B7%D0%B5%D0%BD)

- [`Как обычно организуется полиморфизм типов входных данных в API на Rust? Какова его стоимость?`](#%D0%BA%D0%B0%D0%BA-%D0%BE%D0%B1%D1%8B%D1%87%D0%BD%D0%BE-%D0%BE%D1%80%D0%B3%D0%B0%D0%BD%D0%B8%D0%B7%D1%83%D0%B5%D1%82%D1%81%D1%8F-%D0%BF%D0%BE%D0%BB%D0%B8%D0%BC%D0%BE%D1%80%D1%84%D0%B8%D0%B7%D0%BC-%D1%82%D0%B8%D0%BF%D0%BE%D0%B2-%D0%B2%D1%85%D0%BE%D0%B4%D0%BD%D1%8B%D1%85-%D0%B4%D0%B0%D0%BD%D0%BD%D1%8B%D1%85-%D0%B2-api-%D0%BD%D0%B0-rust-%D0%BA%D0%B0%D0%BA%D0%BE%D0%B2%D0%B0-%D0%B5%D0%B3%D0%BE-%D1%81%D1%82%D0%BE%D0%B8%D0%BC%D0%BE%D1%81%D1%82%D1%8C)

- [`Какие существуют способы и инструменты для обеспечения перспективности исходного кода на Rust?`](#%D0%BA%D0%B0%D0%BA%D0%B8%D0%B5-%D1%81%D1%83%D1%89%D0%B5%D1%81%D1%82%D0%B2%D1%83%D1%8E%D1%82-%D1%81%D0%BF%D0%BE%D1%81%D0%BE%D0%B1%D1%8B-%D0%B8-%D0%B8%D0%BD%D1%81%D1%82%D1%80%D1%83%D0%BC%D0%B5%D0%BD%D1%82%D1%8B-%D0%B4%D0%BB%D1%8F-%D0%BE%D0%B1%D0%B5%D1%81%D0%BF%D0%B5%D1%87%D0%B5%D0%BD%D0%B8%D1%8F-%D0%BF%D0%B5%D1%80%D1%81%D0%BF%D0%B5%D0%BA%D1%82%D0%B8%D0%B2%D0%BD%D0%BE%D1%81%D1%82%D0%B8-%D0%B8%D1%81%D1%85%D0%BE%D0%B4%D0%BD%D0%BE%D0%B3%D0%BE-%D0%BA%D0%BE%D0%B4%D0%B0-%D0%BD%D0%B0-rust)

<hr>

<h3>Почему меня должны волновать типы и способ выражения информации с помощью типов? Как типы помогают повысить гарантии корректности программы?</h3>

В Rust типы — это не просто разметка памяти или способ подсказать компилятору размер переменной. Это инструмент проектирования и верификации, который позволяет переложить проверку бизнес-логики с человека на машину.

Вот почему типы в Rust напрямую влияют на корректность вашей программы:

<h4>1. Превращение логических ошибок в ошибки компиляции</h4>

Самое мощное преимущество — возможность сделать невалидные состояния непредставимыми.

- Пример: Вместо того чтобы проверять `if user.is_activated` в каждой функции, вы создаете два типа: `IncompleteUser` и `ActivatedUser`. Функция отправки уведомления принимает только `ActivatedUser`.
- Результат: Вы физически не сможете скомпилировать код, который пытается отправить письмо неактивированному пользователю. Ошибка обнаруживается не в 3 часа ночи на сервере, а сразу в IDE.

<h4>2. Гарантии безопасности памяти без GC (Garbage Collection)</h4>

Система типов Rust (через механизмы владения и времен жизни — `lifetimes`) гарантирует отсутствие целого класса критических багов:

- Dangling pointers (висячие указатели).
- Double free (двойное освобождение памяти).
- Data races (состояния гонки в многопоточности).

    Компилятор использует типы как «доказательство» того, что доступ к памяти безопасен. Если доказательство не сходится, программа не соберется.

<h4>3. Шаблон NewType: семантическая ясность</h4>

Типы помогают избежать путаницы между данными, которые технически выглядят одинаково, но значат разное.

- Без типов: Вы передаете два `u64` в функцию `transfer(id1, id2, amount)`. Легко перепутать отправителя и получателя.
- С типами: `transfer(from: SenderId, to: ReceiverId, amount: u64)`. 

    Компилятор выдаст ошибку, если вы перепутаете ID местами, хотя «под капотом» это те же числа.

<h4>4. Паттерн Typestate (Состояния в типах)</h4>

Типы позволяют кодировать протоколы работы с объектами.

- Пример: У вас есть объект `File`. Вы можете вызвать `write()` только если файл находится в состоянии (типе) `OpenFile<Writable>`. Если файл закрыт, у типа `File<Closed>` просто нет метода `write()`.
- Результат: Вы гарантированно не забудете открыть файл перед записью или закрыть его после — это проверяет система типов.

<h4>5. Безопасный рефакторинг</h4>

Проекты становятся всё больше. Типы позволяют смело менять архитектуру.

- Когда вы меняете тип возвращаемого значения или структуру данных, компилятор подсвечивает все места в коде, которые нужно исправить. Это гораздо надежнее, чем поиск по тексту или надежда на то, что тесты покроют все случаи.

<h4>Преимущества и недостатки</h4>

|Преимущества|Недостатки|
|------------|----------|
|Надежность: Ошибки обнаруживаются на этапе сборки.|Крутая кривая обучения: Нужно учиться думать «в типах».|
|Документация: Типы — это всегда актуальное описание логики.|Время проектирования: Начало разработки занимает больше времени.|
|Скорость: Конкретные типы позволяют компилятору делать агрессивные оптимизации.|Визуальный шум: Сложные дженерики могут выглядеть громоздко.|

<h4>Резюме</h4>

Вас должны волновать типы, потому что они — ваша страховка. В Rust вы тратите время на проектирование типов сейчас, чтобы никогда не тратить время на отладку глупых ошибок и повреждений памяти в будущем.
Изучите подробнее [`Rust Design Patterns`](https://rust-unofficial.github.io/patterns/intro.html) для эффективного использования этого подхода.


<hr>

<h3>Что необходимо для написания хорошо спроектированных и эргономичных API на [Rust] и почему?<h3>

Для написания хорошо спроектированных и эргономичных API в Rust необходимо придерживаться принципа: «Легко использовать правильно и трудно использовать неправильно».

Вот ключевые элементы и причины их важности:

<h4>1. Использование системы типов для выражения семантики</h4>

Вместо того чтобы полагаться на документацию или проверки в рантайме, идиоматичный API кодирует правила в типах.

- Паттерн NewType: Создавайте обертки вроде `struct UserId(u64)`, чтобы пользователь не перепутал `ID пользователя` с `ID заказа`. Это гарантирует корректность на этапе компиляции.
- Typestate Pattern: Кодируйте состояния объекта в его типах (например, `File<Closed>` и `File<Open>`). Это физически запрещает вызов методов в неверном порядке (например, запись в закрытый файл).

<h4>2. Гибкость входных данных (Trait Bounds)</h4>

Эргономичный API не заставляет пользователя вручную конвертировать данные.

- Использование `AsRef` и `Into`: Вместо `fn process(s: String)`, пишите `fn process<S: Into<String>>(s: S)`. Это позволит пользователю передавать и `&str`, и `String`, и даже свои типы, которые могут быть конвертированы.
- Приоритет `impl Trait`: для аргументов функций предпочтительнее использовать `fn search(item: impl Display)`, так как это чище читается, чем классические дженерики.

<h4>3. Предсказуемость и соблюдение стандартов (Common Traits)</h4>

Ваши типы должны вести себя так, как ожидает опытный разработчик на Rust.

- Реализация стандартных трейтов: Любой тип данных должен, где это уместно, реализовывать `Debug, Clone, Default, PartialEq, Serialize/Deserialize (Serde)`.
- Объекты должны быть `Send` и `Sync`: Если это технически возможно. Это гарантирует, что ваш API можно использовать в многопоточных и асинхронных (`Tokio`) средах без «костылей».

<h4>4. Понятная обработка ошибок</h4>

- Явные перечисления ошибок: Используйте `enum` для всех возможных сбоев и помечайте их атрибутом `#[non_exhaustive]`, чтобы добавление новых ошибок в будущем не ломало код пользователей.
- Интеграция с экосистемой: Реализуйте трейт `std::error::Error` для своих типов ошибок, чтобы они корректно работали с оператором `?` и библиотеками вроде `anyhow` или `thiserror`.

<h4>5. Инкапсуляция и «запечатывание»</h4>

- Sealed Traits: Если вы предоставляете трейт, который пользователь должен использовать, но не должен реализовывать, используйте паттерн «запечатывания». Это позволяет вам добавлять новые методы в трейт, не ломая обратную совместимость.
- Скрытие полей: Все поля структур должны быть приватными по умолчанию, предоставляя доступ через методы-геттеры. Это дает вам свободу менять внутреннюю реализацию в будущем.

<h4>Почему это важно?</h4>

1. Снижение когнитивной нагрузки: Разработчик тратит меньше времени на чтение мануалов, так как API «подсказывает» правильный путь через автодополнение и ошибки компилятора.
2. Безопасность рефакторинга: Если вы измените логику в хорошо спроектированном API, компилятор найдет все несовместимые места в коде пользователя.
3. Производительность: Использование статической диспетчеризации (generics) вместо dyn позволяет компилятору оптимизировать вызовы под конкретные типы данных пользователя.

<h4>Итог:</h4>

Хороший API в Rust — это тот, который превращает логические ошибки в ошибки компиляции. Это делает разработку медленнее на этапе написания кода, но в разы быстрее и дешевле на этапе поддержки и эксплуатации.

Рекомендуемый ресурс для углубления: [`Rust API Guidelines`](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjdtfrDlKuSAxWWU1UIHSl6AigQFnoECBYQAQ&url=https%3A%2F%2Frust-unofficial.github.io%2Fpatterns%2F&usg=AOvVaw2pdk76xLoPutW2f8Pa53eU&opi=89978449).

<hr>

<h3>Зачем существует `mem::replace` и какую цель он преследует? Когда и почему он действительно полезен?</h3>

В Rust `std::mem::replace` существует для того, чтобы легально «украсть» владение данными по изменяемой ссылке, не нарушая при этом правила безопасности памяти.

<h4>Зачем он нужен? (Главная причина)</h4>

Основное правило Rust гласит: переменная никогда не может быть <b>неинициализированной</b>.

Если у вас есть изменяемая ссылка `&mut T`, вы не можете просто переместить значение из-под неё, потому что в этот момент ячейка памяти, на которую указывает ссылка, станет пустой (неинициализированной). Если в этот миг программа упадет (`panic`) или кто-то попытается прочитать данные по этой ссылке, произойдет повреждение памяти `(Undefined Behavior`).

```rust
// Это НЕ СКОМПИЛИРУЕТСЯ
fn swap_out(r: &mut String) -> String {
    let val = *r; // Ошибка! Попытка перемещения (move) из-под ссылки
    *r = String::from("new"); 
    val
}
```

`mem::replace` решает эту проблему, выполняя две операции атомарно для компилятора: он записывает новое значение в ячейку и одновременно возвращает старое.

<h4>Какую цель он преследует?</h4>

1. Соблюдение инвариантов: Гарантировать, что по адресу `&mut T` всегда находятся валидные данные, даже в процессе их замены.
2. Эффективность: Позволяет избежать дорогого клонирования (`.clone()`), когда вам нужно просто забрать старое значение и положить на его место новое (или значение по умолчанию).

<h4>Когда и почему он действительно полезен? (Примеры)</h4>

1. Смена состояний в `Enum`

Это самый частый сценарий. Если вы хотите извлечь данные из одного варианта перечисления и переключить его в другой, не прибегая к `unsafe` или клонированию.

```rust
use std::mem;

enum State {
    Running(String),
    Stopped,
}

fn stop_engine(state: &mut State) {
    // Мы заменяем текущее состояние на Stopped и получаем владение строкой из Running
    if let State::Running(message) = mem::replace(state, State::Stopped) {
        println!("Остановка с сообщением: {}", message);
    }
}
```

2. Извлечение данных для трансформации

Если вам нужно забрать данные из структуры, чтобы передать их в функцию, которая потребляет владение (`move`), а затем записать результат обратно.

```rust
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn process(&mut self) {
        // Заменяем вектор на пустой временно
        let old_data = mem::replace(&mut self.data, Vec::new());
        // Теперь мы владеем old_data и можем делать с ней что угодно
        self.data = some_expensive_computation(old_data);
    }
}
```

3. Реализация структур данных (`Linked Lists`, `Trees`)

При манипуляции узлами часто нужно «отцепить» часть дерева или списка. Для типов `Option<T>` чаще используют `option.take()`, но важно знать, что под капотом `take()` — это всего лишь:
`mem::replace(opt, None)`.

<h4>Почему нельзя без него?</h4>

Без `mem::replace` у вас осталось бы три плохих варианта:

- Клонировать данные: Это медленно и требует лишней памяти.
- Использовать unsafe: Вручную читать и писать в память через `ptr::read` / `ptr::write`, что опасно и чревато ошибками при паниках.
- Оборачивать всё в `Option`: Чтобы иметь возможность делать .take(), что усложняет код постоянными `unwrap()`.

<h4>Итог</h4>

`mem::replace` — это инструмент для безопасного перемещения данных в условиях ограниченного владения. Он позволяет поддерживать высокую производительность Rust, не жертвуя при этом безопасностью памяти.

<hr>

<h3>Как обычно организуется полиморфизм типов входных данных в API на Rust? Какова его стоимость?</h3>

В Rust полиморфизм входных данных в API организуется тремя основными способами, каждый из которых имеет свою семантическую и вычислительную стоимость.

<h4>1. Статическая диспетчеризация (Generics)</h4>

Это наиболее распространенный и идиоматичный подход. Вы определяете функцию, которая принимает любой тип `T`, реализующий определенный трейт.

- Как это выглядит: `fn process<T: MyTrait>(input: T)` или более современный вариант `fn process(input: impl MyTrait)`.
- Механизм: Компилятор использует мономорфизацию. Для каждого конкретного типа, с которым вызывается функция, генерируется отдельная копия машинного кода.

- Стоимость:
    - В рантайме: Нулевая (`Zero-cost`). Вызов происходит напрямую, компилятор может применить агрессивный инлайнинг (встраивание кода).
    - При компиляции: Увеличивается время сборки и размер бинарного файла (`«code bloat»`), так как создается множество копий одной функции.

<h4>2. Динамическая диспетчеризация (Trait Objects)</h4>

Используется, когда типы данных неизвестны на этапе компиляции или когда нужно хранить разные типы в одной коллекции.

- Как это выглядит: `fn process(input: &dyn MyTrait)` или `Box<dyn MyTrait>`.
- Механизм: Используются «толстые указатели» и `vtable` (таблица виртуальных методов). Адрес функции ищется во время выполнения программы.

- Стоимость:
    - В рантайме: Есть небольшие накладные расходы на косвенный вызов (переход по указателю). Невозможен полный инлайнинг кода.
    - При компиляции: Минимальная. Генерируется только одна копия функции, что экономит место в бинарном файле.

<h4>3. Перечисления (Enums)</h4>

Если набор возможных входных типов фиксирован и известен заранее, часто лучше использовать `enum`.

- Как это выглядит: `fn process(input: InputType)`, где `InputType` — это перечисление вариантов.
- Механизм: Простая проверка тега (дискриминанта) через `match`.
- Стоимость:
    - В рантайме: Очень низкая (сравнима со статической диспетчеризацией).
    - Память: Объект `enum` занимает столько места, сколько его самый большой вариант плюс тег.

<h4>Сводная таблица сравнения</h4>

|Метод|Гибкость|Скорость (Runtime)|Время сборки|
|-----|--------|------------------|------------|
|Generics (impl Trait)|Высокая (статическая)|Максимальная|Медленно|
|Trait Objects (dyn)|Максимальная (динамическая)|Средняя|Быстро|
|Enums|Ограничена набором вариантов|Высокая|Быстро|

<h4>Когда и что выбирать?</h4>

1. Для публичных API библиотек почти всегда выбирают Generics с использованием трейтов `AsRef`, `Into` или `Borrow`. Это позволяет пользователю передавать разные типы (например, `&str`, `String`, `PathBuf`) максимально эффективно.
    - Пример: `fn open<P: AsRef<Path>>(path: P)`.
2. Для плагинов и сложных UI-систем, где объекты создаются динамически и их типы могут быть не видны компилятору целиком, выбирают `dyn Trait`.
3. Для внутренней логики, где важна скорость и набор типов ограничен, выбирают `Enums`.

<h4>Компромисс:</h4>

Чтобы уменьшить раздувание кода при использовании дженериков, часто применяют паттерн «Inner Helper»: публичная функция является обобщенной (для удобства пользователя), но она сразу вызывает приватную функцию с `dyn Trait` или конкретным типом, чтобы основная логика не дублировалась в бинарном файле.

<hr>

<h3>Какие существуют способы и инструменты для обеспечения перспективности исходного кода на Rust?</h3>

Обеспечение «перспективности» (`future-proofing`) кода на Rust — это сочетание архитектурных паттернов, встроенных инструментов компилятора и строгого следования идиомам сообщества.

Вот основные способы и инструменты, разделенные по уровням применения:

<h3>1. Инструменты контроля качества и стандартов</h3>

- Rust Edition: Редакции (2018, 2021, 2024, 2027) позволяют языку развиваться, не ломая старый код. Использование актуальной редакции (на текущий момент 2024/2026) гарантирует поддержку новейших оптимизаций.
- Clippy: Это не просто линтер, а инструмент обучения. Он содержит более 700 проверок, которые помогают писать код в соответствии с современными практиками производительности и безопасности.
- Rust API Guidelines: Следование этому «чек-листу» при проектировании публичных интерфейсов гарантирует, что ваш код будет привычен и удобен для других разработчиков в будущем.

<h4>2. Языковые механизмы для обратной совместимости</h4>

- Атрибут `#[non_exhaustive]`: Применяется к структурам и перечислениям. Он запрещает пользователям вашего кода использовать исчерпывающий match или создавать структуру напрямую. Это позволяет вам добавлять новые поля или варианты в будущем, не ломая код пользователей.
- Запечатанные трейты (`Sealed Traits`): Паттерн, использующий приватные модули, чтобы запретить внешним пользователям реализовывать ваш трейт. Это дает вам право добавлять новые методы в трейт в минорных обновлениях.
- `SemVer` через `cargo-public-api`: Инструмент, который отслеживает изменения в публичном интерфейсе вашей библиотеки и предупреждает о нарушениях семантического версионирования.

<h4>3. Архитектурные паттерны</h4>

- `Type-Driven Design`: Кодирование бизнес-логики в типах (например, паттерны `NewType` и `Typestate`). Это переносит проверки корректности на компилятор, делая код устойчивым к будущим изменениям логики.
- Абстрагирование через `impl Trait`: Возврат `impl Iterator` вместо конкретного типа `Map<Filter<...>>` позволяет вам изменить внутреннюю реализацию коллекции, не меняя сигнатуру функции и не ломая код клиента.
- Минимизация ограничений в `struct`: Перенос ограничений трейтов (`bounds`) из определений структур в блоки `impl`. Это делает типы более гибкими для повторного использования в новых контекстах.

<h4>4. Управление зависимостями</h4>

- `Cargo.lock`: Фиксация версий зависимостей гарантирует воспроизводимость сборки через годы.
- Инструмент `cargo-deny`: Позволяет проверять граф зависимостей на наличие уязвимостей, плохих лицензий и дубликатов, что критично для долгосрочной поддержки проекта.

<h4>5. Современные возможности</h4>

- `GATs` (`Generic Associated Types`): Позволяют создавать гораздо более гибкие и абстрактные API, которые могут адаптироваться к сложным сценариям владения данными.
- `Const Generics`: Использование констант в типах (например, размер массива) делает код более производительным и избавляет от магических чисел, облегчая поддержку.

<h4>Резюме:</h4>

Чтобы ваш код оставался актуальным, проектируйте его так, чтобы компилятор мог поймать как можно больше ошибок при будущих изменениях. Используйте `clippy`, применяйте `#[non_exhaustive]` для публичных типов и всегда отдавайте предпочтение строгой типизации перед проверками в рантайме.



<hr>

## Task

Разработайте и реализуйте тип `VendingMachine`, который ведет себя как [торговый автомат][1]:
- `Product` должны иметь price и name;
- `VendingMachine` должен иметь ограниченную ёмкость `Product`;
- `VendingMachine` должен быть способным давать сдачу;
- `VendingMachine` должен отклонять покупку если он не может дать сдачу;
- `Coin` номинальные значения могут быть только `1`, `2`, `5`, `10`, `20` и `50`.

Сделайте API для его использования максимально удобным, насколько это возможно.


[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Vending_machine

<hr>
use std::collections::HashMap;
use std::fmt;
use thiserror::Error; // Для удобных ошибок, можно заменить на ручную реализацию

// Номиналы монет
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

impl Coin {
    // Получить все возможные номиналы для итерации
    pub fn all() -> [Coin; 6] {
        [
            Coin::One,
            Coin::Two,
            Coin::Five,
            Coin::Ten,
            Coin::Twenty,
            Coin::Fifty,
        ]
    }
    
    // Значение монеты
    pub fn value(&self) -> u32 {
        *self as u32
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}₽", self.value())
    }
}

// Продукт
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: u32, // в наименьших единицах (копейках/центах)
}

impl Product {
    pub fn new(name: impl Into<String>, price: u32) -> Self {
        Self {
            name: name.into(),
            price,
        }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}.{:02}₽", self.name, self.price / 100, self.price % 100)
    }
}

// Ошибки автомата
#[derive(Debug, Error, PartialEq)]
pub enum VendingError {
    #[error("Product '{0}' not found")]
    ProductNotFound(String),
    
    #[error("Insufficient funds. Required: {0}, Provided: {1}")]
    InsufficientFunds(u32, u32),
    
    #[error("Insufficient change. Cannot give change for {0}")]
    InsufficientChange(u32),
    
    #[error("Product '{0}' is out of stock")]
    OutOfStock(String),
    
    #[error("Machine is at full capacity")]
    MachineFull,
    
    #[error("Invalid coin: {0}")]
    InvalidCoin(u32),
    
    #[error("No money inserted")]
    NoMoneyInserted,
}

// Результат покупки
#[derive(Debug, PartialEq)]
pub struct PurchaseResult {
    pub product: Product,
    pub change: HashMap<Coin, u32>,
    pub total_change: u32,
}

impl fmt::Display for PurchaseResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Successfully purchased: {}", self.product)?;
        writeln!(f, "Total change: {}.{:02}₽", self.total_change / 100, self.total_change % 100)?;
        
        if !self.change.is_empty() {
            writeln!(f, "Change breakdown:")?;
            for (coin, count) in &self.change {
                if *count > 0 {
                    writeln!(f, "  {} x {}", coin, count)?;
                }
            }
        }
        
        Ok(())
    }
}

// Торговый автомат
pub struct VendingMachine {
    products: HashMap<String, (Product, u32)>, // name -> (product, quantity)
    coins: HashMap<Coin, u32>, // номинал -> количество
    capacity: usize,
    inserted_coins: HashMap<Coin, u32>,
    total_inserted: u32,
}

impl VendingMachine {
    /// Создает новый торговый автомат с указанной вместимостью
    pub fn new(capacity: usize) -> Self {
        Self {
            products: HashMap::new(),
            coins: HashMap::new(),
            capacity,
            inserted_coins: HashMap::new(),
            total_inserted: 0,
        }
    }
    
    /// Добавляет продукт в автомат
    pub fn add_product(&mut self, product: Product, quantity: u32) -> Result<(), VendingError> {
        if self.products.len() >= self.capacity && !self.products.contains_key(&product.name) {
            return Err(VendingError::MachineFull);
        }
        
        let entry = self.products
            .entry(product.name.clone())
            .or_insert_with(|| (product.clone(), 0));
        
        entry.1 += quantity;
        Ok(())
    }
    
    /// Загружает монеты в автомат для сдачи
    pub fn load_coins(&mut self, coins: HashMap<Coin, u32>) {
        for (coin, count) in coins {
            *self.coins.entry(coin).or_insert(0) += count;
        }
    }
    
    /// Вставляет монету
    pub fn insert_coin(&mut self, coin: Coin) -> Result<u32, VendingError> {
        *self.inserted_coins.entry(coin).or_insert(0) += 1;
        self.total_inserted += coin.value();
        Ok(self.total_inserted)
    }
    
    /// Вставляет несколько одинаковых монет
    pub fn insert_coins(&mut self, coin: Coin, count: u32) -> Result<u32, VendingError> {
        for _ in 0..count {
            self.insert_coin(coin)?;
        }
        Ok(self.total_inserted)
    }
    
    /// Покупает продукт
    pub fn purchase(&mut self, product_name: &str) -> Result<PurchaseResult, VendingError> {
        if self.total_inserted == 0 {
            return Err(VendingError::NoMoneyInserted);
        }
        
        // Проверяем наличие продукта
        let (product, quantity) = self.products
            .get_mut(product_name)
            .ok_or_else(|| VendingError::ProductNotFound(product_name.to_string()))?;
        
        if *quantity == 0 {
            return Err(VendingError::OutOfStock(product_name.to_string()));
        }
        
        // Проверяем достаточно ли денег
        if self.total_inserted < product.price {
            return Err(VendingError::InsufficientFunds(
                product.price,
                self.total_inserted,
            ));
        }
        
        // Рассчитываем сдачу
        let change_amount = self.total_inserted - product.price;
        
        // Пытаемся выдать сдачу
        let change = self.calculate_change(change_amount)?;
        
        // Уменьшаем количество продукта
        *quantity -= 1;
        
        // Добавляем внесенные монеты в автомат
        for (coin, count) in &self.inserted_coins {
            *self.coins.entry(*coin).or_insert(0) += count;
        }
        
        // Убираем монеты для сдачи из автомата
        for (coin, count) in &change {
            if let Some(available) = self.coins.get_mut(coin) {
                *available -= count;
            }
        }
        
        // Очищаем внесенные монеты
        let result = PurchaseResult {
            product: product.clone(),
            change: change.clone(),
            total_change: change_amount,
        };
        
        self.clear_inserted_coins();
        
        Ok(result)
    }
    
    /// Возвращает все внесенные деньги
    pub fn cancel(&mut self) -> HashMap<Coin, u32> {
        let coins = self.inserted_coins.clone();
        self.clear_inserted_coins();
        coins
    }
    
    /// Рассчитывает оптимальную сдачу
    fn calculate_change(&self, amount: u32) -> Result<HashMap<Coin, u32>, VendingError> {
        if amount == 0 {
            return Ok(HashMap::new());
        }
        
        let mut change = HashMap::new();
        let mut remaining = amount;
        
        // Копируем текущие монеты в автомате + внесенные
        let mut available_coins = self.available_coins_with_inserted();
        
        // Сортируем номиналы по убыванию для жадного алгоритма
        let mut denominations = Coin::all().to_vec();
        denominations.sort_by(|a, b| b.value().cmp(&a.value()));
        
        for coin in denominations {
            let coin_value = coin.value();
            
            if coin_value > remaining {
                continue;
            }
            
            if let Some(&available) = available_coins.get(&coin) {
                let needed = remaining / coin_value;
                let count = needed.min(available);
                
                if count > 0 {
                    change.insert(coin, count);
                    remaining -= coin_value * count;
                    *available_coins.get_mut(&coin).unwrap() -= count;
                }
            }
            
            if remaining == 0 {
                break;
            }
        }
        
        if remaining > 0 {
            return Err(VendingError::InsufficientChange(amount));
        }
        
        Ok(change)
    }
    
    /// Получает все доступные монеты (в автомате + внесенные)
    fn available_coins_with_inserted(&self) -> HashMap<Coin, u32> {
        let mut all_coins = self.coins.clone();
        
        for (coin, count) in &self.inserted_coins {
            *all_coins.entry(*coin).or_insert(0) += count;
        }
        
        all_coins
    }
    
    /// Очищает внесенные монеты
    fn clear_inserted_coins(&mut self) {
        self.inserted_coins.clear();
        self.total_inserted = 0;
    }
    
    /// Проверяет наличие продукта
    pub fn has_product(&self, product_name: &str) -> bool {
        self.products
            .get(product_name)
            .map_or(false, |(_, qty)| *qty > 0)
    }
    
    /// Получает информацию о продуктах
    pub fn get_products(&self) -> Vec<(Product, u32)> {
        self.products.values().map(|(p, q)| (p.clone(), *q)).collect()
    }
    
    /// Получает информацию о монетах в автомате
    pub fn get_coins(&self) -> HashMap<Coin, u32> {
        self.coins.clone()
    }
    
    /// Получает информацию о внесенных монетах
    pub fn get_inserted_coins(&self) -> HashMap<Coin, u32> {
        self.inserted_coins.clone()
    }
    
    /// Получает общую сумму внесенных денег
    pub fn get_inserted_amount(&self) -> u32 {
        self.total_inserted
    }
}

// Реализация Display для удобного вывода
impl fmt::Display for VendingMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Vending Machine ===")?;
        writeln!(f, "Capacity: {} products", self.capacity)?;
        writeln!(f, "Products:")?;
        
        for (product, quantity) in self.get_products() {
            writeln!(f, "  {} ({} left)", product, quantity)?;
        }
        
        writeln!(f, "Coins in machine:")?;
        let mut total_coins = 0;
        for (coin, count) in &self.coins {
            if *count > 0 {
                writeln!(f, "  {}: {}", coin, count)?;
                total_coins += coin.value() * count;
            }
        }
        writeln!(f, "Total coins value: {}.{:02}₽", total_coins / 100, total_coins % 100)?;
        
        if self.total_inserted > 0 {
            writeln!(f, "Inserted coins:")?;
            for (coin, count) in &self.inserted_coins {
                writeln!(f, "  {}: {}", coin, count)?;
            }
            writeln!(f, "Total inserted: {}.{:02}₽", 
                self.total_inserted / 100, self.total_inserted % 100)?;
        }
        
        Ok(())
    }
}

// Builder pattern для удобного создания автомата
pub struct VendingMachineBuilder {
    capacity: usize,
    products: Vec<(Product, u32)>,
    coins: HashMap<Coin, u32>,
}

impl VendingMachineBuilder {
    pub fn new() -> Self {
        Self {
            capacity: 10,
            products: Vec::new(),
            coins: HashMap::new(),
        }
    }
    
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
    
    pub fn add_product(mut self, product: Product, quantity: u32) -> Self {
        self.products.push((product, quantity));
        self
    }
    
    pub fn add_coins(mut self, coin: Coin, count: u32) -> Self {
        *self.coins.entry(coin).or_insert(0) += count;
        self
    }
    
    pub fn build(self) -> Result<VendingMachine, VendingError> {
        let mut machine = VendingMachine::new(self.capacity);
        
        for (product, quantity) in self.products {
            machine.add_product(product, quantity)?;
        }
        
        machine.load_coins(self.coins);
        
        Ok(machine)
    }
}

// Пример использования
fn main() {
    // Используем builder для создания автомата
    let mut machine = VendingMachineBuilder::new()
        .capacity(5)
        .add_product(Product::new("Coke", 150), 3) // 1.50₽
        .add_product(Product::new("Chips", 200), 5) // 2.00₽
        .add_product(Product::new("Chocolate", 125), 2) // 1.25₽
        .add_coins(Coin::Five, 10)
        .add_coins(Coin::Ten, 5)
        .add_coins(Coin::One, 20)
        .build()
        .expect("Failed to build vending machine");
    
    println!("{}", machine);
    
    // Пример покупки 1
    println!("=== Purchase 1 ===");
    
    // Вставляем монеты
    machine.insert_coin(Coin::Ten).unwrap();
    machine.insert_coin(Coin::Ten).unwrap(); // Всего 20
    machine.insert_coins(Coin::One, 3).unwrap(); // Всего 23
    
    println!("Inserted: {}.{:02}₽", 
        machine.get_inserted_amount() / 100,
        machine.get_inserted_amount() % 100);
    
    // Покупаем шоколад за 1.25₽
    match machine.purchase("Chocolate") {
        Ok(result) => {
            println!("{}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    
    println!("\n{}", machine);
    
    // Пример покупки 2 - недостаточно сдачи
    println!("\n=== Purchase 2 ===");
    
    // Вставляем большую купюру (эмулируем банкноту как 50 монет)
    machine.insert_coins(Coin::Fifty, 1).unwrap();
    
    // Пытаемся купить чипсы за 2.00₽
    // Не будет сдачи, так как нет мелких монет
    match machine.purchase("Chips") {
        Ok(result) => {
            println!("{}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    
    // Отменяем операцию
    let returned = machine.cancel();
    println!("Cancelled, returned coins:");
    for (coin, count) in returned {
        println!("  {} x {}", coin, count);
    }
    
    println!("\n{}", machine);
    
    // Пример покупки 3 - продукта нет
    println!("\n=== Purchase 3 ===");
    
    machine.insert_coins(Coin::Ten, 2).unwrap(); // 20
    
    match machine.purchase("Pepsi") {
        Ok(result) => {
            println!("{}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}