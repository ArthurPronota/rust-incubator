/*
  Примеры сериализации и десериализации.
  Настройки из Cargo.toml важны для работы прогоаммы.
*/

use serde::{
        Serialize,  // Структура данных, которую можно десериализовать из любого формата данных, поддерживаемого Serde.
        Deserialize // Структура данных, которую можно сериализовать в любой формат данных, поддерживаемый Serde.
    } ;

use uuid::Uuid ;    // UUID может быть отформатирован одним из нескольких способов.

use serde_json ;    // JSON — это широко распространенный открытый стандартный формат, использующий удобочитаемый текст для передачи объектов данных, состоящих из пар «ключ-значение».

use chrono::{
        DateTime,
        Utc
    } ;

use toml ;

/// Тип запроса
/*
Для сериализации в Cargo.toml добавляем:
[dependencies]
serde = { version = "1.0.228", features = ["derive"] }
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum TypeRequest {
    /// Успешный запрос
    #[serde(rename = "success")]
    Success,
    /// Ошибка запроса
    #[serde(rename = "error")]
    Error,
}    

/// Публичный тариф
/*
Для сериализации в Cargo.toml добавляем:
[dependencies]
serde = { version = "1.0.228", features = ["derive"] }
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PublicTariff {
    id:             u32,
    price:          u32,
    duration:       String,
    description:    String,
}

/// Приватный тариф
/*
Для сериализации в Cargo.toml добавляем:
[dependencies]
serde = { version = "1.0.228", features = ["derive"] }
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PrivateTariff {
    #[serde(rename = "client_price")]
    client_price:   u32,    // В Rust: client_price; в JSON: "client_price", без #[serde(rename = "client_price")] будет clientPrice
    duration:       String,
    description:    String,
}

/// Поток
/*
Для сериализации Uuid в Cargo.toml добавляем:
[dependencies]
uuid = { version = "1.20.0", features = ["serde", "v4"] }
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Stream {
    #[serde(rename = "user_id")]
    user_id:    Uuid,
    #[serde(rename = "is_private")]
    is_private: bool,
    settings:   u32,
    #[serde(rename = "shard_url")]
    shard_url:  String,
    #[serde(rename = "public_tariff")]
    public_tariff:  PublicTariff,
    #[serde(rename = "private_tariff")]
    private_tariff: PrivateTariff,
}

/// Подарок
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Gift {
    id:     u32,
    price:  u32,
    description: String,
}

/// Отладочные данные
/*
Для сериализации Uuid в Cargo.toml добавляем:
[dependencies]
chrono = { version = "0.4.43", features = ["serde"]}
 */
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Debug {
    duration:   String,
    at:         DateTime<Utc>,
}

/// Структура данных Request
/*
Для сериализации в Cargo.toml добавляем:
[dependencies]
serde = { version = "1.0.228", features = ["derive"] }
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Request {
    #[serde(rename = "type")]   // переименовать поле type_request в type
    type_request:   TypeRequest,
    stream:         Stream,
    gifts:          Vec<Gift>,
    debug:          Debug,
}

/// полукчить &str json базовый
fn get_json_str<'a>() ->&'a str {
r#"
{
  "type": "success",
  "stream": {
    "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
    "is_private": false,
    "settings": 45345,
    "shard_url": "https://n3.example.com/sapi",
    "public_tariff": {
      "id": 1,
      "price": 100,
      "duration": "1h",
      "description": "test public tariff"
    },
    "private_tariff": {
      "client_price": 250,
      "duration": "1m",
      "description": "test private tariff"
    }
  },
  "gifts": [{
    "id": 1,
    "price": 2,
    "description": "Gift 1"
  }, {
    "id": 2,
    "price": 3,
    "description": "Gift 2"
  }],
  "debug": {
    "duration": "234ms",
    "at": "2019-06-28T08:35:46+00:00"
  }
}    
"#    
}

#[cfg(test)]
mod tests {

  use super::* ;

  /// Тестирование serde_json
  #[test]
  fn check_serde_json() {
    // получить структуру Request из json строки
    let res = serde_json::from_str::<Request>(get_json_str()).unwrap() ;
    // получить json строку из структуры Request
    let res_str = serde_json::to_string(&res).unwrap() ;
    // получить структуру Request из json строки
    let res2 = serde_json::from_str::<Request>(&res_str).unwrap() ;
    // сравнить структуру полученную из json строки со структорой полученной из json строки
    assert_eq!(res, res2) ;
  }

  /// Тестирование serde_yaml
  #[test]
  fn check_serde_yaml() {
    // получить структуру Request из json строки для serde_yaml (это возможно)
    let req = serde_yaml::from_str::<Request>(get_json_str()).unwrap() ;
    // получить yaml строку из структуры Request
    let req_str = serde_yaml::to_string(&req).unwrap() ;
    // получить структуру Request из yaml строки
    let req2 = serde_yaml::from_str::<Request>(&req_str).unwrap() ;
    // сравнить структуру полученную из json строки со структорой полученной из yaml строки
    assert_eq!(req, req2) ;
  }

  /// Тестирование toml
  #[test]
  fn check_toml() {
    // получить структуру Request из json строки
    let req = serde_json::from_str::<Request>(get_json_str()).unwrap() ;
    // получить toml строку из структуры Request
    let req_str = toml::to_string(&req).unwrap() ;
    // получить структуру Request из toml строки
    let req2 = toml::from_str::<Request>(&req_str).unwrap() ;
    // сравнить структуру полученную из json строки со структорой полученной из toml строки
    assert_eq!(req, req2) ;
  }

}

fn main() {
    /*
    // Базовая данные в json формате:
    let json_str = r#"
{
  "type": "success",
  "stream": {
    "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
    "is_private": false,
    "settings": 45345,
    "shard_url": "https://n3.example.com/sapi",
    "public_tariff": {
      "id": 1,
      "price": 100,
      "duration": "1h",
      "description": "test public tariff"
    },
    "private_tariff": {
      "client_price": 250,
      "duration": "1m",
      "description": "test private tariff"
    }
  },
  "gifts": [{
    "id": 1,
    "price": 2,
    "description": "Gift 1"
  }, {
    "id": 2,
    "price": 3,
    "description": "Gift 2"
  }],
  "debug": {
    "duration": "234ms",
    "at": "2019-06-28T08:35:46+00:00"
  }
}    
    "# ;
     */
    // json формат
    let req = serde_json::from_str::<Request>(
                                                    get_json_str()
                                                ).unwrap() ;
    println!("req: {:?}", req) ;
    let json_str = serde_json::to_string(&req).unwrap() ;
    println!("json_str: {}", json_str) ;

    // yaml формат
    let yaml_str = serde_yaml::to_string(&req).unwrap() ;
    println!("\nyaml_str: {}", yaml_str) ;
    let yaml_req = serde_yaml::from_str::<Request>(yaml_str.as_str()) ;
    println!("yaml_req: {:?}", yaml_req) ;

    // toml формат
    let tompl_str = toml::to_string(&req).unwrap() ;
    println!("\ntompl_str: {}", tompl_str) ;
    let toml_req = toml::from_str::<Request>(tompl_str.as_str()).unwrap() ;
    println!("toml_req: {:?}", toml_req)

}

/*
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
 */
