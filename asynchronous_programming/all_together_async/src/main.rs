// Putting It All Together: Futures, Tasks, and Threads
// https://doc.rust-lang.org/stable/book/ch17-06-futures-tasks-threads.html
//
use trpl ;
use std::thread ;
use std::time::Duration ;

fn main() {
    let (tr, mut rc) 
            = trpl::channel() ;

    thread::spawn(move || {
        for i in 1..=10 {
            tr
                .send(i)
                .unwrap() 
                ;
            thread::sleep(Duration::from_secs(1));
        }
    }) ;

    trpl::block_on(
        async {
            /*
            let v 
                = rc
                    .recv()
                    .await
                    ;
             */
            while let Some(val) = rc.recv().await {
                println!("{}", val) ;
            }
        }
    ) ;
}
