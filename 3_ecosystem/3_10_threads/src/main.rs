use std::sync::{Arc, Mutex, mpsc} ;
use std::thread ;
use rand::Rng ;
use rayon::prelude::*;

const NUM_CONSUMERS: usize = 2 ;
const MATRIX_SIZE: usize = 4096 ;

fn main() {
    let (tr, rc) = mpsc::sync_channel::<Vec<Vec<u8>>>(NUM_CONSUMERS) ;
    let shared_rc = Arc::new(Mutex::new(rc)) ;

    // поток передачи матриц
    let handl_tr = thread::spawn(move || {
        
        let mut rnd = rand::thread_rng() ;

        loop {
            println!("loop") ;
            if let Err(err) = 
                        tr.send(
                            (0..MATRIX_SIZE)
                                .map(|_|
                                    (0..MATRIX_SIZE)
                                        .map(|_|
                                            rnd.r#gen::<u8>()
                                        )
                                        .collect::<Vec<u8>>()
                                )
                                .collect::<Vec<_>>()                        
                            //m
                        ) {
                eprintln!("Error sending matrix: {}", err) ;
                break ;
            }
        }
    }) ;

    let mut hand_receivers = vec![] ;
    for i in 0..NUM_CONSUMERS {
        let rc_tmp = Arc::clone(&shared_rc) ;
        hand_receivers.push(
            thread::spawn(move || {
                loop {
                    let m = rc_tmp.lock().unwrap().recv().unwrap() ;
                    let sum_items = m
                                        .par_iter()
                                        .map(|row| 
                                            row
                                                .iter()
                                                .map(|&v| v as u64)
                                                .sum::<u64>()
                                        )
                                        .sum::<u64>() ;

                    println!("Thread: {}, sum: {}", i, sum_items) ;
                }
            })
        ) ;
    }

    handl_tr
        .join()
        .unwrap() ;

    for h in hand_receivers {
        h.join().unwrap() ;
    }

}
