use std::borrow::Cow ;
use std::marker::PhantomData ;

struct MyPtr<'a, T> {
    ptr: *const u8, // Компилятор выдаст ошибку: 'a и T не используются
    _marker: PhantomData<&'a T>,
}

fn main() {
    println!("Implement me!");
}
