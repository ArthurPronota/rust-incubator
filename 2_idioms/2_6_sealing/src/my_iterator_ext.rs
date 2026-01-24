/*
    Замыкание, реализующее FnMut, может изменять значения переменных, 
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


*/

//! Extension trait for an [`Iterator`].
//!
//! Stolen from [`itertools` crate][0].
//!
//! [0]: https://docs.rs/itertools/latest/src/itertools/lib.rs.html#2078-2136

use std::fmt;

use self::format::{Format, FormatWith};

/// расширение trait для [`Iterator`].
pub trait MyIteratorExt: Iterator {
    /// Format all iterator elements, separated by `sep`.
    ///
    /// All elements are formatted (any formatting trait)
    /// with `sep` inserted between each element.
    ///
    /// **Panics** if the formatter helper is formatted more than once.
    ///
    /// ```rust
    /// use step_2_6::MyIteratorExt as _;
    ///
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

    /// Format all iterator elements, separated by `sep`.
    ///
    /// This is a customizable version of [`.format()`](MyIteratorExt::format).
    ///
    /// The supplied closure `format` is called once per iterator element,
    /// with two arguments: the element and a callback that takes a
    /// `&Display` value, i.e. any reference to type that implements `Display`.
    ///
    /// Using `&format_args!(...)` is the most versatile way to apply custom
    /// element formatting. The callback can be called multiple times if needed.
    ///
    /// **Panics** if the formatter helper is formatted more than once.
    ///
    /// ```rust
    /// use step_2_6::MyIteratorExt as _;
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

impl<T> MyIteratorExt for T where T: Iterator {}

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
