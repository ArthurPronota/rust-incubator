use std::{pin::pin, time::Duration};

use trpl::{self, ReceiverStream, Stream, StreamExt} ;

fn main() {

    trpl::block_on(
        async {
            let values: Vec<i32> = (1..=10)
                                        .collect(); // vec![] ;
            let iter = values
                                        .iter()
                                        .map(|&n| n * 2)
                                        ;
            let mut stream 
                        = trpl::stream_from_iter(iter) 
                        ;
            //let v = stream.next() ;
            
            while let Some(value) = stream
                                        .next()
                                        .await {
                println!("The value was: {}", value) ;
            }
        }
    ) ;

    // ----------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let values = 1..101 ;
            let iter = values
                        .map(|n| n * 2)
                         ;
            let stream 
                        = trpl::stream_from_iter(iter)
                            ;
            let mut filtered 
                        = stream
                            .filter(|&value| 
                                    value % 3 == 0
                                    || value % 5 == 0) ;
            /*
            let v 
                    = filtered
                        .next()
                        .await ;
             */

            while let Some(val) = filtered
                                    .next()
                                    .await {
                println!("The value was: {}", val) ;
            }

        }
    ) ;

    // --------------------------
    println!("\n\n") ;

    fn get_messages() ->impl Stream<Item = String> {
        let (tr, rc) 
                = trpl::channel() ;

        let messages: Vec<_> = ('a' ..= 'j')
                                    .map(|ch| ch.to_string())
                                    .collect() 
                                    ;
        for message in messages {
            tr
                .send(
                    format!("Message: {}", message)
                  )
                .unwrap()
                ;
        }

        ReceiverStream::new(rc)
    }

    trpl::block_on(
        async {
            let mut messages 
                        = get_messages() ;

            while let Some(message) = messages
                                        .next()
                                        .await {
                println!("{}", message) ;
            }
        }
    ) ;

    // ------------------------------
    println!("\n\n") ;

    trpl::block_on(
        async {
            let mut messages 
                        = pin!(
                            get_messages()
                            .timeout(Duration::from_millis(200))  // Принимает для каждого элемента timeout к переданному потоку.
                          ) 
                          ;
            /*
            let v 
                    = messages
                     .next()
                    .await ;
             */
            while let Some(result) = messages
                                    .next()
                                    .await {
                match result {
                    Ok(message) => println!("{}", message),
                    Err(reason) => eprintln!("Problem: {}", reason),
                }
            }

        }
    ) ;

    // ----------------------
    println!("\n\n") ;

    fn get_messages2() ->impl Stream<Item = String> {
        let (tr, rc) 
                = trpl::channel() ;

        // работает в фоновом режиме в области сформированной среды выполнения, без ожидания VVV
        trpl::spawn_task(   // Предоставленный future начнёт работать в background немедленно когда spawn является вызванный, даже если вы не ждёте возвращённого JoinHandle.
            async move {
                let messages: Vec<String> = ('a' ..= 'j')
                                                    .map(|x| x.to_string())
                                                    .collect()
                                                    ;
                for (index, message) in messages
                                                    .into_iter()
                                                    .enumerate() {
                    let time_sleep = if index % 2 == 0 {
                        100
                    }
                    else {
                        300
                    };
                    trpl::sleep(Duration::from_millis(time_sleep)).await ;
                    /*
                    tr
                        .send(format!("Message: {}", message))
                        .unwrap()
                        ;
                     */
                    if let Err(err) = tr.send(format!("Message: {}", message)) {
                        eprintln!("Cannot send message: {}, error: {}", message, err) ;
                        break ;
                    }
                }
            }
        ) ;
        // работает в фоновом режиме, без ожидания ^^^

        ReceiverStream::new(rc)
    }

    trpl::block_on(
        async move {
            let mut messages 
                    = pin!(
                            get_messages2() // !!! будет работать в области async move {...}
                                .timeout(Duration::from_millis(200))
                        )
                        ;
            /*
            let v = messages
                .next()
                .await
                ;
             */
            while let Some(result) = messages
                                        .next()
                                        .await {
                match result {
                    Ok(message) => println!("{}", message),
                    Err(reason) => println!("Problem: {}", reason),
                }
            }
        }
    ) ;

    // -----------------------------
    println!("\n\n") ;

    fn get_intervals() -> impl Stream<Item = u32> {
        let (tr, rc) = trpl::channel() ;

        trpl::spawn_task(
            async move {
                let mut count = 0 ;
                loop {
                    trpl::sleep(Duration::from_millis(1)).await ;
                    count += 1 ;
                    /*
                    tr.send(count).unwrap() ;
                     */
                    if let Err(err) = tr.send(count) {
                        eprintln!("Cannot send message: {}, error: {}", count, err) ;
                        break ;
                    }
                }
            }
        ) ;

        ReceiverStream::new(rc)
    }

    trpl::block_on(
        async {
            let messages 
                    = get_messages2()
                        .timeout(Duration::from_millis(200)) 
                    ;
            let intervals 
                    = get_intervals() 
                        .map(|x| format!("Interval: {}", x))
                        .throttle(Duration::from_millis(100))
                        .timeout(Duration::from_millis(500))
                    ;
            let merged 
                    = messages
                        .merge(intervals)
                        .take(50) ;
            let mut stream 
                        = pin!(merged) ;

            /*
            let v 
                    = stream
                        .next()
                        .await
                        ;
             */

            while let Some(result) = stream
                                        .next()
                                        .await {
                match result {
                    Ok(message) => println!("{}", message),
                    Err(reson) => eprintln!("Problem: {}", reson),
                }
            }
        }
    ) ;

}
