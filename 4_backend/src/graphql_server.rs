use std::sync::Arc;     // Импорт Arc (Atomic Reference Counting) для потокобезопасного разделяемого владения

use anyhow::Result ;    // Импорт типа Result из крейта anyhow для упрощенной обработки ошибок

use async_graphql::dataloader::{    // Импорт модуля dataloader из крейта async_graphql, содержащий инструменты для пакетной загрузки данных и решения проблемы N+1 запросов
        DataLoader,     // Импорт DataLoader
        Loader,         // Импорт DataLoadeLoader для пакетной загрузки данных
    };

use async_graphql::{    // Импорт основных компонентов из крейта async-graphql
            Context,            // Импорт Context - контекст GraphQL запроса (данные, расширения)
            InputObject,        // Импорт InputObject - трейт для структур-аргументов GraphQL
            Object,             // Импорт Object - трейт для определения полей GraphQL объектов
            Schema,             // Импорт Schema - тип для объединения Query, Mutation и Subscription
            SimpleObject,       // Импорт SimpleObject - трейт для автоматической реализации GraphQL объекта
            EmptySubscription,  // Импорт EmptySubscription - тип для пустой подписки (без real-time)
            ComplexObject,      // Импорт ComplexObject - трейт для сложных объектов с кастомными резолверами
};

use async_graphql_axum::{       // Импорт интеграции async-graphql с фреймворком Axum
            GraphQLRequest,     // Импорт GraphQLRequest - обертка для HTTP запроса с GraphQL
            GraphQLResponse     // Импорт GraphQLResponse - обертка для HTTP ответа с результатом GraphQL
};

use axum::Extension ;       // Импорт Extension - тип для извлечения данных из расширений запроса в Axum
use validator::Validate;    // Импорт трейта Validate для валидации структур

use sqlx ;  // Импорт крейта sqlx для асинхронной работы с базами данных

use crate::common;     // Импорт модуля common из текущего крейта
use crate::db ;        // Импорт модуля db (пул соединений, миграции, функции работы с БД)

use crate::jwt ;       // Импорт модуля jwt (генерация и валидация JWT токенов)
use crate::passw;      // Импорт модуля passw (хэширование и проверка паролей)

use crate::users::{    // Импорт модуля users с его содержимым
        self,          // Импорт самого модуля users
        Users,         // Импорт структуры Users из модуля users
    } ;

use crate::friends::Friends ;   // Импорт структуры Friends из модуля friends (связи пользователь-друг)

// Пользователь
#[derive(
    SimpleObject,   // SimpleObject позволяет async-graphql автоматически превратить её в тип GraphQL.
    Clone,          // Автоматически реализует трейт Clone для возможности клонирования экземпляров структуры
  )
]
#[graphql(complex)] // Указывает async-graphql, что структура имеет сложные поля с отдельной реализацией резолверов
pub struct User {
    id:   u32,      // код пользователя
    name: String,   // наименование пользователя
}

#[ComplexObject]    // Атрибут, указывающий async-graphql, что следующий блок impl содержит дополнительные резолверы для полей структуры User
impl User {
    // Этот метод будет вызываться КАЖДЫЙ РАЗ, когда в запросе встречается поле "friends"
    // когда friends относится к User
    // Асинхронный метод для загрузки списка друзей пользователя
    // Это резолвер
    async fn friends(&self, ctx: &Context<'_>) -> Result<Vec<User>> {

        // Попытка извлечь DataLoader для друзей из контекста GraphQL
        let loader = match ctx.data::<DataLoader<FriendDataLoader>>() {
            Ok(ld) => ld,   // При успехе - сохраняем загрузчик в переменную
            Err(err) => return Err(anyhow::anyhow!("{:?}", err)),   // При ошибке - возвращаем ошибку с описанием
        } ;

        let friends = 
                // Асинхронная загрузка друзей по ID пользователя через DataLoader
                loader.load_one(self.id).await?;

        // Возвращаем список друзей или пустой вектор, если друзей нет
        Ok(friends.unwrap_or_default())
    }    
}

