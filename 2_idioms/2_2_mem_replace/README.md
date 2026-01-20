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
        let mut exclusions = mem::take( // Замещает dest с значением по умолчанию T, возвращая предыдущий dest значение.
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

- What is the reason of [`mem::replace`] existing in [Rust]? What does it give to us? Why cannot we solve the same problems without it?
- [`В чём причина существования `mem::replace` в [Rust]? Что он нам даёт? Почему мы не можем решить те же проблемы без него?`]()

- Provide some meaningful examples of using [`mem::replace`] in [Rust].

<hr>

<h3>В чём причина существования `mem::replace` в [Rust]? Что он нам даёт? Почему мы не можем решить те же проблемы без него?</h3>

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



<hr>


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
