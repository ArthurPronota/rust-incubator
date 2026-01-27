/*
    Этот код создаёт trait MyIteratorExt с двумя реализованными методами
        1) fn format(self, sep: &str) -> Format<Self>
        2) fn format_with<F>(self, sep: &str, format: F) -> FormatWith<Self, F>
     и покрывающей реализацией для всего что является Iterator:
        impl<I: Iterator> MyIteratorExt for I {}


    Внесённые изменения:
1) добавлен модуль mod private для запечатывания trait.
2) модифицировано определение trait:
     pub trait MyIteratorExt: Iterator + private::Sealed
3) модифицирована реализация trait:
    impl<I: Iterator> MyIteratorExt for I {}

Для того чтобы документальные тесты проходили нужно раскомментировать
комментарии //\*
В них содержится код реализующий trait MyIteratorExt

    Запуск теста:   cargo test --doc
    Вывод в части документальных тестов этого файла:
2_idioms\2_6_sealing\src\my_error.rs - my_error::MyError::source (line 33)    

                Описания некоторых разделов rust.

                        1. FnMut

    1. Замыкание, реализующее FnMut, может изменять значения переменных, 
    захваченных из внешнего окружения, между вызовами. Для вызова такого 
    замыкания требуется изменяемая ссылка (&mut self).

    Сигнатура трейта:
```rust
pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
```

Ключевые характеристики
1) Многократный вызов: В отличие от FnOnce, замыкание FnMut можно вызывать
    много раз.
2) Изменяемое состояние: Оно может хранить и обновлять состояние внутри 
    себя (например, счетчик или накопление строк).
3) Иерархия: Любое замыкание, реализующее Fn, автоматически реализует 
    FnMut. Любое FnMut автоматически реализует FnOnce.
4) Требование мутабельности: Чтобы вызвать замыкание типа FnMut, сама 
    переменная, в которой оно хранится, должна быть помечена как mut.

```rust
fn main() {
    let mut count = 0;

    // Это замыкание реализует FnMut, так как оно изменяет `count`
    let mut increment = || {
        count += 1;
        println!("Счетчик: {}", count);
    };

    increment(); // Вызов 1
    increment(); // Вызов 2
}
```

Когда использовать в аргументах функций?

Используйте FnMut, когда вы пишете функцию высшего порядка, которая 
должна вызывать переданное ей замыкание несколько раз и позволять ему 
обновлять своё состояние.

```rust
fn do_twice<F>(mut func: F) 
where 
    F: FnMut() 
{
    func();
    func();
}

fn main() {
    let mut s = String::new();
    // Замыкание добавляет данные в строку при каждом вызове
    do_twice(|| s.push_str("a")); 
    println!("{}", s); // Выведет "aa"
}
```

Сравнение с другими трейтами:
1) Fn: Только чтение окружения (&self). Можно вызывать параллельно из 
    разных потоков.
2) FnMut: Изменение окружения (&mut self). Нельзя вызывать параллельно 
    без синхронизации.
3) FnOnce: Потребление окружения (self). Можно вызвать только один раз.


------------------------------------------------------------------------
                        2. Itertools::Itertools::format

В Rust метод .format() из крейта itertools является одним из самых 
эффективных и удобных способов форматирования элементов итератора в 
строку с использованием разделителя.

Метод .format() позволяет создать объект, который реализует Display. 
Это означает, что данные не выделяются в кучу (no heap allocation) сразу.
Форматирование происходит «лениво» только в тот момент, когда вы реально 
пытаетесь напечатать результат или вызвать to_string().

Пример использования

```rust
use itertools::Itertools; // Импортируем трейт для доступа к методу

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // Создаем объект-форматтер с разделителем ", "
    let formatted = data.iter().format(", ");

    // Данные преобразуются в строку только в момент вывода
    println!("Список: [{}]", formatted); 
    // Вывод: Список: [1, 2, 3, 4, 5]
}
```

------------------------------------------------------------------------
                        3. Itertools::Itertools::format

Метод .format_with() из библиотеки itertools — это продвинутый инструмент
для ленивого форматирования элементов итератора. В отличие от простого 
.format(), он позволяет настраивать отображение каждого отдельного 
элемента с помощью замыкания.

1. Как это работает
Вместо того чтобы выделять память под новую строку для каждого элемента, 
.format_with() использует механизм std::fmt::Arguments. Это позволяет 
форматировать данные напрямую в итоговый буфер (например, в консоль) в 
режиме Zero-allocation (без аллокаций в куче для промежуточных значений).

2. Пример использования
Чтобы использовать метод, необходимо импортировать трейт Itertools.

```rust
use itertools::Itertools;

fn main() {
    let data = vec![10, 20, 30];

    // ", " — разделитель между элементами
    // замыкание |item, f| определяет формат каждого элемента
    let formatted = data.iter().format_with(", ", |item, f| {
        // Мы оборачиваем каждое число в префикс и суффикс
        f(&format_args!("ID: {:03}", item))
    });

    // Форматирование происходит только сейчас, напрямую в stdout
    println!("Users: [{}]", formatted); 
    // Вывод: Users: [ID: 010, ID: 020, ID: 030]
}
```

a) format_args!: Это встроенный макрос Rust, который подготавливает данные 
        для вывода, но не превращает их в строку. Он создает структуру, 
        которая хранит ссылки на данные и инструкции по их форматированию. 
        Это происходит максимально быстро и без выделения памяти в куче.
b) f(...): Мы передаем эти инструкции в форматтер. В данном случае мы 
        говорим: вывести отформатированное число в формате: «"ID: {:03}"».
c) item: Это элемент для форматирования.


Почему этот код хорош:
- Нулевые аллокации (Zero-allocation): Если бы вы использовали 
    format!("ID: {:03}", item)) внутри замыкания, вы бы создавали новую 
    String для каждого числа. Код выше не создает временных строк вообще.
- Эффективность памяти: Весь процесс потребляет константное количество 
    памяти, независимо от того, 3 элемента в векторе или 3 миллиона.
- Гибкость: Вы можете реализовать сколь угодно сложную логику отображения
    (например, менять формат в зависимости от значения числа), и она 
    будет работать максимально быстро.

------------------------------------------------------------------------
                            4. std::fmt::Formatter

В Rust std::fmt::Formatter — это конфигурационный объект, который 
управляет тем, как данные записываются в выходной поток 
(строку, консоль или файл).

Вы никогда не создаете Formatter вручную. Он передается вам системой в 
качестве аргумента, когда вы реализуете трейты форматирования, 
такие как Display или Debug.

Зачем он нужен?
Formatter выполняет две ключевые функции:
1. Предоставляет буфер записи: Он инкапсулирует место, куда должны быть 
    записаны байты (например, через макрос write!).
2. Хранит настройки форматирования: В нем содержатся параметры, указанные
    в строке формата (ширина, точность, выравнивание, заполнение 
    символами).

    Как он работает в коде?
Чаще всего вы сталкиваетесь с ним при реализации трейта std::fmt::Display:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // `f` — это и есть наш Formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Мы используем макрос write!, чтобы передать данные в Formatter
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Ключевые возможности Formatter
Через объект f вы можете получить доступ к параметрам, которые 
    пользователь указал в println!:
- Ширина и выравнивание: Если вызвано println!("{:10}", point), внутри 
    fmt через f.width() можно узнать, что нужно заполнить пространство 
    до 10 символов.
- Отладка структур: Formatter предоставляет вспомогательные методы для 
    красивого вывода структур и списков, например f.debug_struct 
    или f.debug_list.

Почему это эффективно?
- Отсутствие промежуточных строк: Данные записываются напрямую в итоговый
    буфер. Например, если вы печатаете структуру в консоль, Formatter 
    пишет байты сразу в стандартный вывод, не создавая временную строку 
    String в куче.
- Zero-cost: Все настройки форматирования разбираются макросами 
    (вроде format_args!) еще на этапе компиляции, а Formatter в рантайме
    просто выполняет эти инструкции.

    Итог:
std::fmt::Formatter — это «умный посредник» между вашим типом данных и 
местом их вывода. Он берет на себя всю грязную работу по соблюдению 
отступов, точности чисел и управлению буфером, позволяя вам 
сосредоточиться только на том, что именно нужно напечатать. 
Официальная документация std::fmt::Formatter.


*/

