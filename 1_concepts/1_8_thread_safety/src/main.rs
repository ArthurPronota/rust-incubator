use std::sync::{Arc, RwLock} ;
use std::thread ;
use std::time::Duration ;

fn main() {

    let vd = Arc::new(RwLock::new(0)) ;

    let mut throw_arr = vec![] ;
    for _ in 0..2 {
        let vd_clone = vd.clone() ;
        throw_arr.push(
            thread::spawn(move || {
                let mut v = vd_clone.write().unwrap() ;
                *v += 1 ;
                println!("v: {}", v) ;
            })
        ) ;
    }


    let vd_clone = vd.clone() ;
    throw_arr.push(
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            println!("result: {}", vd_clone.read().unwrap()) ;
        })
    ) ;

    for un in throw_arr {
        un.join().unwrap() ;
    }

}