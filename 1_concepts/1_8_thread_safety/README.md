Шаг 1.8: Потоковая безопасность
=======================

__Estimated time__: 1 day

[Rust] has [`Send`] and [`Sync`] marker traits which are fundamental for concurrency and thread safety story in [Rust] and represent one of [fearless concurrency][2] corner stones (which allow to [avoid data races][1] at compile time).

To better understand [`Send`]/[`Sync`]'s purpose, design, limitations and use cases, read through:
- [Official `Send` docs][`Send`]
- [Official `Sync` docs][`Sync`]
- [Rust Book: 16.4. Extensible Concurrency with the Sync and Send Traits][3]
- [Rustonomicon: 8.2. Send and Sync][4]
- [Huon Wilson: Some notes on Send and Sync][5]
- [Piotr Sarnacki: Arc and Mutex in Rust][9]
- [nyanpasu64: An unsafe tour of Rust's Send and Sync][6]
- [Josh Haberman: Thread Safety in C++ and Rust][7]
- [Cliff L. Biffle: Safely writing code that isn't thread-safe][8]
- [Louis Dureuil: Too dangerous for C++][10]
- [Cuong Le: This Send/Sync Secret Separates Professional From Amateur Rust Developers][11]




## Task

Implement the following types, which meet conditions:
1. `OnlySync` is `Sync`, but `!Send`.
2. `OnlySend` is `Send`, but `!Sync`.
3. `SyncAndSend` is both `Sync` and `Send`.
4. `NotSyncNotSend` is both `!Sync` and `!Send`.

All inner details of implementation are on your choice.

Play with these types from multiple threads to see how compile time [fearless concurrency][2] works in practice.


Реализуйте следующие типы, удовлетворяющие условиям:
1. `OnlySync` is `Sync`, but `!Send`.
2. `OnlySend` is `Send`, but `!Sync`.
3. `SyncAndSend` is both `Sync` and `Send`.
4. `NotSyncNotSend` is both `!Sync` and `!Send`.

Все внутренние детали реализации — на ваш выбор.

Поэкспериментируйте с этими типами в нескольких потоках, чтобы увидеть, как на практике работает [Потоковая безопасность][2] на этапе компиляции.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- [`Что означает "безбоязненная конкурентность" в Rust? С помощью каких механизмов Rust точно выполняет эту гарантию?`](#что-означает-безбоязненная-конкурентность-в-rust-с-помощью-каких-механизмов-rust-точно-выполняет-эту-гарантию)


- [`Зачем вообще существуют Send и Sync? Как это связано с внутренней изменчивостью?`](#зачем-вообще-существуют-send-и-sync-как-это-связано-с-внутренней-изменчивостью)

<hr>

<h3>Что означает "безбоязненная конкурентность" в Rust? С помощью каких механизмов Rust точно выполняет эту гарантию?</h3>

«Безбоязненная конкурентность» (Fearless Concurrency) — это концепция Rust, которая означает, что компилятор гарантирует отсутствие типичных ошибок многопоточности (таких как состояния гонки — `data races`) на этапе компиляции. Программист может писать многопоточный код, будучи уверенным, что он не приведет к непредсказуемому поведению памяти.
Rust достигает этого не через «сборщик мусора» или сложные проверки в рантайме, а через систему типов и владения.

<h4>Основные механизмы гарантии конкурентности:</h4>

1. Владение и Перемещение (Ownership & Move)

В Rust объект может иметь только одного владельца. Когда вы передаете переменную в другой поток через `std::thread::spawn`, она перемещается (move).

- <b>Результат</b>: Старый поток больше не может обратиться к этим данным. Это исключает ситуацию, когда два потока одновременно пытаются изменить одну и ту же переменную.

2. Заимствование и Мутабельность (Borrowing)

Правила заимствования Rust гласят: либо много неизменяемых ссылок (`&T`), либо одна изменяемая (`&mut T`).

- <b>Результат</b>: Вы не можете создать изменяемую ссылку в одном потоке, если в другом потоке уже есть любая ссылка на эти же данные. Компилятор просто не соберет такой код.

3. Маркерные трейты: `Send` и `Sync`

Это фундаментальные инструменты, которые «помечают» типы в зависимости от их безопасности в многопоточной среде:

- <b>Send</b>: Позволяет передавать владение типом между потоками. Почти все типы в Rust являются Send (например, i32, String, Vec). Однако Rc (указатель с подсчетом ссылок для одного потока) — не Send, потому что его счетчик не атомарен. Если вы попытаетесь передать Rc в другой поток, компилятор выдаст ошибку.
```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    
    // Vec<i32> реализует Send, поэтому это компилируется
    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);
    });
    
    handle.join().unwrap();
}
```
- <b>Sync</b>: Позволяет безопасно обращаться к типу из нескольких потоков одновременно через ссылки (`&T`). Тип является Sync, если &T является Send. Например, типы с «внутренней мутабельностью» без блокировок (как RefCell) — не Sync.
```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);  // Arc реализует Sync
    
    let handles: Vec<_> = (0..3).map(|i| {
        let data_ref = Arc::clone(&data);
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_ref);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

4. Безопасные абстракции стандартной библиотеки

Rust предоставляет примитивы, которые инкапсулируют небезопасное поведение и делают его безопасным:

- <b>Arc (Atomic Reference Counted)</b>: Атомарный умный указатель, который позволяет нескольким потокам владеть данными. В отличие от Rc, он использует атомарные операции для счетчика, поэтому он Send и Sync. Документация Arc.
- <b>Mutex и RwLock</b>: В Rust Mutex «владеет» данными. Чтобы получить доступ к данным, вы обязаны вызвать .lock(). Это возвращает MutexGuard, который гарантирует эксклюзивный доступ и автоматически освобождает замок, когда выходит из области видимости.

Использование Arc + Mutex
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
Использование Arc + RwLock
```rust
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
```

5. Каналы (Channels)

Rust активно поддерживает философию: «Не общайтесь через разделяемую память, разделяйте память через общение». Трейты Send гарантируют, что как только вы отправили данные в канал (mpsc), вы теряете к ним доступ, и их безопасно получает другой поток.
```rust
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
```
Итог:

Rust остается лидером в системном программировании именно благодаря тому, что ошибки многопоточности — это ошибки компиляции, а не часы отладки в рантайме. Вы можете смело использовать сложные паттерны (параллельные итераторы rayon, асинхронность tokio), зная, что если код скомпилировался, в нем нет состояний гонки по памяти. Официальная глава о конкурентности.

<hr>

<h3>Зачем вообще существуют Send и Sync? Как это связано с внутренней изменчивостью?</h3>

Send и Sync остаются фундаментальными инструментами, которые превращают «ручную» проверку безопасности потоков в автоматическую проверку компилятором. Они нужны для того, чтобы Rust мог гарантировать fearless concurrency (безбоязненную конкурентность).

<h4>1. Зачем они существуют?</h4>

Это маркерные трейты, которые сообщают компилятору о свойствах типа при передаче между потоками:

- `Send`: Отвечает на вопрос «Могу ли я передать владение этим объектом в другой поток?». Если тип реализует Send, его можно безопасно переместить (move) в другой поток.
- `Sync`: Отвечает на вопрос «Могу ли я безопасно делиться ссылками на этот объект между потоками?». Тип является Sync, только если ссылка на него &T является Send.

Без этих трейтов Rust не смог бы запретить, например, передачу ``Rc<T>` в другой поток. Поскольку Rc использует обычный (неатомарный) счетчик ссылок, одновременное изменение этого счетчика из двух потоков привело бы к повреждению памяти (data race). Компилятор видит, что Rc не реализует Send, и запрещает такой код.

<h4>2. Как это связано с внутренней изменчивостью?</h4>

Внутренняя изменчивость (Interior Mutability) — это способность изменять данные через неизменяемую ссылку (`&T`). Именно здесь Send и Sync играют критическую роль, так как бесконтрольное изменение данных через &T из разных потоков — это прямой путь к катастрофе.

Связь можно проследить на трех примерах:

А. `Cell<T>` и `RefCell<T>` (Не Sync)

Эти типы предоставляют внутреннюю изменчивость, но они не используют потокобезопасные механизмы (атомики или блокировки).

- Их реализация Sync отсутствует.
- Почему? Если бы они были Sync, два потока могли бы одновременно вызвать .borrow_mut() или .set(), что привело бы к непредсказуемому поведению. Поэтому их можно использовать только внутри одного потока.

Б. `Mutex<T>` и `RwLock<T>` (Sync)

Эти типы также обеспечивают внутреннюю изменчивость, но делают это через механизмы синхронизации ОС.

- Они реализуют `Sync`, если оборачиваемый тип `T` реализует `Send`.
- Связь: `Mutex` гарантирует, что даже если у десяти потоков есть ссылка `&Mutex<T>`, в каждый момент времени только один из них сможет получить `&mut T` к внутренним данным. Это делает внутреннюю изменчивость безопасной для многопоточности.

В. Атомарные типы (AtomicUsize, AtomicBool)

Они обеспечивают внутреннюю изменчивость на аппаратном уровне.

- Они являются Sync.
- Связь: Поскольку процессор гарантирует атомарность операции изменения, нам не нужны тяжелые блокировки. Ссылками на одно и то же AtomicUsize можно безопасно делиться между всеми потоками.
```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // Создаем атомарное число, обернутое в Arc для совместного владения между потоками
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Создаем 10 потоков
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // Используем fetch_add для безопасного инкремента.
                // Ordering::SeqCst гарантирует строгую последовательность операций.
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Дожидаемся завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Загружаем финальное значение
    println!("Итоговое значение счетчика: {}", counter.load(Ordering::SeqCst));
}
```
<h4>Резюме: Логическая цепочка</h4>

1. Внутренняя изменчивость позволяет обходить правило Rust «нельзя изменять через &T».
2. Но если мы разрешим это в многопоточности без защиты, возникнет состояние гонки.
3. Sync — это фильтр. Компилятор разрешает передавать ссылки &T между потоками только для тех типов, чья внутренняя изменчивость либо защищена (как в Mutex), либо отсутствует, либо реализована атомарно.


</hr>


[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/nomicon/races.html
[2]: https://doc.rust-lang.org/book/ch16-00-concurrency.html
[3]: https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html
[4]: https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
[5]: http://huonw.github.io/blog/2015/02/some-notes-on-send-and-sync
[6]: https://nyanpasu64.github.io/blog/an-unsafe-tour-of-rust-s-send-and-sync
[7]: https://blog.reverberate.org/2021/12/18/thread-safety-cpp-rust.html
[8]: https://cliffle.com/blog/not-thread-safe
[9]: https://web.archive.org/web/20220929143451/https://itsallaboutthebit.com/arc-mutex
[10]: https://blog.dureuill.net/articles/too-dangerous-cpp
[11]: https://blog.cuongle.dev/p/this-sendsync-secret-separates-professional-and-amateur