//! Расширение trait для [`Iterator`].
//!
//! Украден из [`itertools` crate][0].
//!
//! [0]: https://docs.rs/itertools/latest/src/itertools/lib.rs.html#2078-2136

use std::fmt;

use self::format::{Format, FormatWith};

// приватный модуль (видет только в текущем файле), имя модуля `private`
mod private {

    // публичный trait Sealed
    pub trait Sealed {}

    // реализация Sealed для типажа I (что реализует Iterator)
    impl<I: Iterator> Sealed for I {}
}

/// расширение trait для [`Iterator`].
pub trait MyIteratorExt:
                Iterator
                // добавлен ещё один супер трейт: Sealed из приватного модуля private
                + private::Sealed
{
    /// Форматировать все итерационные элементы, разделённые по `sep`.
    ///
    /// Все элементы являются отформатированными (любым formatting trait)
    /// c `sep` вставленным между каждым элементом.
    ///
    /// **Panics** если formatter помошник является отформатированным более чем один раз.
    ///
    /// ```rust
    /// use step_2_6::MyIteratorExt as _;
    /// //* Ошибка компиляции: module `private` is private
    /// use step_2_6::private ;
    /// // */
    /// let data = [1.1, 2.71828, -3.];
    /// assert_eq!(
    ///     format!("{:.2}", data.iter().format(", ")),
    ///            "1.10, 2.72, -3.00");
    /// ```
    fn format(self, sep: &str) -> Format<Self>
    where
        Self: Sized,
    {
        format::new_format_default(self, sep)
    }

    /// Форматировать все итерационные элементы, разделённые по `sep`.
    ///
    /// Это является настраиваемой версией [`.format()`](MyIteratorExt::format).
    ///
    /// Поставленное замыкание `format` является вызванным один раз для каждого итерационного элемента,
    /// with two arguments: the element and a callback that takes a
    /// с двумя аргументами: элемент и callback что принимает 
    /// `&Display` значение т.е. люьая ссылка к типу что реализует `Display`.
    ///
    /// Использование `&format_args!(...)` является наиболее универсальным путём чтобы применить специализированные
    /// элементы форматирования. callback может быть вызван мноэество раз если необъодимо.
    ///
    /// **Panics** если formatter помошник является отформатированным более чем один раз.
    ///
    /// ```rust
    /// use step_2_6::MyIteratorExt as _;
    /// //* Ошибка компиляции: module `private` is private
    /// use step_2_6::private ;
    /// // */
    ///
    /// let data = [1.1, 2.71828, -3.];
    /// let data_formatter = data.iter().format_with(", ", |elt, f| f(&format_args!("{:.2}", elt)));
    /// assert_eq!(format!("{}", data_formatter),
    ///            "1.10, 2.72, -3.00");
    ///
    /// // .format_with() is recursively composable
    /// let matrix = [[1., 2., 3.],
    ///               [4., 5., 6.]];
    /// let matrix_formatter = matrix.iter().format_with("\n", |row, f| {
    ///     f(&row.iter().format_with(", ", |elt, g| g(&elt)))
    /// });
    /// assert_eq!(matrix_formatter.to_string(), "1, 2, 3\n4, 5, 6");
    /// ```
    fn format_with<F>(self, sep: &str, format: F) -> FormatWith<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item, &mut dyn FnMut(&dyn fmt::Display) -> fmt::Result) -> fmt::Result,
    {
        format::new_format(self, sep, format)
    }
}

