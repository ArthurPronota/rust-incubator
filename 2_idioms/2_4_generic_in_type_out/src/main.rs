/*
        1. Представление в памяти Cow<'a, str>: 

// Cow::Borrowed("text") - примерно эквивалентно:
struct CowBorrowed {
    tag: 0,          // Дискриминант для enum
    data: *const u8, // Указатель на строку где-то еще
    len: usize,
}

// Cow::Owned(String) - примерно эквивалентно:
struct CowOwned {
    tag: 1,          // Дискриминант для enum  
    ptr: *mut u8,    // Указатель на выделенную память
    len: usize,
    capacity: usize,
}

        2. Как работает cow.parse()

2.1. Deref coercion (принуждение разыменования)
Тип Cow<'a, str> реализует трейт Deref:

impl<'a, B: ?Sized> Deref for Cow<'a, B>
where
    B: ToOwned,
{
    type Target = B;
    
    fn deref(&self) -> &B {
        match *self {
            Cow::Borrowed(borrowed) => borrowed,
            Cow::Owned(ref owned) => owned.borrow(),
        }
    }
}

Это означает, что:
- &Cow<'a, str> автоматически преобразуется в &str
- При вызове метода на Cow<'a, str>, Rust ищет методы для &str

2.2  Метод parse() для &str
Тип &str имеет метод parse() благодаря реализации трейта FromStr:

// В стандартной библиотеке
impl FromStr for IpAddr {
    type Err = AddrParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Парсинг строки в IpAddr
    }
}

// И есть blanket implementation для parse()
pub trait FromStr: Sized {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

// Blanket implementation для всех типов, реализующих FromStr
impl<T> str {
    pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
        FromStr::from_str(self)
    }
}

Компилятор делает следующее:
    a) Deref coercion: cow → &Cow<'a, str> → &str (через Deref)
    b) Вызов метода: На &str вызывается метод parse::<IpAddr>()
    c) Парсинг: Метод parse() вызывает IpAddr::from_str(&str)
    d) Результат: Возвращает Result<IpAddr, AddrParseError>

*/


use std::net::{IpAddr, SocketAddr};
use std::borrow::Cow ;

const LOCAL_HOST: &str = "localhost" ;
const LOCAL_IP: &str = "127.0.0.1" ;

#[derive(Debug)]
pub struct Error<'a> {
    code:       Cow<'a, str>,
    status:     u16,
    message:    Cow<'a, str>,
}

impl<'a> Default for Error<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            code:    Cow::Borrowed("UNKNOWN"),
            status:  500,
            message: Cow::Borrowed("Unknown error has happened."),
        }
    }
}

impl<'a> Error<'a> {
    pub fn new<S>(code: S) -> Self
    /* тип S должен реализовывать преобразование в Cow::<'a, str>
       Это моут быть:
           1) String
           2) &String
           3) &str
           4) Cow::Owned
           5) Cow::Borrow
     */
        where S: Into<Cow::<'a, str>>,
    {
        let mut err = Self::default();
        err.code = code
                    .into()
                    ;
        err
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message<S>(&mut self, m: S) -> &mut Self 
        where S: Into<Cow<'a, str>>
    {
        self.message = m
                        .into()
                        ;
        self
    }
}

// данные по серверу
#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>) ;

// Компактный enum с автоматической конвертацией
#[derive(Debug)]
pub enum IpParam<'a> {
    Direct(IpAddr),
    Parsed(Cow<'a, str>),
}

impl<'a> IpParam<'a> {
    // Внутренняя конвертация в IpAddr
    fn into_ip_addr(&self) -> std::io::Result<IpAddr> {
        match self {
            IpParam::Direct(ip) => Ok(*ip),
            IpParam::Parsed(cow) => 
                cow
                    // так:
                    .parse() // Описан в начале файла: 2. Как работает cow.parse()
                    // или так:
                    // .parse::<IpAddr>()
                    .map_err(|e| {
                        std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!("Failed to parse IP '{}': {}", cow, e),
                        )
                    }),
        }
    }
}

