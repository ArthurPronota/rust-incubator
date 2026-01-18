use std::borrow::Cow;
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
            Box::new(      // Box<Node<T>> (Выделяет память в куче и затем размещает x в ней.)
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
    
    /*
    // удаление узла
    unsafe fn free(node: *mut T) {
        if ! node.is_null() // проверка что node is_null
        {
            drop(
                // После вызова этой функции исходный указатель становится 
                // собственностью полученного объекта Box. В частности, 
                // деструктор Box вызовет деструктор класса T и освободит 
                // выделенную память. Для обеспечения безопасности память 
                // должна быть выделена в соответствии с используемой 
                // классом Box структурой памяти.
                // VVV
                Box::from_raw(node) // unsafe function
            );
        }

    }
     */

    // Изменяем метод free для безопасного извлечения данных
    unsafe fn take_data(node: *mut Node<T>) -> T {
        // Воссоздаёт объект Box<Node<T>> из сырого указателя node который 
        // ранее был преобразован из Box с помощью into_raw в Node<T>::new() методе
        // Это преобразование: *mut Node<T> -> Node<T>
        let boxed_node = Box::from_raw(node);
        boxed_node.data  // Данные перемещаются из Box
    }
    
}

/*  Структура списка:

              +------+   +----+  +----+
              +      v   +    v  +    v    << Следующий узел
       null   Node3  Node2   Node1   null
         ^    +   ^   +  ^    +            << Предыдущий узел
         +----+   +---+  +----+

                        Или так:

       null <- Node3 <-> Node2 <-> Node1 -> null

       <- предыдущий узел
       -> следуюший узел

 */
// Управляющая структура двусвязного списка
pub struct ConcurrentDoublyLinkedList<T> {
    head: AtomicPtr<Node<T>>,   // начальный узел списка
    tail: AtomicPtr<Node<T>>,   // конечный узел списка
    len: std::sync::atomic::AtomicUsize,    // длина списка. Целый тип который может быть безопасно разделён между потоками.
    _marker: PhantomData<T>,    // Тип нулевого размера использованный чтобы маркировать предметы что "действовать подобно" что они хранит значение типа T (Нужен из за атомарных указателей).
}

impl<T> ConcurrentDoublyLinkedList<T> {

    // создание нового пустого списка
    fn new() ->Self {
        Self { 
            head: AtomicPtr::new(ptr::null_mut()),  // так
            tail: AtomicPtr::new(ptr::null_mut::<Node<T>>()), // или так
            len: std::sync::atomic::AtomicUsize::new(0), // Creates a new atomic integer.
            _marker: PhantomData,
        }
    }

    // Добавить узел в начало списка (lock-free)
    pub fn push_front(&self, data: T) {
        // создание нового узла
        let new_node = Node::new(data);

        loop {
            // загрузить данные по полю head текущего заголовка списка
            let head = 
                    self
                        .head
                        .load( // -> *mut Node<T>, Загрузить значение из указателя.
                            Ordering::Acquire   // Данный порядок применяется только к операциям, способным выполнять загрузку.
                        ) ;
            // модификация next и prev для new_node
            unsafe {  // разименовывание сырого указаиеоя является небезопасным и требует unsafe блока
                // установить next = текущему head для нового узла
                (*new_node)
                    .next
                    .store( // Сохранить значение в указателе.
                        head, 
                        Ordering::Relaxed   // Никаких ограничений по порядку выполнения, только атомарные операции.
                    ) ;

                // установить prev = null для нового узла
                (*new_node)
                    .prev
                    .store( // Сохранить значение в указателе.
                        ptr::null_mut(),
                        Ordering::Relaxed   // Никаких ограничений по порядку выполнения, только атомарные операции.
                    ) ;
            }
            
            // Попытка атомарно установить новый узел как заголовок списка
            match self
                    .head
                    // Сохраняет значение в указатель, если self.head является 
                    // таким же как загруженное head 
                    // VVV
                    .compare_exchange_weak (
                head,   // значение с которым сравниваем
                new_node,       // устанавливаемое значение
                Ordering::Release,  // success описывает необходимый порядок для операции чтения-изменения-записи, которая выполняется, если сравнение с текущей переменной прошло успешно. (порядок доступа к памяти для этой операции.) 
                Ordering::Relaxed,  // failure описывает необходимый порядок для операции загрузки, которая выполняется, если сравнение не удалось.
                    )
            {
                Ok(_) => { // Успешно установили новый заголовок списка
                    if head
                        .is_null()  // список пуст (старый заголовок списка is_null)
                    {
                        // Если список был пуст, новый узел также является последним
                        self
                            .tail
                            .store(
                                new_node, 
                                Ordering::Release
                            ) ;
                    } else {
                        // Связываем старый заголовок списка с ноывм заголовком списка
                        unsafe {    // unsafe из за разименовыввния сырых указателей
                            (*head)
                                .prev
                                .store(
                                    new_node, 
                                    Ordering::Release
                                ) ;
                        }
                    }

                    // уставливаем длину списка
                    self
                        .len
                        .fetch_add( // Добавляет значение к текущему, возвращая предыдущее.
                            1, 
                            Ordering::SeqCst
                        ) ;
                        
                    break;
                },
                Err(_) => { // Другая операция изменила head
                    continue;   // повторяем попытку
                }
            }
        }
    }

