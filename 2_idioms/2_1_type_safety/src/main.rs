use std::{fmt::Debug, marker::PhantomData};

use crate::state::{Deleted, New, Published, Unmoderated};

// параметры типов для Post
mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}

// параметры типов для User
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

// маркерные типы состояний (zero-cost)
mod state {
    pub struct New  ;

    pub struct Unmoderated ;

    pub struct Published ;

    pub struct Deleted ;
}

// структура Post с параметром состояния
#[derive(Clone)]
struct Post<S> {
    id:      post::Id,
    user_id: user::Id,
    title:   post::Title,
    body:    post::Body,
    _state:  std::marker::PhantomData<S>,
}

// трейт досиупа к атрибутам Post
trait PostAccess {
    fn id(&self) ->&post::Id ;
    fn user_id(&self) ->&user::Id ;
    fn title(&self) ->&post::Title ;
    fn body(&self) ->&post::Body ;
}

// реализация PostAccess для Post
impl<S> PostAccess for Post<S> {

    fn id(&self) ->&post::Id {
        &self.id
    }

    fn body(&self) ->&post::Body {
        &self.body
    }

    fn user_id(&self) ->&user::Id {
        &self.user_id
    }

    fn title(&self) ->&post::Title {
        &self.title
    }
}

// раализация Debug для Post
impl<S> Debug for Post<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Post")
            .field("id", self.id())
            .field("user_id", self.user_id())
            .field("title", self.title())
            .field("body", self.body())
            .field("_state", &self._state)
            .finish()
    }
}

// реализация Post<New>
impl Post<New> {
    // создание нового Post
    pub fn new(
                id:      post::Id,
                user_id: user::Id,
                title:   post::Title,
                body:    post::Body,
            ) ->Self {
        Self { 
            id, 
            user_id, 
            title, 
            body, 
            _state: PhantomData 
        }
    }

    // передача в модерацию нового поста с поглощением старого объекта для формирования нового
    pub fn publish(
                self    // self а не &self !!!
            ) ->Post<Unmoderated> {
        Post {
            id:         self.id, 
            user_id:    self.user_id,
            title:      self.title,
            body:       self.body,
            _state:     PhantomData,
        }
    }
}

// модерация Post<Unmoderated>
impl Post<Unmoderated> {

    // опубликовать Post
    pub fn allow(
               self // self а не &self !!!
            ) ->Post<Published> {
        Post { 
            id:         self.id,
            user_id:    self.user_id,
            title:      self.title,
            body:       self.body,
            _state:     PhantomData::<Published>,   // можно так
        }
    }

    // отклонение Post
    pub fn deny(
               self // self а не &self !!!
            ) ->Post<Deleted> {
        Post { 
            id:         self.id,
            user_id:    self.user_id,
            title:      self.title,
            body:       self.body,
            _state:     PhantomData
        }
    }

}

// реализация для опубликованного Post<Published>
impl Post<Published> {
    // удаление Post
    pub fn delete(
                self    // self а не &self !!!
            ) ->Post<Deleted> {
        Post { 
            id:         self.id,
            user_id:    self.user_id,
            title:      self.title,
            body:       self.body,
            _state:     PhantomData
        }
    }
}

fn main() {
    // создание нового поста
    let new_post = Post::new(
            post::Id(1),
            user::Id(1),
            post::Title("The title".to_owned()),
            post::Body("The body".to_owned()),
        ) ;
    println!("New Post: {:?}", new_post) ;

    // передача в модерацию, публикация, удаление поста
    let post_deleted = new_post
        .publish()
        .allow()
        .delete()
        ;
    println!("After delete Post: {:?}", post_deleted) ;

    // создание нового поста
    let new_post = Post::new(
            post::Id(1),
            user::Id(1),
            post::Title("The title".to_owned()),
            post::Body("The body".to_owned()),
        ) ;  
    println!("New Post: {:?}", new_post) ;

    // передача в модерацию, отклонение поста
    let post_deny = new_post
            .publish()
            .deny()
            ;
    println!("Post Denied: {:?}", post_deny) ;

    // создание нового поста
    let new_post = Post::new(
            post::Id(1),
            user::Id(1),
            post::Title("The title".to_owned()),
            post::Body("The body".to_owned()),
        ) ;
    /* no method named `deny` found for struct `Post<New>`
    new_post.deny() ;   // 
     */
    println!("Unmoderated Post: {:?}",
                new_post
                    .publish()
        ) ;
}