// реализация From<IpAddr> into IpParam<'a>
impl<'a> From<IpAddr> for IpParam<'a> {
    fn from(ip: IpAddr) -> Self {
        IpParam::Direct(ip)
    }
}

// реализация From<&'a str> into IpParam<'a>
impl<'a> From<&'a str> for IpParam<'a> {
    fn from(s: &'a str) -> Self {
        if s == LOCAL_HOST {
            return IpParam::Parsed(Cow::Borrowed(LOCAL_IP)) ;
        }
        IpParam::Parsed(Cow::Borrowed(s))
    }
}

// реализация From<String> into IpParam<'a>
impl<'a> From<String> for IpParam<'a> {
    fn from(s: String) -> Self {
        if s.as_str() == LOCAL_HOST {
            return IpParam::Parsed(Cow::Borrowed(LOCAL_IP)) ;
        }
        IpParam::Parsed(Cow::Owned(s))
    }
}

// реализация From<&'a String> into IpParam<'a>
impl<'a> From<&'a String> for IpParam<'a> {
    fn from(s: &'a String) -> Self {
        if s == LOCAL_HOST {
            return IpParam::Parsed(Cow::Borrowed(LOCAL_IP)) ;
        }

        IpParam::Parsed(Cow::Borrowed(s))
    }
}

// реализация From<Cow<'a, str>> into IpParam<'a>
impl<'a> From<Cow<'a, str>> for IpParam<'a> {
    fn from(cow: Cow<'a, str>) -> Self {
        if &cow == LOCAL_HOST {
            return IpParam::Parsed(Cow::Borrowed(LOCAL_IP)) ;
        }
        IpParam::Parsed(cow)
    }
}

impl Server {
    // Основной метод
    pub fn bind<'a, T>(
                &mut    self,
                ip:     T, // impl Into<IpParam<'a>>, // ip должен реализовывать преобразование в IpParam<'a>
                port:   u16
           ) -> std::io::Result<()> 
        where T:    /*impl*/ Into<IpParam<'a>>,
    {
        let ip_param = ip.into();
        let ip_addr = ip_param.into_ip_addr()?;
        
        self.0 = Some(SocketAddr::new(ip_addr, port));
        Ok(())
    }
    
    pub fn get_address(&self) -> Option<SocketAddr> {
        self.0
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            // тестирование IpAddrV4(Ipv4Addr)
            let _ = server.bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            // тестирование &str с IPV6
            let _ = server.bind("::1", 9911);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");

            // тестирование &str с "localhost"
            let _ = server.bind("localhost", 9911);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:9911");

            // тестирование String
            let ip_string = "127.0.0.1".to_owned() ;
            let _ = server.bind(ip_string, 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            // тестирование &String
            let ip_string = "127.0.0.1".to_owned() ;
            let _ = server.bind(&ip_string, 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            // тестирование Cow::Borrowed
            let _ = server.bind(Cow::Borrowed("127.0.0.1"), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");  

            // тестирование Cow::Owned
            let ip_string = "127.0.0.1".to_owned() ;
            let _ = server.bind(Cow::Owned(ip_string), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

        }
    }
}

fn main() {

    // String как аргумент
    let mut err = Error::new("NO_USER".to_string());
    err.status(404).message("User not found".to_string());
    println!("1) err: {:?}", err) ;

    // &String как аргумент
    let no_user = "NO_USER".to_owned() ;
    let user_not_found = "User not found".to_string() ;
    let mut err = Error::new(&no_user);
    err.status(404).message(&user_not_found);
    println!("2) err: {:?}", err) ;

    // &str как аргумент
    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
    println!("3) err: {:?}", err) ;

    // Cow::Borrowed как аргумент
    let mut err = Error::new(Cow::Borrowed("NO_USER"));
    err.status(404).message(Cow::Borrowed("User not found"));
    println!("4) err: {:?}", err) ;

    // Cow::Owned как аргумент
    let mut err = Error::new(Cow::Owned("NO_USER".into()));
    err.status(404).message(Cow::Owned("User not found".into()));
    println!("5) err: {:?}", err) ;

}