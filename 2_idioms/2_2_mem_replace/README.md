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
        let mut exclusions = mem::take(&mut self.exclusions);
        exclusions.drain(..).for_each(|name| {
            self.remove_name(&name);
        });
    }

    fn remove_name(&mut self, name: &str) {
        self.names.remove(name);
    }
}
```

It's worth mentioning, that this problem became much less common after [disjoint capture in closures had been introduced in 2021 Rust edition][5]. For illustration, the `self.name` mutation is intentionally separated into its own method, so we can lock the whole `&mut self`. If we simplify the code straightforwardly, it just compiles fine, due to mutable borrows are disjoint: 
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




## Task

Improve and optimize the code contained in [this step's crate](src/main.rs) to cut off redudant performance costs.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is the reason of [`mem::replace`] existing in [Rust]? What does it give to us? Why cannot we solve the same problems without it?
- Provide some meaningful examples of using [`mem::replace`] in [Rust].




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