/// DataLoader для загрузки друзей пользователей
#[derive(
    Clone   // Автоматически реализует трейт Clone для возможности клонирования экземпляров структуры
  )
]
pub struct FriendDataLoader {
    pub pool:   Arc<db::Database>,  // Пул соединений с базой данных обёрнутый в Arc
}

// Реализуем трейт Loader для типа FriendDataLoader
// Ключ для загрузки - u32 (ID пользователя)
// Загружаем список друзей для каждого пользователя
impl Loader<u32> for FriendDataLoader {
    type Value = Vec<User>;         // Что загружаем - вектор друзей
    type Error = Arc<sqlx::Error>;  // Тип ошибки с оберткой Arc

    // асинхронный метод загрузки
    async fn load(
            &self,
            keys: &[u32],   // массив ID пользователей, для которых нужно загрузить друзей
        ) -> std::result::Result<
                    std::collections::HashMap<  // HashMap
                                            u32,    // колюч пользователя
                                            Self::Value // массив со списком друзей
                                        >,
                                        Self::Error
                                > {

        let rows = 
            // формируем запрос
            sqlx::query_as::<_, 
                (
                  u32,      // user_id
                  u32,      // friend_id
                  String    // Name friend
                )
                >(
                format!(
                r#"
                SELECT friends.user_id, friends.friend_id, users.name 
                FROM friends JOIN users ON friends.friend_id = users.id_user
                WHERE friends.user_id in ({})
                "#,
                keys
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
                )
                .as_str()
            )
            // Выполните запрос и верните все полученные строки, собранные в объект Vec.
            .fetch_all(
                &self.pool.pool
            )
            .await?;  

        // создать пустой HashMap
        let mut results = std::collections::HashMap::new();

        // перебор всех полученных строк
        for row in rows {
            // установить код текущего пользователя
            let user_id = row.0 ; // user_id
            // установить данные по текущему другу
            let friend = User { 
                id: row.1,        // friend_id
                name: row.2,      // Name friend
            } ;

            results
                .entry(user_id) // Получаем Entry (запись) для ключа user_id: существует или нет
                .and_modify(|users: &mut Vec<User>| {   // Обеспечивает изменяемый доступ к занятой точке на месте до любых потенциальных добавлений на карту.
                    // проверка что текущего friend нет в друзьях
                    if ! users
                          .iter()
                          .any(|u| u.id == friend.id)
                    {
                       users.push(friend.clone());  // добавляем вового друга к существующим друзьям
                    }
                  }
                )
                .or_insert_with(|| vec![friend])  // Если ключа user_id нет - создаем новый вектор с другом
                ;
        }

        Ok(results)              
    }
}

// Краткая информация о пользователе
#[derive(
    SimpleObject    // SimpleObject позволяет async-graphql автоматически превратить её в тип GraphQL.
  )
]
pub struct UserShortInfo {
    pub id:     u32,        // код пользователя
    pub name:   String,     // наименование пользователя
}

// Возвразаемая информация о процессе Login
#[derive(
    SimpleObject    // SimpleObject позволяет async-graphql автоматически превратить её в тип GraphQL.
  )
]
pub struct LoginResult {
    token:  String,             // JSON Web Token
    user:   UserShortInfo,      // краткая информация о пользователе
}

/// Краткая информация о друге
#[derive(
    SimpleObject       // SimpleObject позволяет async-graphql автоматически превратить её в тип GraphQL.
  )
]
pub struct FriendShortInfo {
    pub id:     u32,        // код друга
    pub name:   String,     // наименование друга
}

/// Возврашаемая информация о добавленном друге
#[derive(
    SimpleObject,       // SimpleObject позволяет async-graphql автоматически превратить её в тип GraphQL.
  )
]
pub struct AddFriendResult {
    friend: FriendShortInfo,    // краткая информация о друге
}

