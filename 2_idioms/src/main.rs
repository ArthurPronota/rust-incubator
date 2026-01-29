use std::collections::HashMap;
use std::ops::Add;
//use std::collections::HashMap;
use std::fmt;
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

    #[error("The amount of product added must be greater than 0")]
    ZeroAddProd,

    #[error("The number of coins added must be greater than 0")]
    ZeroAddCoins,

    #[error("The vending machine is full and cannot be loaded with new products.")]
    MachineFull,

    #[error("Overflow when loading a vending machine with: `{0}` units: {1}")]
    MachineOweflow(String, u32),

}


/// перечень допустимых монет
#[derive(Clone, Copy, Debug)]
#[derive(Eq, Hash, PartialEq)]
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

impl fmt::Display for PriceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "price: {}.{:02}$", 
            self.value() / 100, 
            self.value() % 100
        )
    }
}


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

/// реализация Display для CapacityType
impl fmt::Display for CapacityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Capacity: {}", self.value())
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

    /// проверка количества добавляемого продукта
    pub fn check_add_quant(&self) ->Result<(), VendingError> {
        match self.value() {
            q if q > 0 => Ok(()),
            _ => Err(VendingError::ZeroAddProd)
        }
    }
}

/// реализация сложения u32 + QuantityProdType
impl Add<QuantityProdType> for u32 {
    type Output = QuantityProdType;

    fn add(self, rhs: QuantityProdType) -> Self::Output {
        QuantityProdType(self + rhs.0)
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

    /// проверка количества добавленых монет
    pub fn check_add(&self) ->Result<(), VendingError> {
        match self.0 {
            q if q > 0 => Ok(()),
            _ => Err(VendingError::ZeroAddCoins),
        }
    }
}

// реализация сложения u32 + QuantityCoinType
impl Add<QuantityCoinType> for u32 {
    type Output = QuantityCoinType;
    fn add(self, rhs: QuantityCoinType) -> Self::Output {
        QuantityCoinType(self + rhs.0)
    }
}

/// продукт (struct)
#[derive(Eq, Hash, PartialEq)]
struct Product(String) ;

// реализация методов Product
impl Product {
    /// создать новый Product
    pub fn new(val: impl Into<String>) ->Result<Self, VendingError> {
        match val.into() {
            v if v.len() > 0 => Ok(Self(v)),
            _ => Err(VendingError::EmptyProductionName),
        }
    }

    // получить реальное наименование Product
    pub fn name(&self) ->&str {
        &self.0
    }

    /// проверка внутренних параметров Product
    pub fn check(&self) ->Result<(), VendingError> {
        match self.name().len() {
            len if len > 0 => Ok(()),
            _ => Err(VendingError::EmptyProductionName),
        } 
    }

}

/*
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
 */

// реализация Display для Product
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
                self.name(), 
        )
    }
}

/// Торговый автомат
struct VendingMachine {
    products:       HashMap<Product, (PriceType, QuantityProdType)>,   // продукция в автомате
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
    
    /// определить capacity автомата
    pub fn define_capacity(&mut self, cap: CapacityType) ->Result<(), VendingError> {
        
        // проверка нового capacity
        if let Err(err) = cap.check() {
            return Err(err);
        }

        self.capacity = cap ;
        Ok(())    
    }

    /// загрузка продукции с установкой цены
    pub fn add_product(
                &mut self,
                prod: Product,
                price: PriceType,
                quant_prod: QuantityProdType
            ) ->Result<(), VendingError> {

        // проверка параметров продукта
        if let Err(err) = prod.check() {
            return Err(err);
        }

        // проверка цены продукции
        if let Err(err) = price.check() {
            return Err(err);
        }

        // проверка количества продукции
        if let Err(err) = quant_prod.check_add_quant() {
            return Err(err);
        }

        // контроль возможности загрузки автомата
        let tot_prods = self.total_products() ;
        if tot_prods >= self.capacity.value() {
            return Err(VendingError::MachineFull)  ;
        } else if quant_prod.value() + tot_prods > self.capacity.value() {
            return Err(VendingError::MachineOweflow(prod.name().to_owned(), quant_prod.value()));
        }

        match self.products.get(&prod) {
            Some(prod_exists) => {
                self.products.insert(
                                prod, 
                                (price, prod_exists.1.0 + quant_prod)
                            ) ;
            },
            None => {
                self.products.insert(
                                prod,
                                (price, quant_prod)
                            ) ;
            },
        }

        Ok(())
    }

