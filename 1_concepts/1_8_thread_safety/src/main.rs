use std::sync::mpsc;  // Multi-producer, single-consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone() ;
    thread::spawn(move || {
        tx1.send("Hello from thread!").unwrap();
    });
    
    thread::spawn(move || {
        tx.send("Hello from thread2!").unwrap();
    });

    for mess in rx {
        println!("Received: {}", mess) ;
    }
    //println!("Received: {}", rx.recv().unwrap());
}