/// Возврашаемая информация об удалённом друге
#[derive(
    SimpleObject,
  )
]
pub struct DelFriendResult {
    friend: FriendShortInfo,    // краткая информация о удалённом друге
}


// Определяем корневой запрос (Query).
pub struct Query;

#[Object]
#[ComplexObject]
impl Query {
    // Наш резолвер. Он принимает jws и возвращает структуру User.
    async fn userplus(&self, ctx: &Context<'_>, jwt: String) ->Result<User> {
        // получить auth_serv для работы с JSON Web Token
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // Проверить jwt и получить данные по нему - это ваша сессия
        let jwt_data = 
                auth_serv.validate_token(&jwt)? ;

        // Получить данные по DB
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // Поиск пользователя из JWT
        let user_found = 
                users::Users::find_no_trans_raise(
                    &db_res.pool, 
                    jwt_data.get_sub()
                )
                .await? ;

        Ok(
            User {  // формируем структуру User
                id:     user_found.id_user(),   // код пользователя
                name:   user_found.name().to_string(),  // наименование пользователя
            }
        )
    }
}


// Входные данные для логирования
#[derive(
    InputObject     // Реализует трейт InputObject для использования структуры в качестве входного аргумента GraphQL
  )
]
struct LoginInputObject {
    name:       String,     // наименование пользователя
    password:   String,     // пароль пользователя
}

// Входные данные для добавления друга (тип inp аргумента)
#[derive(
    InputObject     // Реализует трейт InputObject для использования структуры в качестве входного аргумента GraphQL
  )
]
struct AddFriendObject {
    friendid:   u32,        // код друга
    jwt:        String,     // JSON Web Token
}

// Входные данные для удаления друга (тип inp аргумента)
#[derive(
    InputObject     // Реализует трейт InputObject для использования структуры в качестве входного аргумента GraphQL
  )
]
struct DelFriendObject {
    friendid:   u32,        // код друга
    jwt:        String,     // JSON Web Token
}

// Корневой Mutation тип
pub struct Mutation ;

#[Object]   // Атрибут, указывающий async-graphql, что следующий impl содержит GraphQL поля/методы для типа Mutation
impl Mutation {
    // Выполнение login
    async fn login( // Асинхронная функция (мутация) для входа пользователя в систему
        &self, 
        ctx: &Context<'_>,      // Контекст GraphQL, содержащий данные запроса (пул БД, DataLoader, расширения)
        inp: LoginInputObject,  // Входные данные: имя пользователя и пароль, переданные в аргументе inp
      ) 
      ->Result<LoginResult>     // Возвращает Result с типом LoginResult (успешный вход) или ошибку
    {
        // получить пул соединений с DB
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // получить auth_serv для работы с JSON Web Token
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // установка и проверка параметров пользователя
        let mut tmp_user = Users::default() ;

        tmp_user.set_name(&inp.name)? ;

        tmp_user.set_password(&inp.password)? ;

        // Сформировать новую транзакцию
        let mut trans = db_res
                                            .pool
                                            .begin()
                                            .await
                                            ?;
        // контроль наличия вставляемого пользователя
        let found_user = match Users::find_for_name(
                &mut *trans,
                tmp_user.name(),
                false
            )
            .await? {
           Some(u) => u,
           None => return Err(anyhow::anyhow!(common::INVALID_USERNAME_PASSWORD)),
        } ;

        // проверка соответствия пользователя и его хеша
        if ! passw::check_password(tmp_user.password(), found_user.password())? {
            return Err(anyhow::anyhow!(common::INVALID_USERNAME_PASSWORD)) ;
        }

        /* 
            Возврат данных полного формата, клиент может запросить часть
            (всё что ниже login:)
Формат: 
{
    "data": {   <- добавлен автоматически
        "login": {  <- добавлен автоматически, название метода
            "token":"aaasdasdsfsdgdrghdfgdgh",
            "user": {
                "id": 10,
                "name": "123"
            }
        }
    }
}

или вариант с ошибкой:
{   
    "data":null,
    "errors":[
        {
            "message":"Invalid password !!!!!!!!!!!",
            "locations":[{"line":3,"column":17}],
            "path":["login"]
        }
    ]
}    
        */

        // Формирование выходных данных
        Ok(
            LoginResult {
                token:  auth_serv.generate_token(found_user.id_user())?,
                user:   UserShortInfo { id: found_user.id_user(), name: found_user.name().to_owned() }
            }
        )
    }


