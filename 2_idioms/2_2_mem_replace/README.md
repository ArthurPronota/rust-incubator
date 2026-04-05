Шаг 2.2: Обмен значений с помощью `mem::replace`
=============================================

__Estimated time__: 1 day

Поскольку [Rust] подразумевает [move semantics][1] по умолчанию и довольно строгие [borrowing rules][2], часто возникают ситуации (особенно с большими структурами и перечислениями), когда проверка заимствований может не разрешать изменение значения на месте или замену значений, что довольно запутывает и приводит к созданию ненужных клонов (что, следовательно, влечет за собой избыточные затраты на производительность). Например:

```rust
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        // error: cannot move out of dereference of `&mut`-pointer
        let buf = self.buf;
        self.buf = Vec::new();
        buf
    }
}
```
В подобных ситуациях полезный и необходимый прием — использование [`mem::replace`] (или [`mem::swap`]). Он позволяет поменять местами два значения одного типа без перемещения элементов, частичной деструктуризации и путаницы со ссылками. Таким образом, приведенный выше пример просто преобразуется в:

```rust
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        mem::replace(&mut self.buf, Vec::new())
    }
}
```

Чтобы лучше понять назначение, структуру, ограничения и варианты использования функций [`mem::replace`], [`mem::swap`] и [`mem::take`], ознакомьтесь со следующими материалами:

- [Official `mem::replace` docs][`mem::replace`]
- [Official `mem::swap` docs][`mem::swap`]
- [Official `mem::take` docs][`mem::take`]
- [Karol Kuczmarski: Moving out of a container in Rust][4]
- [Ferrous Systems: Using `mem::take` to reduce heap allocations][6]

Ниже приведены некоторые примеры полезного применения этих функций.

## Сохранение собственных значений в измененных перечислениях

Подробное объяснение этой ситуации приведено в следующей статье:
- [Rust Design Patterns: `mem::replace` to keep owned values in changed enums][3]

> Проверка заимствований не позволит нам удалить `name` из перечисления (потому что там должно быть _что-то_). Конечно, мы могли бы использовать `.clone()` для имени и поместить клон в наш `MyEnum::B`, но это был бы пример антипаттерна «Клонировать, чтобы удовлетворить проверку заимствований». В любом случае, мы можем избежать дополнительного выделения памяти, заменив `e` только на изменяемое заимствование.
> 
> `mem::replace` позволяет нам заменить значение, заменив его чем-то другим. В данном случае мы помещаем пустую `String`, для которой не требуется выделение памяти. В результате мы получаем исходное `name` _в качестве принадлежащего значения_. Затем мы можем обернуть это в другое enum.
>
```rust
enum MyEnum {
    A { name: String },
    B { name: String },
}

fn swizzle(e: &mut MyEnum) {
    use self::MyEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::replace(name, String::new()) },
        B { ref mut name } => A { name: mem::replace(name, String::new()) },
    }
}
```

> Смотри, мама, никаких квот! А ещё ты можешь почувствовать себя Индианой Джонсом, выполняя это.

## Мутирующая встроенная коллекция

Рассмотрим следующую ситуацию:
```rust
use std::collections::HashSet ;

struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    fn apply_exclusions(&mut self) {
        self    // первое изменяемле заимствование &mut self
            .exclusions
            .drain(..)
            .for_each(|name| {
                self.remove_name(&name);    // второе изменяемле заимствование &mut self
            })
    }
    
    fn remove_name(&mut self, name: &str) {
        self.names.remove(name);
    }
}
```
который не компилируется из-за двух изменяемых заимствований:
```rust
error[E0500]: closure requires unique access to `*self` but it is already borrowed
  --> src/lib.rs:10:44
   |
10 |         self.exclusions.drain(..).for_each(|name| {
   |         ------------------------- -------- ^^^^^^ closure construction occurs here
   |         |                         |
   |         |                         first borrow later used by call
   |         borrow occurs here
11 |             self.remove_name(&name);
   |             ---- second borrow occurs due to use of `*self` in closure
```


Использование [`mem::take`] позволяет нам избежать проблемы с двумя изменяемыми заимствованиями практически без затрат (`Vec::default()` ничего не делает), путем замены значения во временной переменной:

