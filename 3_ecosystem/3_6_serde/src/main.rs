use serde::{
        Deserialize,    // Структура данных, которую можно десериализовать из любого формата данных, поддерживаемого Serde.
        Serialize       // Структура данных, которую можно сериализовать в любой формат данных, поддерживаемый Serde.
    };

use serde_json ;    // JSON — это широко распространенный открытый стандартный формат, использующий удобочитаемый текст для передачи объектов данных, состоящих из пар «ключ-значение».

/// Структура точки
#[derive(
    Debug,
    Deserialize,
    Serialize
)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Сериализуйте заданную структуру данных в виде строки JSON.
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    // Десериализовать экземпляр типа T из строки JSON-текста.
    let deserialized /*: Point  */ = serde_json::from_str::<Point>(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
/*
fn main() {
    println!("Implement me!");
}
 */