    // регистрация нового пользователя
    async fn register(
        &self, 
        ctx: &Context<'_>,          // Контекст GraphQL, содержащий данные запроса (пул БД, DataLoader, расширения)
        inp: LoginInputObject,      // Входные данные: имя пользователя и пароль, переданные в аргументе inp
      )
      ->Result<LoginResult> // Возвращает Result с типом LoginResult (успешная регистрация) или ошибку
    {
        // получить пул соединений с DB
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // получить auth_serv для работы с JSON Web Token
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // установка параметров пользователя 
        let mut tmp_user = Users::default() ;

        tmp_user.set_name(&inp.name)? ;

        tmp_user.set_password(&inp.password)? ;

        // Сформировать новую транзакцию
        let mut trans = db_res
                                            .pool
                                            .begin()
                                            .await
                                            ?;
        // контроль наличия вставляемого пользователя
        if Users::find_for_name(
                &mut *trans,
                tmp_user.name(),
                true
            )
            .await?
            .is_some() 
            {
                return Err(anyhow::anyhow!("A user named: {} already exists.", tmp_user.name()));
            }

        // вставить нового пользователя
        tmp_user = Users::int_user(
            &mut *trans,
            tmp_user.name(),
            &passw::hash_password(tmp_user.password())?,    // сгенерировать hash of password
        )
        .await? ;

        // выполнить commit
        trans
            .commit()
            .await? ;

        /* 
            Возврат данных полного формата, клиент может запросить часть
            (всё что ниже login:)
Формат: 
{
    "data": {   <- добавлен автоматически
        "register": {  <- добавлен автоматически, название метода
            "token":"aaasdasdsfsdgdrghdfgdgh",
            "user": {
                "id": 10,
                "name": "123"
            }
        }
    }
}

или вариант с ошибкой:
{   
    "data":null,
    "errors":[
        {
            "message":"Invalid password !!!!!!!!!!!",
            "locations":[{"line":3,"column":17}],
            "path":["login"]
        }
    ]
}    
        */

        // формирование выходных данных
        Ok(
            LoginResult {
                token:  auth_serv.generate_token(tmp_user.id_user())?,
                user:   UserShortInfo { id: tmp_user.id_user(), name: tmp_user.name().to_owned() }
            }
        )
    }    

    // добавление друга
    async fn addfriend(
        &self, 
        ctx: &Context<'_>,          // Контекст GraphQL, содержащий данные запроса (пул БД, DataLoader, расширения)
        inp: AddFriendObject,       // Входные данные: код друга и JWT, переданные в аргументе inp
      ) 
      ->Result<AddFriendResult>     // Возвращает Result с типом AddFriendResult (успешное добавление друга) или ошибку
    {
        // получить пул соединений с DB
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // получить auth_serv для работы с JSON Web Token
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // Проверить jwt и получить данные по нему - это ваша сессия
        let jwt_data = 
                auth_serv.validate_token(&inp.jwt)? ;

        // установка друга с проверкой его параметров
        let mut tmp_friend = Friends::default() ;

        // установить код текущего пользователя
        tmp_friend.set_user_id(
                    jwt_data.get_sub()  // код текущего пользователя
                )? ;

        // установить код друга
        tmp_friend.set_friend_id(inp.friendid)? ;

        // проверить полученные данные
        tmp_friend.validate()? ;

        // Создать новую транзацию
        let mut trans = db_res
                                        .pool
                                        .begin()
                                        .await? ;
        // вставить друга
        tmp_friend = Friends::insert(
            &mut *trans,
            tmp_friend.user_id(),
            tmp_friend.friend_id(),
        )
        .await? ;

        // получить данные по другу
        let tmp_user = Users::find_for_id_user_raise(
                        &mut *trans, 
                        tmp_friend.friend_id(), 
                        false,
                    )
                    .await? ;

        // выполнить commit
        trans
            .commit()
            .await? ;

        // Сформировать выходные данные
        Ok(
            AddFriendResult {
                friend: FriendShortInfo { id: tmp_user.id_user(), name: tmp_user.name().to_owned() }
            }
        )
    }  

