// Управление памятью вручную с помощью сырых указателей.
use std::ptr::{
            self,
            NonNull // не null сырой указатель (*mut T but non-zero and covariant)
        } ;
/*
Атомарные типы обеспечивают примитивную связь между потоками через общую 
память и являются строительными блоками для других типов, используемых в 
параллельных вычислениях.
*/
use std::sync::atomic::{
                AtomicPtr,  // Тип указателя, который можно безопасно использовать совместно несколькими потоками.
                Ordering    // Порядок операций в памяти определяет способ синхронизации памяти атомарными операциями.
            } ;
use std::marker::PhantomData;
use std::fmt;

// Узел двусвязного списка
struct Node<T> {
    data: T,
    next: AtomicPtr<Node<T>>,
    prev: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    // создание нового узла
    fn new(data: T) -> *mut Self {
        Box::into_raw(     // *mut Node<T> (Потребляет Box, возвращая обернутый сырой указатель. Box into raw.)
            Box::new(      // Box<Node<T>> (Выделяет память в куче и затем размезает x в ней.)
                Self {
                    data,
                    next: AtomicPtr::new(   // Сырой указательный тип который может быть безопасно разделён между потоками.
                                ptr::null_mut() // Создаёт null изменяемый сырой указатель.
                            ),
                    prev: AtomicPtr::new(
                                ptr::null_mut::<Node<T>>()  // или так
                            ),
                }
            )
        )
    }
    
    // удаление узла
    unsafe fn free(node: *mut T) {
        if ! node.is_null() // проверка что node is_null
        {
            drop(
                /*
                После вызова этой функции исходный указатель становится 
                собственностью полученного объекта Box. В частности, 
                деструктор Box вызовет деструктор класса T и освободит 
                выделенную память. Для обеспечения безопасности память 
                должна быть выделена в соответствии с используемой 
                классом Box структурой памяти.
                VVV
                 */
                Box::from_raw(node) // unsafe function
            );
        }
    }
}

/*
        +----+   +----+  +----+
        +    v   +    v  +    v    << Следующий узел
     Node3   Node2   Node1   null
     +  ^    +   ^    +  ^         << Превыдущий узел
     |  +----+   +----+  |
     +-------------------+
 */
// Управляющая структура двусвязного списка
pub struct ConcurrentDoublyLinkedList<T> {
    head: AtomicPtr<Node<T>>,   // начальный узел списка
    tail: AtomicPtr<Node<T>>,   // конечный узел списка
    len: std::sync::atomic::AtomicUsize,    // длина списка. Целый тип который может быть безопасно разделён между потоками.
    _marker: PhantomData<T>,    // Тип нулевого ращмера использованный чтобы маркировать предметы что "действовать подобно" что они владеют типом T (Нужен из за атомарных указателей).
}

impl<T> ConcurrentDoublyLinkedList<T> {
    
    // создание нового пустого списка
    fn new() ->Self {
        Self { 
            head: AtomicPtr::new(ptr::null_mut()),  // так
            tail: AtomicPtr::new(ptr::null_mut::<Node<T>>()), // или так
            len: std::sync::atomic::AtomicUsize::new(0),
            _marker: PhantomData,
        }
    }

