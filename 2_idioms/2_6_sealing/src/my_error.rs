/// Базовые ожидания для значений ошибок.
///
/// Упрощенна версия [`std::error::Error`].
use std::{
    any::TypeId,    // Тип прередставляющий глобальный уникальный идентификатор типа.
    fmt::{Debug, Display},
};

// приватный модуль с именем private
mod private {
    use std::any::TypeId ;

    // публичный трейт SealedTypeId с методом type_id(...) скрытый из документации
    pub trait SealedTypeId {
        /// Gets the `TypeId` of `self`.
        #[doc(hidden)]
        fn type_id(&self) -> TypeId
            where
                Self: 'static ;
    }
}

/// Базовые ожидания для значений ошибок.
pub trait MyError: Debug + 
                   Display + 
                   private::SealedTypeId // добавлен трейт SealedTypeId из модуля private
{
    /// Низкоуровневый источник этой ошибки (ошибки на более низком уровне), если есть.
    /// 
    ///
    /// # Примеры
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// use step_2_6::MyError;
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
    ///             println!("Error: {e}, type_id: {:?}", e.type_id());
    ///             println!("Caused by: {}, type_id: {:?}", e.source().unwrap(), e.source().unwrap().type_id());
    ///             println!("SuperErrorSideKick: {:?}", e.source.source()) ;
    ///         }
    ///         _ => println!("No error"),
    ///     }
    /// }
    /// ```
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        None
    }

    /*
    /// Gets the `TypeId` of `self`.
    ///
    /// __This is memory-unsafe to override in user code.__
    #[doc(hidden)]
    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
     */
}

/// пример реазлизации запечатанного (sealed) метода
impl<T: ?Sized> private::SealedTypeId for T 
{
    #[doc(hidden)]
    fn type_id(&self) -> TypeId
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
