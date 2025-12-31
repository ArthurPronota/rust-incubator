// Working with Any Number of Futures
// https://doc.rust-lang.org/stable/book/ch17-03-more-futures.html
// Listing 17-23
//

use trpl::{self, Either} ;
use std::{pin::{Pin, pin}, thread, time::{Duration, Instant}} ;

fn main() {
    trpl::block_on(
        async {
            let f = async {
                5
            } ;

            let f2 = async {
                6
            } ;

            let v = trpl::join!(f, f2) ;

            println!("v: {:?}", v) ;
        }
    ) ;

    // ----------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) = trpl::channel() ;

            let tr_fut = async move {
                let vars = vec![
                            String::from("1"),
                            String::from("2"),
                            String::from("3"),
                            String::from("4"),
                ] ;

                for val in vars {
                    println!("2) sent: {}", val) ;
                    tr.send(val)
                      .unwrap()
                      ;
                    trpl::sleep(Duration::from_millis(500))
                        .await
                        ;
                }
            } ;
            
            let rc_fut = async {
                while let Some(v) = rc
                                                .recv()
                                                .await {
                    println!("\t2) received: {}", v) ;
                }
            } ;

            let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
                                                            Box::pin(tr_fut),   // Constructs a new Pin<Box<T>>
                                                            Box::pin(rc_fut),
                                                        ] ;

            trpl::join_all(futures)
                .await
                ;
        }
    ) ;

    // -------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) = trpl::channel() ;
            let tr2 = tr.clone() ;

            let tr_fut = pin!(
                async move {
                    let vals = vec![
                        String::from("1"),
                        String::from("2"),
                        String::from("3"),
                        String::from("4"),
                    ] ;

                    for val in vals {
                        println!("3) sent: {}", val) ;
                        tr.send(val).unwrap() ;
                        trpl::sleep(Duration::from_millis(500)).await ;
                    }
                }
            ) 
            ;

            let tr2_fut = pin!(
                async move {
                    for val in vec!["5", "6", "7", "8"] {
                        println!("\t3) sent: {}", val) ;
                        tr2.send((*val).to_owned()).unwrap() ;
                        trpl::sleep(Duration::from_millis(500)).await ;
                    }
                }
            ) 
            ;

            let rc_fut = pin!(
                async {
                    while let Some(v) = rc.recv().await {
                        println!("\t\t3) received: {}", v) ;
                    }
                }
            )
            ;

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> 
                    = vec![tr_fut, tr2_fut, rc_fut] ;

            trpl::join_all(futures)
                .await 
                ;
        }
    ) ;

    // ------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let a = async {
                1u32
            } ;

            let b = async {
                "Hello!"
            } ;

            let c = async {
                true
            } ;

            let (a_result, b_result, c_result) 
                    = trpl::join!(a, b, c) ;

            println!("a_result: {}, b_result: {}, c_result: {}", a_result, b_result, c_result) ;
        }
    ) ;

    // ----------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let slow = async {
                println!("slow started.") ;
                trpl::sleep(Duration::from_millis(100)).await ;
                println!("slow finished.") ;
                "slow"
            } ;

            let fast = async {
                println!("fast started.") ;
                trpl::sleep(Duration::from_millis(50)).await ;
                println!("fast finished.") ;
                "fast"
            } ;

            match trpl::select(slow, fast).await {
                Either::Left(left) => println!("It's: {}", left),
                Either::Right(right) => println!("It's: {}", right),
            }  ;

        }
    ) ;

    // ----------------------
    println!("\n\n") ;

    fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("name: {} run for {}ms", name,ms) ;
    }

    trpl::block_on( 
        async {
            let a = async {
                println!("a started.") ;
                slow("a", 30);
                slow("a",  10);
                slow("a", 20);
                trpl::sleep(Duration::from_millis(50))
                    .await ;
                println!("a fihished.") ;
            } ;

            let b = async {
                println!("b started.") ;
                slow("b", 75);
                slow("b", 10);
                slow("b", 15);
                slow("b", 350);
                trpl::sleep(Duration::from_millis(50)).await ;
                println!("b finished.") ;
            } ;
            trpl::select(a, b)
                    .await ;
        }
    ) ;

    // ----------------------------
    println!("\n\n\n") ;

    let one_ms = Duration::from_millis(1) ;

    trpl::block_on(
        async {
            let a = async {
                println!("a started.") ;
                slow("a", 30);
                trpl::sleep(one_ms).await ;
                slow("a", 10);
                trpl::sleep(one_ms).await ;
                slow("a", 20);
                trpl::sleep(one_ms).await ;
                println!("a finished.") ;
            } ;

            let b = async {
                println!("b stared.") ;
                slow("b", 75);
                trpl::sleep(one_ms).await ;
                slow("b", 10);
                trpl::sleep(one_ms).await ;
                slow("b", 15);
                trpl::sleep(one_ms).await ;
                slow("b", 340);
                trpl::sleep(one_ms).await ;
                println!("b finished.")
            } ;

            trpl::select(a, b).await ;
        }
    ) ;

    // ---------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let a = async {
                println!("a started.") ;
                slow("a", 30);
                trpl::yield_now() // Уступает выполнение назад к Tokio runtime.
                    .await ;
                slow("a", 10);
                trpl::yield_now().await ;
                slow("a", 20);
                trpl::yield_now().await ;
                println!("a finished.") ;
            } ;

            let b = async {
                println!("b started.") ;
                slow("b", 75);
                trpl::yield_now().await ;
                slow("b", 10);
                trpl::yield_now().await ;
                slow("b", 15);
                trpl::yield_now().await ;
                slow("b", 350);
                trpl::yield_now().await ;
                println!("b finished.") ;
            } ;

            trpl::select(a, b).await ;
        }
    ) ;

    // -----------------------
    println!("\n\n") ;

    let one_ns = Duration::from_nanos(1) ;

    trpl::block_on(
        async {
            let start = Instant::now() ;
            async {
                for i in 1..=10000 {
                    trpl::sleep(one_ns).await ;
                }
            }
            .await ;
            let time = Instant::now() - start ;
            println!("sleep vesion finished after: {} seconds.", time.as_secs_f32()) ;

            let start = Instant::now() ;
            async {
                for i in 1..=10000 {
                    trpl::yield_now().await ;
                }
            }
            .await ;

            let time = Instant::now() - start ;
            println!("yeld version finished after: {} seconds", time.as_secs_f32()) ;
        }
    ) ;

    // -----------------------------
    println!("\n\n") ;

    async fn timeout<F /*: Future */>(
                future_to_try: F,
                max_time:      Duration,
             ) ->Result<F::Output, Duration>
        where F:    Future
    {
        match trpl::select(
                    future_to_try,
                    trpl::sleep(max_time)
                ).await {
            Either::Left(output) => Ok(output),
            Either::Right(_) => Err(max_time),
        }
    }

    trpl::block_on(
        async {
            let slow = async {
                trpl::sleep(Duration::from_secs(5)).await ;
                "I finished!"
            } ;

            match timeout(
                    slow,
                    Duration::from_secs(20)
              ).await {
                Ok(message)    => println!("Successfull with message: {}", message),
                Err(duration) => println!("Failed after seconds: {}", duration.as_secs()),
            }

        }

    ) ;

}