    // Добавление в начало списка (lock-free)
    pub fn push_front(&self, data: T) {
        // создание нового узла
        let new_node = Node::new(data);

        loop {
            // загрузить данные по заголовку списка
            let head = 
                    self
                        .head
                        .load( // -> *mut Node<T>, Загрузить значение из указателя.
                            Ordering::Acquire   // Данный порядок применяется только к операциям, способным выполнять загрузку.
                        ) ;
            // модификация next и prev для new_node
            unsafe {
                // установить next значение для нового узла
                (*new_node)
                    .next
                    .store( // Сохранить значение в указателе.
                        head, 
                        Ordering::Relaxed   // Никаких ограничений по порядку выполнения, только атомарные операции.
                    ) ;
                // установить prev значение для нового узла в null
                (*new_node)
                    .prev
                    .store( // Сохранить значение в указателе.
                        ptr::null_mut(),
                        Ordering::Relaxed   // Никаких ограничений по порядку выполнения, только атомарные операции.
                    ) ;
            }
            
            // Попытка атомарно установить новый узел как голову
            match self
                    .head
                    // Сохраняет значение в указатель, если self.head значение является 
                    // таким же как предоставленное current значением.
                    // VVV
                    .compare_exchange_weak (
                head,   // значение с которым сравниваем
                new_node,       // устанавливаемое значение
                Ordering::Release,  // success описывает необходимый порядок для операции чтения-изменения-записи, которая выполняется, если сравнение с текущей переменной прошло успешно. (порядок доступа к памяти для этой операции.) 
                Ordering::Relaxed,  // failure описывает необходимый порядок для операции загрузки, которая выполняется, если сравнение не удалось.
            ) {
                Ok(_) => { // Успешно установили новый заголовок списка
                    if head
                        .is_null()  // старый заголовок списка is_null
                    {
                        // Если список был пуст, новый узел также является последним
                        self
                            .tail
                            .store(
                                new_node, 
                                Ordering::Release
                            ) ;
                    } else {
                        // Связываем старую голову с новой
                        unsafe {
                            (*head)
                                .prev
                                .store(
                                    new_node, 
                                    Ordering::Release
                                ) ;
                        }
                    }
                    self.len.fetch_add(1, Ordering::SeqCst);
                    break;
                }
                Err(_) => { // Другая операция изменила head
                    continue;   // повторяем попытку
                }
            }
        }
    }

}

