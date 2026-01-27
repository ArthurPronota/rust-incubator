use std::collections::HashMap;
//use std::collections::HashMap;
use std::{fmt};
//use thiserror::Error; // Для удобных ошибок, можно заменить на ручную реализацию
//use std::path::Display;
use std::num::NonZeroU32 ;

/// перечень допустимых монет
#[derive(Clone, Copy, Debug)]
enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

/// реализация методов для Coin
impl Coin {
    /// получить значение Coin
    pub fn value(&self) ->u32 {
        *self as u32    // необходимы traites: Clone, Copy
    }
}

// пеализация Display для Coin
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} {}", 
            self.value(),
            match self.value() {
                1 => "cent",
                _ => "cents",
            }
        )
    }
}

// цена (cents)
struct PriceType(u32) ;

// реализация методов для PriceType
impl PriceType {
    pub fn new(value: impl Into<u32>) -> Result<Self, String> {
        match value.into() {
            v if v > 0 => Ok(Self(v)),
            _ => Err("Цена должна быть больше нуля".to_string())
        }
    }

    pub fn value(&self) -> u32 {
        self.0
    }

}
/*
impl From<u32> for Price {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
 */

// вместимость торгового автомата
struct CapacityType(u32) ;

// реализация методов для PriceType
impl CapacityType {
    pub fn new(value: impl Into<u32>) -> Result<Self, String> {
        match value.into() {
            v if v > 0 => Ok(Self(v)),
            _ => Err("Цена должна быть больше нуля".to_string()),
        }
    }
}

// тип количества продукции
struct QuantityProdType(u32) ;

// тип количества монет
struct QuantityCoinType(u32) ;

/// продукт (struct)
struct Product {
    name:   String,
    price:  PriceType,
}

// реализация методов для Product
impl Product {
    /// создание нового Product
    fn new(name: impl Into<String>, price: PriceType) ->Result<Self, String> {
        if price.0 <= 0 {
            return Err("Цена должна быть больше нуля".to_string()) ;
        }
        
        let name_val = name.into() ;  // Конвертирует этот тип в (обычно выведенный) входной тип.
        if name_val.len() == 0 {
            return Err("наименование продукции пусто".to_string()) ;
        }

        Ok(
            Self { 
                name: name_val,
                price 
            }
        )
    }
}

// реализация Display для Product
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}.{}$", 
                self.name, 
                self.price.value() / 100,
                self.price.value() % 100,
        )
    }
}


//quantity 

/// Торговый автомат
struct VendingMachine {
    products:       HashMap<String, (Product, QuantityProdType)>,   // продукция в автомате
    coins:          HashMap<Coin, QuantityCoinType>, // количество монет разного номинала в автомате
    capacity:       CapacityType,    // вместимость торгового автомата (штуки)
    inserted_coins: HashMap<Coin, QuantityCoinType>,    // монеты внесённые за покупку
}

// реализация методов торгового автоиата
impl VendingMachine {
    /// создание нового автомата
    pub fn new(capacity: CapacityType) -> Result<Self, String> {
        match capacity.0 {  // проверка вместимости автомата
            c if c > 0 => 
                Ok(
                    Self { 
                        products: HashMap::new(),
                        coins: HashMap::new(),
                        capacity, 
                        inserted_coins: HashMap::new(),
                    }
                ),
            _ => Err("Вместимость торгового автоиата должна быть больше 0".to_owned()),
        }
    }
    
}

fn main() {
    let c = Coin::Fifty ;
    println!("c: {}", c) ;

    let v1 = 0 ;
    let v = NonZeroU32::new(v1) ;

    println!("{:?}", v) ;

    //println!("{:?}, {:?}", PriceType::new(0), PriceType(0).0) ;
    let v = PriceType::new(10i64 as u32) ;

}