    /// проверка наличия продукта
    pub fn has_product(&self, prod: &Product) -> bool {
        self
            .products
            .get(&prod)
            .map_or(
                false,
                |p| p.1.value() > 0
            )
    }

    /// начальная загрузка монет в автомат
    pub fn load_coins(
                &mut self,
                coins: HashMap<Coin, QuantityCoinType>
            ) ->Result<(), VendingError> {

        for (coin, quantity) in coins {
            // проверка количества добавляемых монет
            if let Err(err) = quantity.check_add() {
                return Err(err);
            }

            match self.coins.get(&coin) {
                Some(count_exists) => {
                    self.coins.insert(coin, count_exists.0 + quantity) ;
                },
                None => {
                    self.coins.insert(coin, quantity) ;
                },
            }
        }

        Ok(())
    }

    /// добавить монету в приёмник монет
    pub fn insert_coin(&mut self, coin: Coin) /* -> Result<(), VendingError> */ {
        self
            .coins
            .entry(coin)
            .and_modify(|x| {
                *x = 1 + QuantityCoinType(x.0) ;
            })
            .or_insert(QuantityCoinType(1)) ;
    }

    /// получить сумму (cents) внесённую за покупку
    pub fn total_inserted(&self) ->u32 {
        self
            .inserted_coins
            .iter()
            .map(|x| {
                x.0.value() * x.1.value()
            })
            .sum()
    }

    /// получить количество продукции в автомате
    pub fn total_products(&self) -> u32 {
        self
            .products
            .iter()
            .map(|x| 
                x.1.1.value()
            )
            .sum()
    }

    /// очистка вставленных монет
    pub fn clear_inserted_coins(&mut self) {
        self.inserted_coins = HashMap::new() ;
    }
  

}

// реализация Display для VendingMachine
impl fmt::Display for VendingMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "******** Vending Machine ********")? ;
        writeln!(f, "{} products", self.capacity)? ;
        writeln!(f, "\nProducts:")? ;

        for (prod, (price, quants)) in &self.products {
            if quants.value() > 0 {
                writeln!(f, "\tproduct: {}, {}, ({} left)", 
                    prod.name(),
                    price,
                    quants.value()
                )? ;
            }
        }
        writeln!(f, "Total products: {}", self.total_products())?;

        writeln!(f, "\nCoins in machine")?;
        let mut tot_sum = 0u32 ;
        for (coin, quants) in &self.coins {
            writeln!(f, "\t{}: {}", coin, quants.value())?;
            tot_sum += quants.value() ;
        }
        writeln!(f, "Total amount in the vending machine: {}.{:02}$", 
            tot_sum / 100, 
            tot_sum % 100,
        )? ;

        // вывод данных о вставленных за продукцию монет
        if self.total_inserted() > 0 {
            writeln!(f, "\nInserted coins:")? ;
            let mut tot_amount = 0u32 ;
            for (coin, quants) in &self.inserted_coins {
                writeln!(f, "\t{}: {}", coin, quants.value())? ;
                tot_amount += quants.value() ;
            }
            writeln!(f, "Total inserted: {}.{:02}$", 
                tot_amount / 100,
                tot_amount % 100,
            )?;
        }

        writeln!(f, "*********************************\n")? ;

        Ok(())
    }
}


fn main() {

    // создание автомата
    let mut vm: VendingMachine ;
    match VendingMachine::new(
                    CapacityType(30)
                  ) 
    {
        Ok(v) => vm = v,
        Err(err) => panic!("{}", err),
    }

    // загрузка Coca-Cola
    if let Err(err) = 
                vm.add_product(
                    Product(String::from("Pepsi-Cola")),
                    PriceType(146),
                    QuantityProdType(20),
                ) {
        panic!("{}", err) ;
    }

    // загрузка Coca-Cola
    if let Err(err) = vm.add_product(
            Product::new("Coca-Cola").unwrap(),
            PriceType(145),
            QuantityProdType(10),
        ) {
        panic!("{}", err) ;
    }

    // загрузка монет для сдачи
    if let Err(err) = vm.load_coins(
            HashMap::from([
                (Coin::One, QuantityCoinType(20)),
                (Coin::Two, QuantityCoinType(15)),
                (Coin::Five, QuantityCoinType(19)),
                (Coin::Ten, QuantityCoinType(11)),
                (Coin::Twenty, QuantityCoinType(14)),
                (Coin::Fifty, QuantityCoinType(12)),
            ])
        ) {
        panic!("{}", err) ;
    }

    println!("{}", vm) ;
}

