/*
    Contact: https://artaudiochats.t.me/

    Созданы две реализации:
    1. Релизация mpsc на основе Arc<Mutex>
    2. Релизация mpmc на основе crossbeam_channel

    Вторая реализация лучше стандартного mpsc.
        a) crossbeam-channel использует эффективный механизм "кражи" или 
            "перехвата" сообщений. Когда два потока вызывают .recv() на клонах одного ресивера, библиотека гарантирует, что сообщение достанется только одному из них — тому, кто освободился первым.
        b) Отсутствие Mutex: Вам не нужно оборачивать Receiver в 
            Arc<Mutex<...>>. Клон в crossbeam — это дешевая операция, 
            работающая на атомарных счетчиках, что гораздо быстрее блокировок.
        c) Автоматическое завершение: Цикл while let Ok(msg) = r.recv() — это идиома. 
            Он автоматически прервется, как только будут удалены (dropped) 
            все объекты Sender, связанные с этим каналом.

    Важный нюанс: drop(resv) в главном потоке
    В коде выше я есть вызвов drop(resv) в main. Если этого не сделать,
        в главном потоке останется один "живой" получатель. 
        Программа никогда не завершится, так как воркеры будут думать, 
        что кто-то еще может забрать данные.

*/

// Полезные примитивы синхронизации.
use std::sync::{Arc, Mutex, mpsc} ;
// Нативные потоки.
use std::thread ;
// Это первичный trait, который следует использовать при генерации случайных значений.
use rand::Rng ;
// В прелюдии rayon импортируются различные характеристики ParallelIterator.
use rayon::prelude::*;
// Multi-producer multi-consumer каналы для передачи сообщений.
use crossbeam_channel::bounded ;

// количество потредителей матриц
const NUM_CONSUMERS: usize = 2 ;
// размер матрицы
const MATRIX_SIZE: usize = 4096 ;

fn main() {

    // -------------- 1. Релизация mpsc на основе Arc<Mutex> --------------

    // Создает новый синхронный ограниченный канал.
    let (tr, rc) = mpsc::sync_channel::<Vec<Vec<u8>>>(NUM_CONSUMERS) ;
    // создание обёртки Arc<Mutex(rc)> для приёмника
    let shared_rc = Arc::new(Mutex::new(rc)) ;

    // поток передачи матриц
    let handl_tr = thread::spawn(move || {
        // Получите генератор случайных чисел, инициализируемый системой с 
        // отложенной инициализацией и расположенный локально в потоке.
        let mut rnd = rand::thread_rng() ;

        loop {
            if let Err(err) = 
                        // Отправляет значение по этому синхронному каналу.
                        tr.send(
                            // создание матрицы размером MATRIX_SIZE x MATRIX_SIZE
                            (0..MATRIX_SIZE)
                                .map(|_|
                                    (0..MATRIX_SIZE)
                                        .map(|_|
                                            // генерация случайного значения для u8
                                            rnd.r#gen::<u8>()
                                        )
                                        .collect::<Vec<u8>>()
                                )
                                .collect::<Vec<_>>()                        
                        ) { // обработка ошибки
                eprintln!("Error sending matrix: {}", err) ;
                break ;
            }
        }
    }) ;

    let mut hand_receivers = vec![] ;
    for i in 0..NUM_CONSUMERS {
        // клонирование обёртки приёмника для создаваемого потока
        let rc_tmp = Arc::clone(&shared_rc) ;
        hand_receivers.push(
            // создаём поток приёмника
            thread::spawn(move || {
                loop {
                    // получаем данные из приёмника с блокировкой mutex
                    let m = rc_tmp.lock().unwrap().recv().unwrap() ;
                    // выполнение параллельно-последовательного варианта суммирования 
                    // элементов матрицы
                    let sum_items = m
                                        // праллельна итерация по строкам матрицы
                                        .par_iter() 
                                        .map(|row| 
                                            row
                                                // последовательная итерация строки матрицы
                                                .iter()
                                                .map(|&v| v as u64)
                                                .sum::<u64>()
                                        )
                                        .sum::<u64>() ;
                    // вывод суммы элементов матрицы
                    println!("Thread: {}, sum: {}", i, sum_items) ;
                }
            })
        ) ;
    }

    // -------------- 2. Релизация mpmc на основе crossbeam_channel --------------

    // Создает канал с ограниченной пропускной способностью.
    let (sndr, resv) = bounded::<Vec<Vec<u8>>>(NUM_CONSUMERS) ;

    // поток передачи матриц
    let handl_tr2 = thread::spawn(move || {
        
        // Получите генератор случайных чисел, инициализируемый системой с 
        // отложенной инициализацией и расположенный локально в потоке.
        let mut rnd = rand::thread_rng() ;

        loop {
            if let Err(err) = 
                // Блокирует текущий поток до тех пор, пока не будет отправлено 
                // сообщение или канал не будет разорван.
                sndr.send(
                    // создание матрицы размером MATRIX_SIZE x MATRIX_SIZE
                    (0..MATRIX_SIZE)
                        .map(|_| {
                            (0..MATRIX_SIZE)
                                .map(|_| {
                                    rnd.r#gen::<u8>()
                                })
                                .collect::<Vec<u8>>()
                        })
                        .collect::<Vec<_>>()
            ) { // обработка ошибки
                eprintln!("Error sending matrix2: {}", err) ;
                break ;
            }
        }
    }) ;

    let mut hand_trans2 = vec![] ;

    for i in 0..NUM_CONSUMERS {
        // клонируем приёмник
        let resv_clone = resv.clone() ;

        hand_trans2.push(
            // создаём поток приёмника
            thread::spawn(move || {
                // Блокирует текущий поток до получения сообщения или до тех пор, 
                // пока канал не опустеет и не будет разорван.
                while let Ok(m) = resv_clone.recv() {
                    let sum_items = 
                            // выполнение параллельно-последовательного варианта суммирования 
                            // элементов матрицы                    
                            m
                                // праллельна итерация по строкам матрицы
                                .par_iter()
                                .map(|row| {
                                    row
                                        // последовательная итерация строки матрицы
                                        .iter()
                                        .map(|&x| {
                                            x as u64
                                        })
                                        .sum::<u64>()
                                })
                                .sum::<u64>() ;
                    println!("\tThread: {}, sum: {}", i, sum_items) ;
                }
            })
        ) ;

    }

    // Удаление resv, чтобы он не держал канал открытым в главном потоке
    drop(resv);

    // -- Ожидает завершения соответствующих потоков с обработкой ошибок --

    handl_tr.join().unwrap() ;

    for h in hand_receivers {
        h.join().unwrap() ;
    }

    handl_tr2.join().unwrap() ;

    for h in hand_trans2 {
        h.join().unwrap() ;
    }

}