//use std::collections::HashMap;
//use std::fmt;
//use thiserror::Error; // Для удобных ошибок, можно заменить на ручную реализацию


/// перечень допустимых монет
#[derive(Clone, Copy)]
enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}


impl Coin {

    fn value(&self) ->u32 {
        *self as u32    // необъодим traits Clone, Copy
    }

}



/*
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
 */

fn main() {
    println!("Implement me!");
}
