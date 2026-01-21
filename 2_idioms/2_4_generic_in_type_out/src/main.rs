/*
        Представление в памяти Cow<'a, str>: 

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
*/


use std::net::{IpAddr, SocketAddr};
use std::borrow::Cow ;

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

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind(&mut self, ip: IpAddr, port: u16) {
        self.0 = Some(SocketAddr::new(ip, port))
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

            server.bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind("::1".parse().unwrap(), 9911);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}