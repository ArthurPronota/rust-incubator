use std::{any::Any, fmt};

use step_2_6::MyError;

#[derive(Debug)]
struct SuperErrorSideKick;

#[derive(Debug)]
struct SuperError {
    source: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl MyError for SuperError {

    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        Some(&self.source)
    }

}

impl MyError for SuperErrorSideKick {}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError { source: SuperErrorSideKick })
}

fn main() {
    match get_super_error() {
        Err(e) => {
            println!("Error: {e}, type_id: {:?}", e.type_id());
            println!("Caused by: {}, type_id: {:?}", e.source().unwrap(), e.source().unwrap().type_id());
            println!("SuperErrorSideKick: {:?}", e.source.source()) ;
        }
        _ => println!("No error"),
    }
}