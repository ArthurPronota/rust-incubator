pub mod my_error;
pub mod my_iterator_ext;

pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};

// --------------------------------

mod private {
    pub trait Sealed {}
}

pub trait SealedTrait : private::Sealed {
    fn method(&self);
}

pub struct TypeThatImplsSealed;

impl private::Sealed for TypeThatImplsSealed {}

impl SealedTrait for TypeThatImplsSealed {
    fn method(&self) {}
}