/// покрывающая реализация MyIteratorExt для типажа I (что реализует Iterator), покрывающая реализация
impl<I: Iterator> MyIteratorExt for I {}

// модуль format
mod format {
    use std::{
            cell::RefCell,  // внутренне заимствование
            fmt // Утилиты для форматирования и печати строк.
        };

    /// Структура форматировать с [помощью]
    /// Форматирует все итерационные элементы лениво, разделённые по `sep`.
    ///
    /// Значение форматирования может быть отформатированным однажды,
    /// послетого итератор является исчерпанным
    ///
    /// Смотри [`.format_with()`](crate::MyIteratorExt::format_with) для большей информации.
    #[derive(Clone)]
    pub struct FormatWith<'a, I, F> {
        /// сепаратор
        sep:    &'a str,
        /// внутренние данные Option<(итератор, функция форматирования)> обёрнутые в RefCell
        /// FormatWith использует внутренную изменчивость потому что Display::fmt принримает &self
        inner:  RefCell<Option<(I, F)>>,
    }

    /// Структура форматер по умолчанию
    /// Форматировать все итерационные элементы лениво, разделённые по `sep`.
    ///
    /// Значение форматирования может быть отформатированным однажды, 
    /// после этого итератор является исчерпанным
    ///
    /// See [`.format()`](crate::MyIteratorExt::format)
    /// Смотри [`.format()`](crate::MyIteratorExt::format)
    /// для большей информации.
    #[derive(Clone)]
    pub struct Format<'a, I> {
        /// сепаратор
        sep: &'a str,
        /// внутренние данные Option<Итератор> обёрнутые в RefCell
        /// Format использует внутренную изменчивость потому что Display::fmt принимает &self.
        inner: RefCell<Option<I>>,
    }

    /// новое форматирование с функцией (замыканием)
    pub fn new_format<I, F>(iter: I, separator: &str, f: F) -> FormatWith<'_, I, F>
    where
        I:  Iterator,   // итератор
        F:  FnMut(  // изменяемое замыкание
                I::Item,    // Item итератора
                &mut dyn FnMut(     // изменяемое замыкание для форматирования
                    &dyn fmt::Display
                ) -> fmt::Result    // возвращает результат форматирования
            ) -> fmt::Result,   // возвращает результат форматирования
    {
        // создаёт структуру `форматировать с (FormatWith)`
        FormatWith {
            sep:    separator,  // сепаратор
            inner:  RefCell::new(Some((iter, f))),  // внутренние данные
        }
    }

    /// новое форматирование по умолчанию, без специальной функции форматирования (замыкания)
    pub fn new_format_default<I>(iter: I, separator: &str) -> Format<'_, I>
    where
        I: Iterator,    // итератор
    {
        // создаёт структуру `форматирование` (Format)
        Format {
            sep:    separator,  // сепаратор
            inner:  RefCell::new(Some(iter)),   // внутренние данные
        }
    }

    // реализация Display для структуры FormatWith (форматирование с [помощью])
    impl<'a, I, F> fmt::Display for FormatWith<'a, I, F>
    where
        I: Iterator,
        F: FnMut(
                I::Item,
                &mut dyn FnMut(&dyn fmt::Display) -> fmt::Result
            ) -> fmt::Result,
    {
        // создаём метод форматирования
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // получить итератор и форматер из self.inner
            let (mut iter, mut format) 
                    = match self
                                .inner
                                .borrow_mut()
                                .take() // Вытаскивает значение Option, оставляя None на его месте.
            {
                Some(t) => t,   // именно t деструктуируется в коретежные значения (mut iter, mut format) 
                None => panic!("FormatWith: was already formatted once"),
            };
            // ^^^ после этой операции self.inner == None

            if let Some(fst) = iter.next() {
                format(fst, &mut |disp: &dyn fmt::Display| disp.fmt(f))?;
                iter.try_for_each(|elt| {
                    if !self.sep.is_empty() {
                        f.write_str(self.sep)?; // Записать данные в лежащий в основе буфер содержащийся в этом форматере.
                    }
                    format(elt, &mut |disp: &dyn fmt::Display| disp.fmt(f))
                })?;
            }
            Ok(())
        }
    }

    /// реализация для структуры Format (форматирование по умолчанию).
    impl<'a, I> Format<'a, I>
    where
        I: Iterator,    // итератор
    {
        fn format<F>(&self, f: &mut fmt::Formatter, mut cb: F) -> fmt::Result
        where
            F: FnMut(&I::Item, &mut fmt::Formatter) -> fmt::Result,
        {
            let mut iter = match self
                                        .inner
                                        .borrow_mut()
                                        .take() // Вытаскивает значение Takes Option, оставляя None на его месте.
            {
                Some(t) => t,   // итератор из Option
                None => panic!("Format: was already formatted once"),
            };

            if let Some(fst) = iter.next() {
                cb(&fst, f)?;
                iter.try_for_each(|elt| {
                    if !self.sep.is_empty() {
                        f.write_str(self.sep)?;
                    }
                    cb(&elt, f)
                })?;
            }
            Ok(())
        }
    }

    macro_rules! impl_format {
        ($($fmt_trait:ident)*) => {
            $(
                impl<'a, I> fmt::$fmt_trait for Format<'a, I>
                where
                    I: Iterator,
                    I::Item: fmt::$fmt_trait,
                {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        self.format(f, fmt::$fmt_trait::fmt)
                    }
                }
            )*
        }
    }

    impl_format! {
        Display Debug UpperExp LowerExp UpperHex LowerHex Octal Binary Pointer
    }
}