fn main() {
    println!("Implement me!");
}
/*
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::marker::PhantomData;
use std::fmt;

// Узел двусвязного списка
struct Node<T> {
    data: T,
    next: AtomicPtr<Node<T>>,
    prev: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            data,
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
        }))
    }
    
    unsafe fn free(node: *mut Self) {
        if !node.is_null() {
            drop(Box::from_raw(node));
        }
    }
}

// Основная структура двусвязного списка
pub struct ConcurrentDoublyLinkedList<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
    len: std::sync::atomic::AtomicUsize,
    _marker: PhantomData<T>,
}

impl<T> ConcurrentDoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            tail: AtomicPtr::new(ptr::null_mut()),
            len: std::sync::atomic::AtomicUsize::new(0),
            _marker: PhantomData,
        }
    }
    
    // Добавление в начало списка (lock-free)
    pub fn push_front(&self, data: T) {
        let new_node = Node::new(data);
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next.store(head, Ordering::Relaxed);
                (*new_node).prev.store(ptr::null_mut(), Ordering::Relaxed);
            }
            
            // Попытка атомарно установить новый узел как голову
            match self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    // Успешно установили новую голову
                    if head.is_null() {
                        // Если список был пуст, новый узел также является хвостом
                        self.tail.store(new_node, Ordering::Release);
                    } else {
                        // Связываем старую голову с новой
                        unsafe {
                            (*head).prev.store(new_node, Ordering::Release);
                        }
                    }
                    self.len.fetch_add(1, Ordering::SeqCst);
                    break;
                }
                Err(_) => {
                    // Другая операция изменила head, повторяем попытку
                    continue;
                }
            }
        }
    }
    
    // Добавление в конец списка (lock-free)
    pub fn push_back(&self, data: T) {
        let new_node = Node::new(data);
        
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            unsafe {
                (*new_node).prev.store(tail, Ordering::Relaxed);
                (*new_node).next.store(ptr::null_mut(), Ordering::Relaxed);
            }
            
            // Попытка атомарно установить новый узел как хвост
            match self.tail.compare_exchange_weak(
                tail,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    // Успешно установили новый хвост
                    if tail.is_null() {
                        // Если список был пуст, новый узел также является головой
                        self.head.store(new_node, Ordering::Release);
                    } else {
                        // Связываем старый хвост с новым
                        unsafe {
                            (*tail).next.store(new_node, Ordering::Release);
                        }
                    }
                    self.len.fetch_add(1, Ordering::SeqCst);
                    break;
                }
                Err(_) => {
                    // Другая операция изменила tail, повторяем попытку
                    continue;
                }
            }
        }
    }
    
    // Удаление из начала списка (lock-free)
    pub fn pop_front(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None; // Список пуст
            }
            
            unsafe {
                let next = (*head).next.load(Ordering::Acquire);
                
                // Попытка атомарно переместить head на следующий узел
                match self.head.compare_exchange_weak(
                    head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        if next.is_null() {
                            // Если список стал пустым, обнуляем tail
                            self.tail.store(ptr::null_mut(), Ordering::Release);
                        } else {
                            // Обнуляем prev у нового head
                            (*next).prev.store(ptr::null_mut(), Ordering::Release);
                        }
                        
                        self.len.fetch_sub(1, Ordering::SeqCst);
                        
                        // Извлекаем данные и освобождаем память
                        let data = ptr::read(&(*head).data);
                        Node::free(head);
                        return Some(data);
                    }
                    Err(_) => {
                        // Другая операция изменила head, повторяем попытку
                        continue;
                    }
                }
            }
        }
    }
    
    // Удаление из конца списка (lock-free)
    pub fn pop_back(&self) -> Option<T> {
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            if tail.is_null() {
                return None; // Список пуст
            }
            
            unsafe {
                let prev = (*tail).prev.load(Ordering::Acquire);
                
                // Попытка атомарно переместить tail на предыдущий узел
                match self.tail.compare_exchange_weak(
                    tail,
                    prev,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        if prev.is_null() {
                            // Если список стал пустым, обнуляем head
                            self.head.store(ptr::null_mut(), Ordering::Release);
                        } else {
                            // Обнуляем next у нового tail
                            (*prev).next.store(ptr::null_mut(), Ordering::Release);
                        }
                        
                        self.len.fetch_sub(1, Ordering::SeqCst);
                        
                        // Извлекаем данные и освобождаем память
                        let data = ptr::read(&(*tail).data);
                        Node::free(tail);
                        return Some(data);
                    }
                    Err(_) => {
                        // Другая операция изменила tail, повторяем попытку
                        continue;
                    }
                }
            }
        }
    }
    
    // Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
    
    // Получение длины (приблизительное значение в многопоточной среде)
    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }
    
    // Итерация по списку (не потокобезопасна для модификаций во время итерации)
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.load(Ordering::Acquire),
            list: self,
        }
    }
    
    // Очистка списка (lock-free)
    pub fn clear(&self) {
        while self.pop_front().is_some() {}
    }
}

// Реализация Drop для очистки ресурсов
impl<T> Drop for ConcurrentDoublyLinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// Итератор по списку
pub struct Iter<'a, T> {
    current: *mut Node<T>,
    list: &'a ConcurrentDoublyLinkedList<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let node = &*self.current;
                let result = &node.data;
                self.current = node.next.load(Ordering::Acquire);
                Some(result)
            }
        }
    }
}

// Обратный итератор
pub struct ReverseIter<'a, T> {
    current: *mut Node<T>,
    list: &'a ConcurrentDoublyLinkedList<T>,
}

impl<'a, T> ReverseIter<'a, T> {
    pub fn new(list: &'a ConcurrentDoublyLinkedList<T>) -> Self {
        Self {
            current: list.tail.load(Ordering::Acquire),
            list,
        }
    }
}

impl<'a, T> Iterator for ReverseIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let node = &*self.current;
                let result = &node.data;
                self.current = node.prev.load(Ordering::Acquire);
                Some(result)
            }
        }
    }
}

// Реализация Debug для отладки
impl<T: fmt::Debug> fmt::Debug for ConcurrentDoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// Потокобезопасные методы для работы с произвольными позициями
impl<T: Clone> ConcurrentDoublyLinkedList<T> {
    // Вставка после определенного узла (lock-free с использованием указателя на узел)
    pub unsafe fn insert_after(&self, node_ptr: *mut Node<T>, data: T) -> Result<*mut Node<T>, &'static str> {
        if node_ptr.is_null() {
            return Err("Null pointer provided");
        }
        
        let new_node = Node::new(data);
        
        unsafe {
            let next = (*node_ptr).next.load(Ordering::Acquire);
            
            // Устанавливаем связи нового узла
            (*new_node).prev.store(node_ptr, Ordering::Relaxed);
            (*new_node).next.store(next, Ordering::Relaxed);
            
            // Попытка атомарно вставить новый узел
            match (*node_ptr).next.compare_exchange_weak(
                next,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    if !next.is_null() {
                        // Обновляем prev у старого следующего узла
                        (*next).prev.store(new_node, Ordering::Release);
                    } else {
                        // Если вставляли после хвоста, обновляем tail
                        self.tail.store(new_node, Ordering::Release);
                    }
                    
                    self.len.fetch_add(1, Ordering::SeqCst);
                    Ok(new_node)
                }
                Err(_) => {
                    // Не удалось вставить, освобождаем память
                    Node::free(new_node);
                    Err("Concurrent modification detected")
                }
            }
        }
    }
    
    // Удаление произвольного узла (lock-free)
    pub unsafe fn remove_node(&self, node_ptr: *mut Node<T>) -> Result<T, &'static str> {
        if node_ptr.is_null() {
            return Err("Null pointer provided");
        }
        
        unsafe {
            let prev = (*node_ptr).prev.load(Ordering::Acquire);
            let next = (*node_ptr).next.load(Ordering::Acquire);
            
            // Обновляем связи соседних узлов
            if !prev.is_null() {
                // Попытка атомарно обновить next у предыдущего узла
                match (*prev).next.compare_exchange_weak(
                    node_ptr,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {}
                    Err(_) => return Err("Concurrent modification detected"),
                }
            } else {
                // Удаляемый узел - голова
                match self.head.compare_exchange_weak(
                    node_ptr,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {}
                    Err(_) => return Err("Concurrent modification detected"),
                }
            }
            
            if !next.is_null() {
                // Попытка атомарно обновить prev у следующего узла
                match (*next).prev.compare_exchange_weak(
                    node_ptr,
                    prev,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {}
                    Err(_) => return Err("Concurrent modification detected"),
                }
            } else {
                // Удаляемый узел - хвост
                match self.tail.compare_exchange_weak(
                    node_ptr,
                    prev,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {}
                    Err(_) => return Err("Concurrent modification detected"),
                }
            }
            
            self.len.fetch_sub(1, Ordering::SeqCst);
            
            // Извлекаем данные и освобождаем память
            let data = ptr::read(&(*node_ptr).data);
            Node::free(node_ptr);
            Ok(data)
        }
    }
}

// Безопасный враппер для работы с узлами по значению
impl<T: PartialEq + Clone> ConcurrentDoublyLinkedList<T> {
    // Поиск узла по значению
    pub fn find_node(&self, value: &T) -> Option<*mut Node<T>> {
        let mut current = self.head.load(Ordering::Acquire);
        
        while !current.is_null() {
            unsafe {
                if &(*current).data == value {
                    return Some(current);
                }
                current = (*current).next.load(Ordering::Acquire);
            }
        }
        
        None
    }
    
    // Удаление по значению (первое вхождение)
    pub fn remove_value(&self, value: &T) -> Option<T> {
        if let Some(node_ptr) = self.find_node(value) {
            unsafe {
                match self.remove_node(node_ptr) {
                    Ok(data) => Some(data),
                    Err(_) => None,
                }
            }
        } else {
            None
        }
    }
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Barrier;
    use std::time::Duration;
    
    #[test]
    fn test_basic_operations() {
        let list = ConcurrentDoublyLinkedList::new();
        
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        
        list.push_front(1);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
        
        list.push_back(2);
        assert_eq!(list.len(), 2);
        
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert!(list.is_empty());
    }
    
    #[test]
    fn test_iteration() {
        let list = ConcurrentDoublyLinkedList::new();
        
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        
        let items: Vec<_> = list.iter().collect();
        assert_eq!(items, vec![&1, &2, &3]);
        
        let rev_items: Vec<_> = ReverseIter::new(&list).collect();
        assert_eq!(rev_items, vec![&3, &2, &1]);
    }
    
    #[test]
    fn test_concurrent_push_front() {
        let list = std::sync::Arc::new(ConcurrentDoublyLinkedList::new());
        let barrier = std::sync::Arc::new(Barrier::new(10));
        
        let mut handles = Vec::new();
        
        for i in 0..10 {
            let list_clone = list.clone();
            let barrier_clone = barrier.clone();
            
            handles.push(thread::spawn(move || {
                barrier_clone.wait();
                list_clone.push_front(i);
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Все 10 элементов должны быть в списке
        assert_eq!(list.len(), 10);
        
        // Проверяем, что все элементы есть (порядок не гарантирован)
        let mut found = [false; 10];
        for &item in list.iter() {
            found[item as usize] = true;
        }
        
        assert!(found.iter().all(|&f| f));
    }
    
    #[test]
    fn test_concurrent_push_pop() {
        let list = std::sync::Arc::new(ConcurrentDoublyLinkedList::new());
        
        // Инициализируем список
        for i in 0..5 {
            list.push_back(i);
        }
        
        let producer = {
            let list = list.clone();
            thread::spawn(move || {
                for i in 5..10 {
                    list.push_back(i);
                    thread::sleep(Duration::from_micros(10));
                }
            })
        };
        
        let consumer = {
            let list = list.clone();
            thread::spawn(move || {
                let mut count = 0;
                while count < 10 {
                    if let Some(_) = list.pop_front() {
                        count += 1;
                    }
                    thread::sleep(Duration::from_micros(5));
                }
            })
        };
        
        producer.join().unwrap();
        consumer.join().unwrap();
        
        // Список должен быть пуст
        assert!(list.is_empty());
    }
    
    #[test]
    fn test_find_and_remove() {
        let list = ConcurrentDoublyLinkedList::new();
        
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(2); // Дубликат
        
        // Находим первый узел со значением 2
        let node_ptr = list.find_node(&2);
        assert!(!node_ptr.unwrap().is_null());
        
        // Удаляем по значению
        assert_eq!(list.remove_value(&2), Some(2));
        
        // В списке осталось [1, 3, 2]
        let items: Vec<_> = list.iter().collect();
        assert_eq!(items.len(), 3);
    }
    
    #[test]
    fn test_clear() {
        let list = ConcurrentDoublyLinkedList::new();
        
        for i in 0..100 {
            list.push_back(i);
        }
        
        assert_eq!(list.len(), 100);
        
        list.clear();
        
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
    
    #[test]
    fn test_stress_concurrent() {
        let list = std::sync::Arc::new(ConcurrentDoublyLinkedList::new());
        let num_threads = 8;
        let num_operations = 1000;
        
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let list = list.clone();
            let barrier = barrier.clone();
            
            handles.push(thread::spawn(move || {
                barrier.wait();
                
                for i in 0..num_operations {
                    if thread_id % 2 == 0 {
                        list.push_front(thread_id * 1000 + i);
                    } else {
                        list.push_back(thread_id * 1000 + i);
                    }
                    
                    // Иногда удаляем элементы
                    if i % 10 == 0 {
                        if thread_id % 2 == 0 {
                            let _ = list.pop_front();
                        } else {
                            let _ = list.pop_back();
                        }
                    }
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Проверяем, что список не в поврежденном состоянии
        let mut count = 0;
        let mut current = list.head.load(Ordering::Acquire);
        
        while !current.is_null() {
            unsafe {
                let node = &*current;
                // Проверяем связи
                let next = node.next.load(Ordering::Acquire);
                if !next.is_null() {
                    unsafe {
                        let next_node = &*next;
                        let prev_of_next = next_node.prev.load(Ordering::Acquire);
                        assert_eq!(prev_of_next, current);
                    }
                }
                current = next;
                count += 1;
            }
        }
        
        // Проверяем, что счетчик длины примерно правильный
        let len = list.len();
        assert!(count == len, "Counted {} nodes, but len() is {}", count, len);
        
        println!("Stress test completed. Final length: {}", len);
    }
}

// Пример использования
fn main() {
    // Создаем потокобезопасный список
    let list = ConcurrentDoublyLinkedList::new();
    
    // Добавляем элементы из разных потоков
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let list_ref = &list;
            thread::spawn(move || {
                for j in 0..10 {
                    if i % 2 == 0 {
                        list_ref.push_front(i * 100 + j);
                    } else {
                        list_ref.push_back(i * 100 + j);
                    }
                }
            })
        })
        .collect();
    
    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Выводим содержимое
    println!("List contents:");
    for item in list.iter() {
        print!("{} ", item);
    }
    println!();
    
    println!("List length: {}", list.len());
    
    // Очищаем список
    list.clear();
    println!("After clear, list is empty: {}", list.is_empty());
}
*/