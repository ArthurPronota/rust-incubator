pub mod my_error;
pub mod my_iterator_ext;

pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};

// --------------------------------

/// приватный модуль `private`
mod private {
    /// публичный трейт Sealed
    pub trait Sealed {}
}

/// запечатанный trait SealedTrait с супертрейтом private::Sealed 
pub trait SealedTrait : private::Sealed {
    fn method(&self);
}

/// структура для реализации запечатанного трейта SealedTrait
pub struct TypeThatImplsSealed;

/// реализация trait Sealed для структуры TypeThatImplsSealed
impl private::Sealed for TypeThatImplsSealed {}

/// реализация SealedTrait для структуры TypeThatImplsSealed
impl SealedTrait for TypeThatImplsSealed {
    fn method(&self) {}
}