    // Добавить узел в конец списка (lock-free)
    pub fn push_back(&self, data: T) {
        
        // создание нового узла
        let new_node = Node::new(data);
        
        loop {
            let tail = self
                                        .tail
                                        .load(  // загрузка даных о конце списка
                                            Ordering::Acquire
                                        );
            unsafe {  // unsafe нужун при разименовывании raw point
                (*new_node)
                    .prev
                    .store( // установить предыдущий узел для нового узла
                        tail,
                        Ordering::Relaxed
                    );
                (*new_node)
                    .next
                    .store( // установить следуюшийузел для нового узла
                        ptr::null_mut(),
                        Ordering::Relaxed
                    );
            }
            
            // Попытка атомарно установить новый узел как окончание списка
            match self
                    .tail
                    // Сохраняет значение в указатель, если self.tail является 
                    // таким же как загруженное tail 
                    // VVV                    
                    .compare_exchange_weak(
                tail,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {  // Успешно установили новый хвост
                    if tail.is_null() { // tail is null
                        // Если список был пуст, новый узел также является заголовком списка
                        self
                            .head
                            .store( // сохранить новый заголовок
                                new_node,
                                Ordering::Release
                            );
                    } else {
                        // Связываем старый хвост с новым
                        unsafe {
                            (*tail)
                                .next
                                .store(
                                    new_node,
                                    Ordering::Release
                                );
                        }
                    }
                    self
                        .len
                        .fetch_add( // добавляем 1 к количеству узлов списка
                            1, 
                            Ordering::SeqCst
                        );

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
            let head = self
                                        .head
                                        .load(  // загрузка заголовка списка
                                            Ordering::Acquire
                                        );
            if head.is_null() {
                return None; // Список пуст
            }
            
            unsafe {
                // загрузка узла следующего за заголовочным узлом списка
                let next = (*head)
                                            .next
                                            .load(
                                                Ordering::Acquire
                                            );
                
                // Попытка атомарно переместить head на следующий узел
                match self
                        .head
                        .compare_exchange_weak(
                    head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        if next.is_null() {
                            // Если список стал пустым, обнуляем tail
                            self
                                .tail
                                .store(
                                    ptr::null_mut(),
                                    Ordering::Release
                                );
                        } else {
                            // Обнуляем prev у нового head
                            (*next)
                                .prev
                                .store(
                                    ptr::null_mut(),
                                    Ordering::Release
                                );
                        }
                        
                        self
                            .len
                            .fetch_sub( // Вычитаем из текущего значения, возвращая предыдущее значение.
                                1,
                                Ordering::SeqCst
                            );
                        
                        /*
                        // Проблема в текущем коде заключается в 
                        // использовании ptr::read для чтения данных из 
                        // узла. Для типов, которые не реализуют Copy 
                        // (например, String), это небезопасно и приведёт
                        // к двойному освобождению памяти или неопределённому
                        //  поведению.
                                                
                        // Извлекаем данные и освобождаем память
                        let data = ptr::read(&(*head).data);
                        //std::mem::forget(&(*head).data);
                        Node::free(head);
                         */
                        let data = Node::take_data(head) ;
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
                        
                        /*
                        // Проблема в текущем коде заключается в 
                        // использовании ptr::read для чтения данных из 
                        // узла. Для типов, которые не реализуют Copy 
                        // (например, String), это небезопасно и приведёт
                        // к двойному освобождению памяти или неопределённому
                        //  поведению.

                        // Извлекаем данные и освобождаем память
                        let data = ptr::read(&(*tail).data);
                        Node::free(tail);
                         */
                        let data = Node::take_data(tail) ;
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
        self
            .head
            .load(
                Ordering::Acquire
            )
            .is_null()
    }
    
    // Получение длины (приблизительное значение в многопоточной среде)
    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    // Очистка списка (lock-free)
    pub fn clear(&self) {
        while self.pop_front().is_some() 
        {
        }
    }    

    // создание структуры итератора для чтения с начала списка
    pub fn get_iter(&self) -> Iter<'_, T> {
        Iter {
            current: self
                        .head
                        .load(
                            Ordering::Acquire
                        ),
            _marker: PhantomData,
        }
    }

    // создание структуры итератора для чтения с конца списка
    pub fn get_reviter(&self) ->RevIter<'_, T> {
        RevIter { 
            current: self.tail.load(Ordering::Acquire), 
            _marker: PhantomData 
        }
    }

}

// реализация Drop для ConcurrentDoublyLinkedList
impl<T> Drop for ConcurrentDoublyLinkedList<T>  {
    fn drop(&mut self) {
        self.clear();
    }
}

// Структура для итератора прямого обхода без модификации
struct Iter<'a, T> {
    current:    *mut Node<T>,                       // текущий узел
    _marker:    PhantomData<&'a Node<T>>,           // маркерный трейт
}

// реализация итератора прямого обхода для структуры Iter
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T ;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current.is_null() {
            return None ;
        }

        unsafe {    // unsafe для разименоывания сыпых указателей
            let result = &(*self.current).data ;
            self.current = (*self.current)
                                    .next
                                    .load(
                                        Ordering::Acquire
                                    ) ;
            Some(result)                                    
        }
    }
}

