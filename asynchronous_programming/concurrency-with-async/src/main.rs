// Applying Concurrency with Async
// https://doc.rust-lang.org/stable/book/ch17-02-concurrency-with-async.html
//
// Working with Any Number of Futures
// https://doc.rust-lang.org/stable/book/ch17-03-more-futures.html
//
use trpl ;
use std::time::Duration ;

fn main() {
    trpl::block_on(  // Запускает одиночный future до завершения на специально созданном Tokio Runtime.
        async {
            let handl = trpl::spawn_task(   // Порождает новую асинхронную задачу, возвращая JoinHandle для него.
                /*  Предоставленный future начнёт работать в фоновом 
                    режиме немедленно когда spawn является вызванным, 
                    даже если вы не ожидаете возвращённого JoinHandle.
                    VVV
                */
                async {
                    for i in 1..20 {
                        println!("This number: {} from the first task.", i) ;
                        // Асинхронный аналог для std::thread::sleep.
                        trpl::sleep(Duration::from_millis(500))
                            .await
                            ;
                    }
                }
            ) ;

            for i in 1..10 {
                println!("\tThis number: {} from the second task.", i) ;
                // Асинхронный аналог для std::thread::sleep.
                trpl::sleep(Duration::from_millis(500))
                    .await
                    ;
            }

            handl
                .await
                .unwrap()
                ;
        }
    ) ;

    // ----------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let fut1 = async {
                for i in 1..10 {
                    println!("This number: {} from the first task.", i) ;
                    trpl::sleep(Duration::from_millis(500))
                        .await
                        ;
                }
            } ;

            let fut2 = async {
                for i in 1..10 {
                    println!("\tThis number: {} from the second task.", i) ;
                    trpl::sleep(Duration::from_millis(500))
                        .await
                        ;
                }
            } ;

            // Объединяет результат 2-х of futures, ожидая для них обоих чтобы бать завершёными.
            // Эта функция вернёт новый фьючер, который ожидает завершения обоих 
            // фьючеров. Возвращённый фьючер будет содержать кортеж из обоих 
            // результатов.
            trpl::join(fut1, fut2)
                    .await // Приостонавливает исполнение до тех пор пока результат [Future] является готовым.
                   ;
        }
    ) ;

    // ----------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) 
                    = trpl::channel() ;

            let val = String::from("hi") ;
            tr.send(val)
                .unwrap()
                ;

            let received = rc
                    .recv()
                    .await
                    .unwrap()
                    ;

            /*
            let v: Option<String> = None ;
            let v2 = v.unwrap() ;   // panic
             */

            println!("received: {}", received) ;
        }
    ) ;

    // ----------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) 
                    = trpl::channel() ;

            let vals = vec![
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ] ;

            for val in &vals {
                println!("send: {}", val) ;

                tr
                    .send((*val).clone())
                    .unwrap()
                    ;
                
                trpl::sleep(Duration::from_millis(500))
                        .await ;
                
            }

            let mut count = 0 ;

            while let Some(v) = rc
                                            .recv()
                                            .await {
                println!("received: {}", v) ;
                if {count += 1 ; count} >= vals.len(){
                    break;
                }
            }
        }
    ) ;

    // --------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) 
                    = trpl::channel() ;

            let vals = vec![
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
            ] ;

            let tr_fut = async {
                for val in &vals {
                    println!("2) send: {}", val) ;
                    tr
                        .send((*val).to_owned())
                        .unwrap()
                        ;
                    trpl::sleep(Duration::from_millis(500))
                            .await
                            ;
                }
            } ;

            let rc_fut = async {
                let mut num = 0 ;
                while let Some(val) = rc
                                        .recv()
                                        .await {
                    println!("\t2) received: {}", val) ;
                    if {num += 1 ; num} >= vals.len() {
                        break;
                    }
                }
            } ;

            trpl::join(tr_fut, rc_fut)
                .await
                ;
        }
    ) ;

    // ------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) = trpl::channel() ;

            let tr_fut = async move {
                let vals = vec![
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                ] ;

                for val in vals {
                    println!("3) send: {}", val) ;
                    tr
                        .send(val)
                        .unwrap()
                        ;
                    trpl::sleep(Duration::from_millis(500))
                        .await ;
                }
            } ;

            let rc_fut = async {
                while let Some(v) = rc
                                               .recv()
                                               .await {
                    println!("\t3) received: {}", v) ;
                }
            } ;

            trpl::join(tr_fut, rc_fut)
                .await ;
        }
    ) ;

    // ---------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let (tr, mut rc) = trpl::channel() ;
            let tr2 = tr.clone() ;

            let tr_fut = async move {
                let vals = vec![
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                ] ;

                for val in vals {
                    println!("4) send: {}", val) ;
                    tr.send(val)
                        .unwrap() 
                        ;
                    trpl::sleep(Duration::from_millis(500))
                        .await
                        ;
                }
            } ;

            let tr2_fut = async move {
                let vals = vec![
                        String::from("5"),
                        String::from("6"),
                        String::from("7"),
                        String::from("8"),
                ] ;

                for val in vals {
                    println!("\t4) send: {}", val) ;
                    tr2.send(val)
                        .unwrap()
                        ;
                    trpl::sleep(Duration::from_millis(500))
                        .await
                        ;
                }
            } ;

            let rc_fut = async {
                while let Some(v) = rc.recv().await {
                    println!("\t\treceived: {}", v) ;
                }
            } ;

            // так: VVV
            trpl::join3(tr_fut, tr2_fut, rc_fut)
                .await ;
            // или так:
            //trpl::join!(tr_fut, tr2_fut, rc_fut) ;
        }
    ) ;
}
