/*
            1. Зарегистрировать нового пользователя
$ cargo run --bin client -- user-register Arthur Pass10
JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg0NTA5OSwiaWF0IjoxNzc0ODQxNDk5fQ.6x6eY79uL8D6sYvud7Rry4vrubM_1tpJM96Dl056DIE
UserId: 2, UserName: Arthur

$ cargo run --bin client -- user-register Tom Pass20
JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTc3NDg0NTQ1OCwiaWF0IjoxNzc0ODQxODU4fQ.IemC6q7S82YK7R1n1jNCcbhZ7bIFAZa_ugDj7R8XTcw
UserId: 3, UserName: Tom

            2. Выполнить login
$ cargo run --bin client -- login Arthur Pass10
JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg0NTIwNCwiaWF0IjoxNzc0ODQxNjA0fQ.WU7VQdxy4l0WyIf7kQSnkotI91uItcn3xOviwdxlPdU
Registered user: UserId: 2, UserName: Arthur

            3. Добавить друга
$ cargo run --bin client -- add-friend 3 eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg0OTk4MiwiaWF0IjoxNzc0ODQ2MzgyfQ.Rx_NBrq5oqcnvzZ6i_QI9tdVpMfhKOD1p07GZKicVs0
Added friend: FriendId: 3, FriendName: Tom

            4. Удалить друга
$ cargo run --bin client -- del-friend 3 eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc3NDg1ODc2MiwiaWF0IjoxNzc0ODU1MTYyfQ.jw624nSVrSA3HjbdrqC4S8uWwfJAY1DU8uAKC838LK4
Deleted friend: FriendId: 3, FriendName: Tom

*/

//use std::path;

use anyhow::Result ;
use clap::Parser;

#[path = "../common.rs"]
mod common ;

#[path = "../args.rs"]
mod args ;

#[path = "../client_executor.rs"]
mod client_executor ;

#[path = "../graphql_client.rs"]
mod graphql_client ;

fn main() ->Result<()>{
    
    // получить все необходтиые для работы параметры, остальные игнорируем
    let (http_port, http_host, ..) = common::get_all_env_vars()? ;

    let args = args::Args::parse() ;

    //println!("args: {:?}", args) ;

    // Выполнить действие из аргументов командной строки
    client_executor::any_command(&args, &http_host, http_port)? ;

    Ok(())
}
