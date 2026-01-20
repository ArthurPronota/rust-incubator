Шаг 2.3: Ограничение поведения, а не данных.
==================================

__Estimated time__: 1 day

Часто, когда мы хотим абстрагироваться от какого-либо типа или поведения в [Rust], мы начинаем с этого:
```rust
struct UserService {
    repo: UserRepo,
}
```
к этому:
```rust
struct UserService<R: UserRepo> {
    repo: R,
}
```

Здесь мы указываем привязку `R: UserRepo`, поскольку хотим ограничить типы в поле `repo` для реализации поведения `UserRepo`.

Однако такое ограничение непосредственно на тип приводит к так называемому «загрязнению границ трейтов»: нам приходится повторять это ограничение в каждой отдельной реализации, даже в тех, которые никак не связаны с поведением `UserRepo`.
```rust
struct UserService<R: UserRepo> {
    repo: R,
}

impl<R> Display for UserService<R>
where
    R: Display + UserRepo, // <- We are not interested in UserRepo here,
{                          //    all we need is just Display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}
```

В сложной кодовой базе подобное загрязнение, возникающее из-за множества различных типов данных, в какой-то момент может превратиться в настоящий кошмар.

Решение этой проблемы заключается в понимании того, что __трейт представляет собой определенное поведение__, и, в действительности, __нам это поведение нужно только тогда, когда мы его объявляем__. Объявление типа не содержит ничего о поведении, оно полностью посвящено данным. __Поведение проявляется в функциях и методах__. Поэтому давайте просто будем ожидать определенного поведения, когда оно нам действительно понадобится:
```rust
struct UserService<R> {
    repo: R,
}

// Ожидайте отображения (Expect Display) при выражении поведения отображения.
impl<R: Display> Display for UserService<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}

// Ожидайте UserRepo, когда мы будем описывать фактическое поведение UserService,
// которое работает с Users.
impl<R: UserRepo> UserService<R> {
    fn activate(&self, user: User) {
        // Changing User state in UserRepo...
    }
}
```

Размещение ограничений трейтов на блоках `impl`, методах и функциях, а не на типах, _уменьшает загрязнение трейтами_, _снижает [связность][1] частей кода_ и _делает обобщенный код более чистым, понятным и эргономичным_.



## Снять ненужные ограничения

В качестве более общего правила: __следует стараться максимально расширять ограничения трейтов__ (особенно в библиотечном коде), поскольку это увеличивает разнообразие вариантов использования типа.


Иногда для этого необходимо отказаться от использования `#[derive]`, поскольку это может привести к ненужной привязке к трейту. Например:
```rust
#[derive(Clone)]
struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}

struct My;

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // compile error as `My` doesn't impl `Clone`
```
Это происходит потому, что `#[derive(Clone)]` применяет ограничения `K: Clone` и `V: Clone` в производном коде, несмотря на то, что они совершенно не нужны, поскольку [`Arc` всегда реализует `Clone`][2] (также рассмотрите ограничение `T: ?Sized` в [linked implementation][2], которое снимает неявное ограничение `T: Sized`, поэтому позволяет использовать `Arc::clone()` даже для [unsized types][3]).

Предоставляя реализованный вручную код, мы можем без проблем клонировать значения типа `Loader<My, My>`:
```rust
struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}

// Ручная реализация используется для того, чтобы избежать применения ненужных ограничений Clone.
impl<K, V> Clone for Loader<K, V> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // it compiles now!
```

## Task

Переработайте код, содержащийся в [crate этого шага](src/main.rs), чтобы максимально уменьшить загрязнение границ трейтов.

## Questions

После выполнения всех вышеперечисленных действий вы должны уметь ответить (и понять, почему) на следующие вопросы:

- Which problems do trait bounds impose in [Rust] when are placed on a type definition?
- [`Какие проблемы создают ограничения трейтов в [Rust] при их размещении на определении типа?`]()

- Why placing trait bounds on `impl` blocks is better?
- When cannot we do that and should use trait bounds on a type definition? When is it preferred?
- What are the problems with `std` derive macros regarding type parameters? How could they be solved?

<hr>

<h3>Какие проблемы создают ограничения трейтов в [Rust] при их размещении на определении типа?</h3>


<hr>


[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Coupling_(computer_programming)
[2]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#impl-Clone
[3]: ../../1_concepts/1_7_sized