```rust
impl Names {
    fn apply_exclusions(&mut self) {
        let mut exclusions = mem::take( // Замещает dest со значением по умолчанию T, возвращая предыдущее dest значение.
                                &mut self.exclusions
                             );
        exclusions
            .drain(..)  // Удаляет из вектора subslice, указанную заданным диапазоном, и возвращает двусторонний итератор по удаленному subslice.
            .for_each(|name| {
                self.remove_name(&name);  // (первое изменяемое заимствование &mut self) удаление из HashSet значения name
            });
    }

    // удаление из HashSet значения name
    fn remove_name(&mut self, name: &str) {
        self.names.remove(name);
    }
}
```

Стоит отметить, что эта проблема стала гораздо реже встречаться после [Функция непересекающегося захвата в замкнутых структурах была введена в версии Rust 2021 года. - (disjoint capture in closures had been introduced in 2021 Rust edition)][5]. Для иллюстрации мутация `self.name` намеренно выделена в отдельный метод, чтобы мы могли заблокировать весь `&mut self`. Если мы упростим код, он просто скомпилируется без проблем, поскольку изменяемые заимствования не пересекаются:
```rust
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    fn apply_exclusions(&mut self) {
        self.exclusions.drain(..).for_each(|name| {
            self.names.remove(&name);
        })
    }
}
```

Как это работает сейчас (Rust 2021+)

Начиная с редакции 2021 года, компилятор анализирует код и захватывает только те поля, которые реально используются. Это и называется «непересекающимся захватом» (disjoint capture).

В примере выше:

- Код видит, что вначале используется только `exclusions`.
- Он захватывает только `exclusions`.
- Поле `names` остается свободным, и его можно изменять параллельно в замыкании.

## Task

Улучшите и оптимизируйте код, содержащийся в [в crate этого шага](src/main.rs), чтобы сократить избыточные затраты на производительность.


## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- [В чём причина существования `mem::replace` в Rust? Что он нам даёт? Почему мы не можем решить те же проблемы без него?][020201]

- [Приведите несколько наглядных примеров использования `mem::replace` в Rust.][020202]

<hr>

<a name="q-020201"><h3>В чём причина существования `mem::replace` в [Rust]? Что он нам даёт? Почему мы не можем решить те же проблемы без него?</h3></a>

В Rust std::mem::replace остается критически важным инструментом для управления владением, особенно при работе с изменяемыми ссылками (`&mut`).

<h4>В чём причина существования mem::replace?</h4>

Основная причина — необходимость соблюдения инварианта безопасности памяти: «Переменная никогда не должна оставаться в неинициализированном состоянии».

В Rust вы не можете просто «забрать» значение из-за изменяемой ссылки, оставив там «дыру».

```rust
// Это НЕ СКОМПИЛИРУЕТСЯ
fn swap_out(r: &mut String) -> String {
    let val = *r; // Ошибка: попытка перемещения из-под ссылки.
                  // После этого r указывал бы на невалидную память.
    *r = String::from("new"); 
    val
}
```
`mem::replace` позволяет сделать это атомарно с точки зрения компилятора: он записывает новое значение в ячейку и одновременно возвращает старое.

<h4>Что он нам даёт?</h4>

1. Перемещение из-под ссылки (`&mut T`): Позволяет забрать владение объектом, который находится внутри структуры или массива, по ссылке.
2/ Соблюдение правил `Borrow Checker`: Он гарантирует, что объект по ссылке всегда остается валидным. В любой момент времени (даже если произойдет паника) по адресу ссылки будет лежать корректное значение.
3. Эффективность: Это низкоуровневая операция, которая часто превращается в эффективное копирование памяти (на уровне инструкций процессора), минимизируя накладные расходы.

<h4>Почему мы не можем решить проблемы без него?</h4>

Без `mem::replace` (или его аналогов) у нас было бы всего два плохих пути:

1. Клонирование (`Clone`):

    Вы могли бы склонировать данные, но это дорого (аллокация в куче). `mem::replace` работает с исходным объектом без лишних затрат.

```rust
let old = r.clone(); // Дорого
*r = new_val;
```

2. Использование `unsafe`:
    Вы могли бы использовать `ptr::read` и `ptr::write`, чтобы вручную переместить байты. Но это крайне опасно: если между чтением и записью произойдет паника, Rust попытается вызвать деструктор для пустой памяти, что приведет к `Undefined Behavior`.

<h4>Типичный пример: изменение состояния (`Enum`)</h4>

Это самый частый сценарий использования:

