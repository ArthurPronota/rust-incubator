/// Базовые ожидания для значений ошибок.
///
/// Упрощенна версия [`std::error::Error`].
use std::{
    any::TypeId,    // Тип прередставляющий глобальный уникальный идентификатор типа.
    fmt::{Debug, Display},
};

mod private {
    pub struct Token;
}

/// Базовые ожидания для значений ошибок.
pub trait MyError: Debug
                   + Display
                   //+ private::SealedTypeId // добавлен трейт SealedTypeId из модуля private
{
    /// Низкоуровневый источник этой ошибки (ошибки на более низком уровне), если есть.
    /// 
    ///
    /// # Примеры
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::any::TypeId ;
    ///
    /// use step_2_6::MyError;
    /// //* Ошибка компиляции: module `private` is private
    /// use step_2_6::private ;
    /// // */
    ///
    /// #[derive(Debug)]
    /// struct SuperError {
    ///     source: SuperErrorSideKick,
    /// }
    /// 
    /// #[derive(Debug)]
    /// struct SuperErrorSideKick;     
    ///
    /// impl fmt::Display for SuperError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "SuperError is here!")
    ///     }
    /// }
    ///
    /// impl fmt::Display for SuperErrorSideKick {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "SuperErrorSideKick is here!")
    ///     }
    /// }
    ///      
    /// impl MyError for SuperError {
    ///     fn source(&self) -> Option<&(dyn MyError + 'static)> {
    ///         Some(&self.source)
    ///     }
    ///     //* Ошибка: использование private::Token
    ///     #[doc(hidden)]
    ///     fn type_id(&self, _: private::Token) -> TypeId
    ///         where
    ///             Self: 'static,
    ///     {
    ///         TypeId::of::<Self>()
    ///     }
    ///     // */
    /// }
    ///
    /// // реализации по умолчанию
    /// impl MyError for SuperErrorSideKick {}
    ///
    /// fn get_super_error() -> Result<(), SuperError> {
    ///     Err(SuperError { source: SuperErrorSideKick })
    /// }
    ///
    /// fn main() {
    ///     match get_super_error() {
    ///         Err(e) => {
    ///             println!("Error: {e}");
    ///             println!("Caused by: {}", e.source().unwrap());
    ///         }
    ///         _ => println!("No error"),
    ///     }
    /// }
    /// ```
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        None
    }

    /// Gets the `TypeId` of `self`.
    ///
    /// __This is memory-unsafe to override in user code.__
    #[doc(hidden)]
    fn type_id(&self,
               //* Ошибка: при переопределении метода
               _: private::Token    // запечатывание метода трейта
               // */
            ) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }

}

/// пример реализации источника ошибки
impl<'a, T: MyError + ?Sized> MyError for &'a T {
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        MyError::source(&**self)
    }
}
