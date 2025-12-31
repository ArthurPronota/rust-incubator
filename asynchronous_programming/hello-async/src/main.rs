// Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
// https://doc.rust-lang.org/stable/book/ch17-00-async-await.html
//
// Futures and the Async Syntax
// https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html
//
// cargo run -- https://utro.ru  https://www.rust-lang.org
// https://doc.rust-lang.org/stable/book/ch17-02-concurrency-with-async.html#applying-concurrency-with-async
//
use trpl::{
        Either, // Комбинирует 2-а различных futures, streams, or sinks имеющие один и тот-же ассоциированный тип в один тип.
        Html    // Тонкая оболочка вокруг scraper::Html 
    } ;
use std::future::Future ;

async fn page_title(url: &str) ->Option<String> {
    // так: VVV
    let responce = trpl::get(url)   //Извлекает данные с URL, паникует вместо возврата Result
                                .await // Приостанавливает выполнение до тех пор пока результат Furure будет готов
                                // Будет приостановлено исполнение текущей функции до тех пор пока исполнитель получит будущее завершённым.
                                ;
    let responce_text = responce
                                    .text() // Получает полный текст ответа, если ответ не может быть десириализован паникует вместо получения Result.
                                    .await // Приостанавливает выполнение ло тех пор пока результат Furure будет готов
                                    // Будет приостановлено исполнение текущей функции до тех пор пока исполнитель получит будущее завершённым.
                                    ;
    // или  так: VVV
    /*
    let responce_text = 
                        trpl::get(url)
                        .await
                        .text()
                        .await
                        ;
     */

    Html::parse(&responce_text)    // Разбирает Html документ из строки
            .select_first("title")  // Получает первый в документном совпадении строкового seector, если это не валидный CSS selector то паника.
            .map(|title| title.inner_html())
}

// асинхронная функция page_title трансормируется компилятором в 
// эту не асинхронную функцию VVV
fn page_title_real(url: &str) ->impl Future<Output = Option<String>> {
    async move {
        let responce_text = trpl::get(url)
                .await
                .text()
                .await
                ;
        Html::parse(&responce_text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

async fn page_title_2(url: &str) ->(&str, Option<String>) {

    let text_page 
            = trpl::get(url)
                .await
                .text()
                .await
                ;

    let title = Html::parse(&text_page)
                            .select_first("title")
                            .map(|title| title.inner_html())
                            ;
    (url, title)
}

fn main() {
    fn example() -> i32 {
        let x = {   // x имеет тип never
            return 5;   // реальный возврат из функции
            // ^^^^
            // любой код следующий этому выражению недостижим
        };
        10  // недостижимое выражение
    }

    println!("example(): {}", example()) ;

    // ------------------------
    println!() ;

    let args: Vec<_> = std::env::args()
                        .collect()
                        ;

    //let url = &args[0] ;
    let url = &args[1] ;
    println!("url: {}", url) ;

    let res_single_title = trpl::block_on( // Запускает одиночный future до завершения на специально разработанном Tokio Runtime.
                                page_title(url)
                            ) ;

    match res_single_title {
        Some(title) => println!("0) The title of url: {} was: {}", url, title),
        None => print!("0) Url: {} had no title", url),
    }

    // илди так: VVV
    // ---------------------
    println!("\n\n") ;

    // установка среды выполнения 
    // VVV
    trpl::block_on(  // Старое название: run(), Запускает одиночный future до завершению на специально разработаном Tokio Runtime.
        async {
            match page_title(url).await {
                Some(title) => println!("The title of url: {} was: {}", url, title),
                None  => print!("url: {} had no title", url),
            }
        }
    ) ;

    // ----------------------
    println!() ;

    let args: Vec<_> = std::env::args()
                            .collect()
                            ;
    trpl::block_on(
        async {
            let f1 = page_title_2(&args[1]) ;
            let f2 = page_title_2(&args[2]) ;

            let (url, maybe_title) = match trpl::select(  // Старое название race(), Запускает 2-а futures, принимая любой завершающийся первым и отменая другой.
                                                                        f1,
                                                                        f2
                                                                    )
                                                                    .await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

            println!("url: {} returned first.", url) ;
            match maybe_title {
                Some(title) => println!("Its page title was: {}", title),
                None => println!("It had no title."),
            }
        }
    ) ;

}