    // удаление друга
    async fn delfriend(
        &self, 
        ctx: &Context<'_>,      // Контекст GraphQL, содержащий данные запроса (пул БД, DataLoader, расширения)
        inp: DelFriendObject,   // Входные данные: код друга и JWT, переданные в аргументе inp
      ) 
      ->Result<DelFriendResult> // Возвращает Result с типом DelFriendResult (успешное удаление друга) или ошибку
    {
        // получить пул соединений с DB
        let db_res = 
                ctx.data::<Arc<db::Database>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // получить auth_serv для работы с JSON Web Token
        let auth_serv = 
                ctx.data::<Arc<jwt::AuthService>>()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))? ;

        // Проверить jwt и получить данные по нему - это ваша сессия
        let jwt_data = 
                auth_serv.validate_token(&inp.jwt)? ;

        // сформировать и проверить данные по другу
        let mut tmp_friend = Friends::default() ;

        // установить код текущего пользователя
        tmp_friend.set_user_id(
                    jwt_data.get_sub()  // код текущего пользователя
                )? ;

        // установить код друга
        tmp_friend.set_friend_id(inp.friendid)? ;

        // проверить полученные данные
        tmp_friend.validate()? ;

        // Создать новую транзацию
        let mut trans = db_res
                                        .pool
                                        .begin()
                                        .await? ;

        Friends::delete(
            &mut *trans,
            tmp_friend.user_id(),
            tmp_friend.friend_id(),
        )
        .await? ;
        
        // получить данные по другу
        let tmp_user = Users::find_for_id_user_raise(
                        &mut *trans, 
                        tmp_friend.friend_id(), 
                        false,
                    )
                    .await? ;

        // выполнить commit
        trans
            .commit()
            .await? ;

        // сформировать выходные данные
        Ok(
            DelFriendResult { 
                friend: FriendShortInfo { 
                            id: tmp_user.id_user(),
                            name: tmp_user.name().to_owned()
                        } 
            }
        )
    }
}

// Создание типа данных GraphQL схема
type MySchema = Schema<     // Определение псевдонима типа для GraphQL схемы
                    Query,      // Тип Query - корневой тип для операций чтения (запросов)
                    Mutation,   // Mutation - корневой тип для операций изменения (мутаций)
                    EmptySubscription,  // EmptySubscription - корневой тип для подписок (здесь пустой, без real-time уведомлений)
                >;

// Обработчик запросов от клиента
pub async fn graph_handler(
                schema:     Extension<MySchema>,    // Извлекаем схему GraphQL из расширений Axum (внедрение зависимости)
                req:        GraphQLRequest          // Получаем GraphQL запрос (содержит query, variables, operation_name)
             ) 
             ->GraphQLResponse // Функция возвращает GraphQL ответ (JSON с data или errors)
{
    schema
        .execute(   // Вызываем метод execute на схеме для выполнения GraphQL запроса
        req.into_inner()    // Преобразуем GraphQLRequest из обертки Axum во внутренний тип async_graphql::Request
        )
        .await  // Ожидаем асинхронного выполнения запроса к схеме
        .into() // Преобразуем async_graphql::Response в GraphQLResponse (обертку Axum для HTTP ответа)
}