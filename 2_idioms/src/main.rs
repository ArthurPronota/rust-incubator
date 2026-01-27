use core::error;
use std::collections::HashMap;
//use std::collections::HashMap;
use std::{fmt};
//use thiserror::Error; // Для удобных ошибок, можно заменить на ручную реализацию
//use std::path::Display;
// use std::num::NonZeroU32 ;
// use core::error;
use thiserror::Error;   // для  #[error(...)]

/// Ошибки торгового автомата
#[derive(Error, Debug)]
enum VendingError {
    #[error("The price must be greater than zero")]
    ZeroPrice,

    #[error("The product name is empty")]
    EmptyProductionName,

    #[error("The capacity of the vending machine must be greater than zero")]
    CapacityMachineZero,

}


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
        *self as u32    // необходимы traits: Clone, Copy
    }
}

// реализация Display для Coin
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
    /// создание нового PriceType
    pub fn new(value: impl Into<u32>) -> Result<Self, VendingError> {
        match value.into() {
            v if v > 0 => Ok(Self(v)),
            _ => Err(VendingError::ZeroPrice)
        }
    }

    /// получение значения из PriceType
    pub fn value(&self) -> u32 {
        self.0
    }

    /// проверка значения PriceType
    pub fn check(&self) ->Result<(), VendingError> {
        match self.value() {
            v if v > 0 => Ok(()),
            _ => Err(VendingError::ZeroPrice)
        }
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
    /// создание нового CapacityType
    pub fn new(value: impl Into<u32>) -> Result<Self, VendingError> {
        match value.into() {
            v if v > 0 => Ok(Self(v)),
            _ => Err(VendingError::ZeroPrice),
        }
    }

    /// получить значение из CapacityType
    pub fn value(&self) ->u32 {
        self.0
    }

    /// проверка значения CapacityType
    pub fn check(&self) ->Result<(), VendingError> {
        match self.value() {
            v if v > 0 => Ok(()),
            _ => Err(VendingError::CapacityMachineZero),
        }
    }
}

// тип количества продукции
struct QuantityProdType(u32) ;

// реализация методов для QuantityProdType
impl QuantityProdType {
    /// получить реальное количество продукции
    pub fn value(&self) ->u32 {
        self.0
    }
}

// тип количества монет
struct QuantityCoinType(u32) ;

// реализация методов для QuantityCoinType
impl QuantityCoinType {
    /// получить реальное кол-во Coin
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// продукт (struct)
struct Product {
    name:   String,
    price:  PriceType,
}

// реализация методов для Product
impl Product {
    
    /// создание нового Product
    fn new(name: impl Into<String>, price: PriceType) ->Result<Self, VendingError> {
        if price.value() <= 0 {
            return Err(VendingError::ZeroPrice) ;
        }
        
        let name_val = name.into() ;  // Конвертирует этот тип в (обычно выведенный) входной тип.
        if name_val.len() == 0 {
            return Err(VendingError::EmptyProductionName) ;
        }

        Ok(
            Self { 
                name: name_val,
                price 
            }
        )
    }

    /// проверка содержимого Product
    pub fn check(&self) ->Result<(), VendingError> {
        match self.name.len() {
            v_len if v_len != 0 => self.price.check(),
            _ => Err(VendingError::EmptyProductionName)
        }
    }

    /// получение имени Product
    pub fn name(&self) -> &str {
        &self.name
    }

    /// получение реальной цены
    pub fn price(&self) ->u32 {
        self.price.value()
    }

}

// реализация Display для Product
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}.{}$", 
                self.name(), 
                self.price() / 100,
                self.price() % 100,
        )
    }
}



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
    pub fn new(capacity: CapacityType) -> Result<Self, VendingError> {
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
            _ => Err(VendingError::CapacityMachineZero),
        }
    }
    
}

fn main() {
    let c = Coin::Fifty ;
    println!("c: {}", c) ;

    let v1 = 0 ;
    //let v = NonZeroU32::new(v1) ;
    //println!("{:?}", v) ;

    //println!("{:?}, {:?}", PriceType::new(0), PriceType(0).0) ;
    let v = PriceType::new(10i64 as u32) ;

}

