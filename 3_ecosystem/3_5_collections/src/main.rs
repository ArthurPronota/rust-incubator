fn main() {
    println!("Implement me!");
}

/*
use im::HashMap;
use im::HashSet;
use std::sync::Arc;

// Структура User
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: u64,
    pub nickname: String,
    pub email: String,
}

// Трейт UsersRepository
pub trait UsersRepository {
    /// Возвращает пользователя по его ID
    fn get_user_by_id(&self, id: u64) -> Option<&User>;
    
    /// Возвращает несколько пользователей по их IDs
    fn get_users_by_ids(&self, ids: &[u64]) -> Vec<&User>;
    
    /// Возвращает IDs пользователей, чей nickname содержит заданную строку
    fn find_user_ids_by_nickname(&self, search_term: &str) -> HashSet<u64>;
}

// Реализация на основе неизменяемых коллекций
#[derive(Debug, Clone)]
pub struct ImUsersRepository {
    // HashMap для быстрого поиска по ID (O(log n))
    users_by_id: HashMap<u64, Arc<User>>,
    
    // Индекс для поиска по nickname (значение -> множество IDs)
    // Можно было бы использовать HashMap<String, HashSet<u64>>, но
    // для поиска по подстроке лучше использовать линейный поиск
}

impl ImUsersRepository {
    /// Создает новый репозиторий из списка пользователей
    pub fn new(users: Vec<User>) -> Self {
        // Создаем HashMap для быстрого поиска по ID
        let users_by_id = users
            .into_iter()
            .map(|user| {
                let id = user.id;
                (id, Arc::new(user))
            })
            .collect();
        
        Self {
            users_by_id,
        }
    }
    
    /// Добавляет пользователя (возвращает новую версию репозитория)
    pub fn add_user(&self, user: User) -> Self {
        let new_users_by_id = self.users_by_id
            .update(user.id, Arc::new(user));
        
        Self {
            users_by_id: new_users_by_id,
        }
    }
    
    /// Обновляет пользователя (возвращает новую версию репозитория)
    pub fn update_user(&self, user: User) -> Self {
        self.add_user(user)  // Для im::HashMap update заменяет или добавляет
    }
    
    /// Удаляет пользователя по ID (возвращает новую версию)
    pub fn remove_user(&self, id: u64) -> Self {
        let new_users_by_id = self.users_by_id.without(&id);
        
        Self {
            users_by_id: new_users_by_id,
        }
    }
    
    /// Возвращает количество пользователей
    pub fn len(&self) -> usize {
        self.users_by_id.len()
    }
    
    /// Проверяет, пуст ли репозиторий
    pub fn is_empty(&self) -> bool {
        self.users_by_id.is_empty()
    }
}

impl UsersRepository for ImUsersRepository {
    fn get_user_by_id(&self, id: u64) -> Option<&User> {
        self.users_by_id
            .get(&id)
            .map(|arc_user| arc_user.as_ref())
    }
    
    fn get_users_by_ids(&self, ids: &[u64]) -> Vec<&User> {
        ids.iter()
            .filter_map(|id| self.get_user_by_id(*id))
            .collect()
    }
    
    fn find_user_ids_by_nickname(&self, search_term: &str) -> HashSet<u64> {
        // Поиск по подстроке требует линейного обхода
        // Можно было бы создать инвертированный индекс, но
        // для простоты используем линейный поиск
        self.users_by_id
            .iter()
            .filter(|(_, user)| {
                user.nickname
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            })
            .map(|(id, _)| *id)
            .collect()
    }
}

// Оптимизированная версия с индексами для поиска
#[derive(Debug, Clone)]
pub struct IndexedImUsersRepository {
    // Основная мапа для поиска по ID
    users_by_id: HashMap<u64, Arc<User>>,
    
    // Индекс для точного поиска по nickname
    // Полезно, если нужен точный поиск, а не по подстроке
    users_by_exact_nickname: HashMap<String, HashSet<u64>>,
}

impl IndexedImUsersRepository {
    pub fn new(users: Vec<User>) -> Self {
        let mut users_by_id = HashMap::new();
        let mut users_by_exact_nickname = HashMap::new();
        
        for user in users {
            let id = user.id;
            let nickname = user.nickname.clone();
            let user_arc = Arc::new(user);
            
            // Добавляем в основную мапу
            users_by_id = users_by_id.update(id, user_arc.clone());
            
            // Обновляем индекс по nickname
            let mut ids_for_nickname = users_by_exact_nickname
                .get(&nickname)
                .cloned()
                .unwrap_or_else(HashSet::new);
            
            ids_for_nickname = ids_for_nickname.update(id);
            users_by_exact_nickname = users_by_exact_nickname.update(nickname, ids_for_nickname);
        }
        
        Self {
            users_by_id,
            users_by_exact_nickname,
        }
    }
    
    // Поиск по точному совпадению nickname (быстро, O(log n))
    pub fn find_user_ids_by_exact_nickname(&self, nickname: &str) -> HashSet<u64> {
        self.users_by_exact_nickname
            .get(nickname)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
}

impl UsersRepository for IndexedImUsersRepository {
    fn get_user_by_id(&self, id: u64) -> Option<&User> {
        self.users_by_id
            .get(&id)
            .map(|arc_user| arc_user.as_ref())
    }
    
    fn get_users_by_ids(&self, ids: &[u64]) -> Vec<&User> {
        ids.iter()
            .filter_map(|id| self.get_user_by_id(*id))
            .collect()
    }
    
    fn find_user_ids_by_nickname(&self, search_term: &str) -> HashSet<u64> {
        // Для поиска по подстроке все равно нужен линейный обход,
        // но можно оптимизировать, если сделать индекс по словам
        self.users_by_id
            .iter()
            .filter(|(_, user)| {
                user.nickname
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            })
            .map(|(id, _)| *id)
            .collect()
    }
}

// Пример использования
fn main() {
    // Создаем тестовых пользователей
    let users = vec![
        User {
            id: 1,
            nickname: "alice_rust".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            nickname: "bob_developer".to_string(),
            email: "bob@example.com".to_string(),
        },
        User {
            id: 3,
            nickname: "charlie_rustacean".to_string(),
            email: "charlie@example.com".to_string(),
        },
        User {
            id: 4,
            nickname: "david_coder".to_string(),
            email: "david@example.com".to_string(),
        },
        User {
            id: 5,
            nickname: "eve_rust".to_string(),
            email: "eve@example.com".to_string(),
        },
    ];
    
    // Создаем репозиторий
    let repo = ImUsersRepository::new(users);
    
    println!("Всего пользователей: {}", repo.len());
    
    // Тест 1: Получение пользователя по ID
    if let Some(user) = repo.get_user_by_id(1) {
        println!("User 1: {} ({})", user.nickname, user.email);
    }
    
    // Тест 2: Получение нескольких пользователей
    let user_ids = vec![1, 3, 5];
    let users = repo.get_users_by_ids(&user_ids);
    println!("\nПользователи с ID {:?}:", user_ids);
    for user in users {
        println!("  - {} (ID: {})", user.nickname, user.id);
    }
    
    // Тест 3: Поиск по nickname
    let search_term = "rust";
    let matching_ids = repo.find_user_ids_by_nickname(search_term);
    println!("\nПользователи с '{}' в nickname: {:?}", search_term, matching_ids);
    
    // Тест 4: С неизменяемостью
    let repo2 = repo.add_user(User {
        id: 6,
        nickname: "frank_newbie".to_string(),
        email: "frank@example.com".to_string(),
    });
    
    println!("\nПосле добавления пользователя:");
    println!("  repo1 содержит {} пользователей", repo.len());
    println!("  repo2 содержит {} пользователей", repo2.len());
    
    // Тест 5: Работа с оптимизированной версией
    println!("\n--- Оптимизированная версия ---");
    let users = vec![
        User {
            id: 10,
            nickname: "john_doe".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: 11,
            nickname: "jane_doe".to_string(),
            email: "jane@example.com".to_string(),
        },
        User {
            id: 12,
            nickname: "john_smith".to_string(),
            email: "john.smith@example.com".to_string(),
        },
    ];
    
    let indexed_repo = IndexedImUsersRepository::new(users);
    
    // Быстрый поиск по точному nickname
    let exact_match = indexed_repo.find_user_ids_by_exact_nickname("john_doe");
    println!("Точный поиск 'john_doe': {:?}", exact_match);
    
    // Поиск по подстроке (медленнее)
    let substring_match = indexed_repo.find_user_ids_by_nickname("john");
    println!("Поиск по подстроке 'john': {:?}", substring_match);
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_user_by_id() {
        let repo = create_test_repository();
        
        // Существующий пользователь
        assert!(repo.get_user_by_id(1).is_some());
        assert_eq!(repo.get_user_by_id(1).unwrap().nickname, "test_user_1");
        
        // Несуществующий пользователь
        assert!(repo.get_user_by_id(999).is_none());
    }
    
    #[test]
    fn test_get_users_by_ids() {
        let repo = create_test_repository();
        
        // Существующие IDs
        let users = repo.get_users_by_ids(&[1, 2]);
        assert_eq!(users.len(), 2);
        assert!(users.iter().any(|u| u.id == 1));
        assert!(users.iter().any(|u| u.id == 2));
        
        // Смесь существующих и несуществующих
        let users = repo.get_users_by_ids(&[1, 999, 2]);
        assert_eq!(users.len(), 2); // Только существующие
        
        // Все несуществующие
        let users = repo.get_users_by_ids(&[999, 1000]);
        assert!(users.is_empty());
    }
    
    #[test]
    fn test_find_user_ids_by_nickname() {
        let repo = create_test_repository();
        
        // Поиск по подстроке
        let ids = repo.find_user_ids_by_nickname("user");
        assert_eq!(ids.len(), 3); // Все три пользователя содержат "user"
        
        // Поиск по точному совпадению (если бы был индекс)
        let ids = repo.find_user_ids_by_nickname("test_user_1");
        assert_eq!(ids.len(), 1);
        assert!(ids.contains(&1));
        
        // Поиск без учета регистра
        let ids = repo.find_user_ids_by_nickname("TEST");
        assert_eq!(ids.len(), 3);
        
        // Поиск несуществующего
        let ids = repo.find_user_ids_by_nickname("xyz");
        assert!(ids.is_empty());
    }
    
    #[test]
    fn test_immutability() {
        let repo1 = create_test_repository();
        let initial_count = repo1.len();
        
        // Добавляем пользователя в новую версию
        let repo2 = repo1.add_user(User {
            id: 100,
            nickname: "new_user".to_string(),
            email: "new@example.com".to_string(),
        });
        
        // Проверяем, что исходная версия не изменилась
        assert_eq!(repo1.len(), initial_count);
        assert!(repo1.get_user_by_id(100).is_none());
        
        // Проверяем, что новая версия содержит нового пользователя
        assert_eq!(repo2.len(), initial_count + 1);
        assert!(repo2.get_user_by_id(100).is_some());
    }
    
    #[test]
    fn test_remove_user() {
        let repo1 = create_test_repository();
        let initial_count = repo1.len();
        
        // Удаляем пользователя
        let repo2 = repo1.remove_user(1);
        
        // Проверяем, что пользователь удален в новой версии
        assert_eq!(repo2.len(), initial_count - 1);
        assert!(repo2.get_user_by_id(1).is_none());
        
        // Проверяем, что исходная версия не изменилась
        assert_eq!(repo1.len(), initial_count);
        assert!(repo1.get_user_by_id(1).is_some());
    }
    
    fn create_test_repository() -> ImUsersRepository {
        let users = vec![
            User {
                id: 1,
                nickname: "test_user_1".to_string(),
                email: "user1@example.com".to_string(),
            },
            User {
                id: 2,
                nickname: "test_user_2".to_string(),
                email: "user2@example.com".to_string(),
            },
            User {
                id: 3,
                nickname: "test_user_3".to_string(),
                email: "user3@example.com".to_string(),
            },
        ];
        
        ImUsersRepository::new(users)
    }
}
*/