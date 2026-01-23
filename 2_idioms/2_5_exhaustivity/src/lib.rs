/*
                Внесённые изменения:
    1. Добавлен #[non_exhaustive] для pub enum Event
    2. Добавлен пропущенный рукав: Event::NameUpdated(ev) => self.apply(ev), 
        для pub enum Event
*/

/// события полученные из источника
pub trait EventSourced<Ev: ?Sized> {
    // метод применить
    fn apply(&mut self, event: &Ev);
}

/// модуль пользователя
pub mod user {
    use std::time::SystemTime;

    use super::{
        event,          // модуль event
        EventSourced    // трейт EventSourced
    };

    /// структура пользователя
    #[derive(Debug)]
    pub struct User {
        /// идентификатор пользователя
        pub id:               Id,     
        /// Имя пользователя
        pub name:             Option<Name>,
        /// пользователь в Online с
        pub online_since:     Option<SystemTime>,
        /// DateTime создания пользователя
        pub created_at:       CreationDateTime,
        /// DateTime последней активности пользователя
        pub last_activity_at: LastActivityDateTime,
        /// DateTime удаления пользователя
        pub deleted_at:       Option<DeletionDateTime>,
    }

    /// реализация события создания пользователя
    impl EventSourced<event::UserCreated> for User {
        /// модификация структуры User на основе данных события
        fn apply(&mut self, ev: &event::UserCreated) {
            self.id = ev.user_id;
            self.created_at = ev.at;
            self.last_activity_at = LastActivityDateTime(ev.at.0);
        }
    }

    /// реализация события модификация имени пользователя
    impl EventSourced<event::UserNameUpdated> for User {
        /// модификация структуры User на основе данных события
        fn apply(&mut self, ev: &event::UserNameUpdated) {
            self.name = ev.name.clone();
        }
    }

    /// реализация события пользователь стал Online
    impl EventSourced<event::UserBecameOnline> for User {
        /// модификация структуры User на основе данных события
        fn apply(&mut self, ev: &event::UserBecameOnline) {
            self.online_since = Some(ev.at);
        }
    }

    // реализация события пользователь стал Offline
    impl EventSourced<event::UserBecameOffline> for User {
        /// модификация структуры User на основе данных события
        fn apply(&mut self, ev: &event::UserBecameOffline) {
            self.online_since = None;
            self.last_activity_at = LastActivityDateTime(ev.at);
        }
    }

    /// реализация события пользователь стал Deleted
    impl EventSourced<event::UserDeleted> for User {
        /// модификация структуры User на основе данных события
        fn apply(&mut self, ev: &event::UserDeleted) {
            self.deleted_at = Some(ev.at);
            self.last_activity_at = LastActivityDateTime(ev.at.0);
        }
    }

    /// Перечень событий для пользователя
    #[non_exhaustive]   // добавлен non_exhaustive
    #[derive(Debug)]
    pub enum Event {
        /// созданный
        Created(event::UserCreated),
        /// имя модифицировано
        NameUpdated(event::UserNameUpdated),
        /// стал Online
        Online(event::UserBecameOnline),
        /// стал Offline
        Offline(event::UserBecameOffline),
        /// стал удалённым
        Deleted(event::UserDeleted),
    }

    /// реализация событий Event для пользователя
    impl EventSourced<Event> for User {
        fn apply(&mut self, ev: &Event) {
            // добавлен match
            match ev {
                Event::Created(ev) => self.apply(ev),
                Event::Online(ev) => self.apply(ev),
                Event::Offline(ev) => self.apply(ev),
                Event::Deleted(ev) => self.apply(ev),
                // добалена пропущенная обработка события
                Event::NameUpdated(ev) => self.apply(ev),
            }
        }
    }

    /// идентификатор пользователя
    #[derive(Clone, Copy, Debug)]
    pub struct Id(pub u64);

    /// Имя пользователя
    #[derive(Clone, Debug)]
    pub struct Name(pub Box<str>);

    /// DateTime создания пользователя
    #[derive(Clone, Copy, Debug)]
    pub struct CreationDateTime(pub SystemTime);

    /// DateTime последней активности пользователя
    #[derive(Clone, Copy, Debug)]
    pub struct LastActivityDateTime(pub SystemTime);

    /// DateTime удаления пользователя
    #[derive(Clone, Copy, Debug)]
    pub struct DeletionDateTime(pub SystemTime);
}

// модуль событий (структуры с данными событий пользователя)
pub mod event {
    use std::time::SystemTime;

    use super::user;

    /// пользователь созданный
    #[derive(Debug)]
    pub struct UserCreated {
        pub user_id: user::Id,
        pub at: user::CreationDateTime,
    }

    /// пользовательское имя модифицированное
    #[derive(Debug)]
    pub struct UserNameUpdated {
        pub user_id: user::Id,
        pub name: Option<user::Name>,
        pub at: SystemTime,
    }

    /// польсователь ставший Online
    #[derive(Debug)]
    pub struct UserBecameOnline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    /// пользователь ставший Offline
    #[derive(Debug)]
    pub struct UserBecameOffline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    /// пользователь ставший удалённым
    #[derive(Debug)]
    pub struct UserDeleted {
        pub user_id: user::Id,
        pub at: user::DeletionDateTime,
    }
}