// Структура для итератора обратного обхода без модификации
struct RevIter<'a, T> {
    current:    *mut Node<T>,                       // текущий узел
    _marker:    PhantomData<&'a Node<T>>,           // маркерный трейт
}

// реализация итератора обратного обхода без модификации
impl<'a, T> Iterator for RevIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let result = &(*self.current).data ;
            self.current = (*self.current)
                                .prev
                                .load(
                                    Ordering::Acquire
                                ) ;
            Some(result)                                
        }
    }
}
fn main() {

    // Обшее тестирование
    println!("--- Обшее Тестирование ---");
    let list = ConcurrentDoublyLinkedList::new();

    list.push_front(
            "abc".to_owned()
        );

    println!("pop value: {:?}, len: {}, is empty: {}", 
                list.pop_front(), list.len(), list.is_empty()
        ) ;

    list.push_front(
            "abc".to_owned()
        );

    list.clear();
    println!("is_empty: {}", list.is_empty()) ;

    let list = ConcurrentDoublyLinkedList::new();

    list.push_front(String::from("Hello"));
    list.push_back(String::from("World"));
    list.push_front(String::from("First"));
    
    for unit in list.get_iter() {
        println!("unit: {}", unit) ;
    }

    list.clear();
    println!("is_empty: {}", list.is_empty()) ;

    let list = ConcurrentDoublyLinkedList::new();

    list.push_front(String::from("Hello"));
    list.push_back(String::from("World"));
    list.push_front(String::from("First"));

    for unit in list.get_reviter() {
        println!("rev_unit: {}", unit) ;
    }

    // Тестирование со строками
    let list = ConcurrentDoublyLinkedList::new();

    list.push_front(String::from("Hello"));
    list.push_back(String::from("World"));
    list.push_front(String::from("First"));

    println!("pop_front: {:?}", list.pop_front());
    println!("pop_back: {:?}", list.pop_back());
    println!("pop_back: {:?}", list.pop_back());
    println!("pop_back (should be None): {:?}", list.pop_back());

    println!("--- Тестирование с числами ---");
    
    let list2 = ConcurrentDoublyLinkedList::new();
    list2.push_front(42);
    list2.push_back(100);
    
    println!("pop_front: {:?}", list2.pop_front());
    println!("pop_back: {:?}", list2.pop_back());
    
    println!("--- Тестирование с Cow ---");
    
    let list3 = ConcurrentDoublyLinkedList::new();
    list3.push_front(Cow::from("Borrowed"));
    list3.push_back(Cow::from("Owned".to_string()));
    
    while let Some(value) = list3.pop_front() {
        println!("Value: {}", value);
    }

}