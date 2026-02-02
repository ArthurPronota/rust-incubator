use chrono::{
        Datelike, 
        NaiveDate, 
    } ;

fn main() {
    //unimplemented!() // Указывает на нереализованный код, вызывая панику с сообщением «not implemented».
    let user = User::with_birthdate(2010, 02, 1) ;

    println!("age: {}, is_adult: {}", user.age(), user.is_adult()) ;
}

const NOW: &str = "2019-06-26";

struct User(NaiveDate) ;

impl User {
    /// создание пользователя на основании дня рождения
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        //unimplemented!()
        
        match NaiveDate::from_ymd_opt(year, month, day)  {
            Some(nd) => Self(nd),
            None => panic!("Invalid birthdate: {}-{}-{}", year, month, day),
        }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        //unimplemented!()

        match NaiveDate::parse_from_str(NOW, "%Y-%m-%d") {
            Ok(now_d) => {
                let mut years = now_d.year() - self.0.year() ;
                if years < 0 {
                    years = 0 ;
                }

                if (now_d.month(), now_d.day()) < (self.0.month(), self.0.day()) {
                    if years > 0 {
                        years -= 1 ;
                    }
                }

                return years as u16 ;
            },
            Err(err) => panic!("Error: {} convert NaiveDate to NaiveDateTime for: {}", err, NOW),
        }
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            ((2016, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    /// проврка на adult
    #[test]
    fn is_adult() {
        let user = User::with_birthdate(2000, 2, 1) ;

        assert!(user.is_adult()) ;
    }

    /// проврка на не adult
    #[test]
    fn isnt_adult() {
        let user = User::with_birthdate(2010, 02, 1) ;
        assert!(!user.is_adult()) ;
    }

    /// проверка возраста
    #[test]
    fn check_age() {
        let user = User::with_birthdate(2010, 02, 1) ;

        assert_eq!(user.age(), 9) ;
    }
}