```rust
enum State {
    Waiting,
    Processing(String),
}

fn transition(state: &mut State) {
    // Мы хотим "вытащить" строку из Processing, не клонируя её
    if let State::Processing(data) = mem::replace(state, State::Waiting) {
        // Теперь у нас есть владение `data`
        println!("Обработка: {}", data);
    }
}
```

<h4>Итог</h4>

`mem::replace` — это способ легально и безопасно «украсть» данные по изменяемой ссылке, не нарушая гарантий Rust.

Примечание: Для частного случая с `Option<T>` чаще используют метод `option.take()`, который внутри себя вызывает `mem::replace(&mut opt, None)`.

<hr>

<a name="q-020202"><h3>Приведите несколько наглядных примеров использования `mem::replace` в Rust.</h3></a>

`std::mem::replace` остается незаменимым инструментом в ситуациях, когда вам нужно «забрать» владение данными из-под изменяемой ссылки (`&mut T`), одновременно оставив там валидное значение.

Вот три наиболее наглядных примера:

<h4>1. Извлечение данных из перечисления (`Enum`)</h4>

Это самый частый сценарий. Представьте, что у вас есть состояние, содержащее тяжелые данные (например, `String`), и вы хотите переключить состояние, забрав эти данные себе без лишнего клонирования.

```rust
use std::mem;

enum MyState {
    Active(String),
    Empty,
}

fn deactivate(state: &mut MyState) -> String {
    // Мы хотим забрать String, но по ссылке &mut это сделать нельзя.
    // mem::replace забирает старое состояние и записывает на его место Empty.
    if let MyState::Active(name) = mem::replace(state, MyState::Empty) {
        name // Теперь мы владеем этой строкой
    } else {
        String::new()
    }
}

fn main() {
    let mut s = MyState::Active("Важные данные".to_string());
    let data = deactivate(&mut s);
    println!("Извлечено: {}", data);
}
```

<h4>2. Реализация методов в структурах данных (например, `Linked List`)</h4>

При реализации связанных списков или деревьев вам часто нужно «отцепить» узел. `mem::replace` позволяет забрать текущий узел, заменив его на `None` или другой узел.

```rust
use std::mem;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct List {
    head: Option<Box<Node>>,
}

impl List {
    fn replace_head(&mut self, mut new_node: Box<Node>) -> Option<Box<Node>> {
        // Мы забираем текущую голову списка и ставим на её место новую.
        // Старая голова возвращается как результат функции.
        let old_head = mem::replace(&mut self.head, Some(new_node));
        old_head
    }
}
```

Примечание: Для Option чаще используют метод .take(), который внутри является оберткой над mem::replace.

<h3>3. Обновление значения в структуре через вычисление</h3>

Если вам нужно изменить поле структуры, основываясь на его старом значении, но функция преобразования требует владения (`owned`), `mem::replace` — единственный безопасный путь.

```rust
use std::mem;

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn transform_data(&mut self) {
        // Заменяем данные на пустой вектор временно, чтобы получить владение старым вектором
        let old_data = mem::replace(&mut self.data, Vec::new());
        
        // Теперь мы можем передать old_data в функцию, которая потребляет её
        self.data = self.process(old_data);
    }

    fn process(&self, d: Vec<u8>) -> Vec<u8> {
        // Какая-то логика трансформации
        d
    }
}
```

<h4>Почему эти примеры важны?</h4>

- Без клонирования: Во всех случаях мы избегаем `old_data.clone()`, что экономит память и время `CPU`.
- Безопасность при панике: Если бы мы использовали unsafe и чтение указателя, то при панике в середине функции Rust попытался бы очистить память, которая уже «украдена». `mem::replace` гарантирует, что в любой момент времени в переменной лежит что-то валидное (хотя бы `Empty` или `Vec::new()`).

`mem::replace` считается низкоуровневым кирпичиком, на котором строятся более высокоуровневые методы вроде `Option::take()` или `Vec::swap_remove()`. Документация [std::mem::replace](https://doc.rust-lang.org/std/mem/fn.replace.html).

<ht>

[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`mem::swap`]: https://doc.rust-lang.org/std/mem/fn.swap.html
[`mem::take`]: https://doc.rust-lang.org/std/mem/fn.take.html
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/a/30290070/1828012
[2]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
[3]: https://rust-unofficial.github.io/patterns/idioms/mem-replace.html
[4]: http://xion.io/post/code/rust-move-out-of-container.html
[5]: https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html
[6]: https://ferrous-systems.com/blog/rustls-borrow-checker-p1

[020201]: #q-020201
[020202]: #q-020202
