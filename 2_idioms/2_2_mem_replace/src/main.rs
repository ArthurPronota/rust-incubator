use std::collections::HashSet ;
use std::mem ;

struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    /*
    fn _apply_exclusions(&mut self) {
        self    // первое изменяемле заимствование &mut self
            .exclusions
            .drain(..)
            .for_each(|name| {
                self.remove_name(&name);    // второе изменяемле заимствование &mut self
            })
    }
     */

    fn __apply_exclusions(&mut self) {
        self
            .exclusions
            .drain(..)
            .for_each(|name| {
                self
                    .names
                    .remove(&name) 
                    ;
            });
    }

    fn apply_exclusions(&mut self) {
        let mut exclusions = 
                // это установка self.exclusions = vec![] ;
                mem::take( // Замещает dest со значением по умолчанию T, возвращая предыдущее dest значение.
                    &mut self.exclusions
                )
                ;
        exclusions
            .drain(..) // Удаляет из вектора subslice, указанную заданным диапазоном, и возвращает двусторонний итератор по удаленному subslice.
            .for_each(|name| {
                self.remove_name(&name); // (первое изменяемое заимствование &mut self) удаление из HashSet значения name
            })
            ;
    }
    
    // удаление из HashSet значения name
    fn remove_name(&mut self, name: &str) {
        self
            .names
            .remove(name)
            ;

        // mem::replace<T>(dest: &mut T, src: T) -> T // Перемещает src в упомянутую dest, возвращая предыдущее dest значение.

        // mem::swap<T>(x: &mut T, y: &mut T) // Меняет местами значения в двух изменяемых местах, не деинициализируя ни одно из них.

        // Option::take(&mut self) -> Option<T> // Удаляет значение из Option возвращая его, оставляя вместо него значение «None».

    }
}

/*
// Это НЕ СКОМПИЛИРУЕТСЯ
fn swap_out(r: &mut String) -> String {
    let val = *r; // Ошибка: попытка перемещения из-под ссылки.
                  // После этого r указывал бы на невалидную память.
    *r = String::from("new"); 
    val
}
 */


fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Clone> Trinity<T> {
    fn rotate(&mut self) {
        let a = self.a.clone();
        let b = self.b.clone();
        let c = self.c.clone();
        self.a = b;
        self.b = c;
        self.c = a;
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: Clone + PartialEq> Solver<T> {
    fn resolve(&mut self) {
        let mut unsolved = Vec::with_capacity(self.unsolved.len());
        'l: for t in self.unsolved.iter_mut() {
            for _ in 0..3 {
                if *t == self.expected {
                    continue 'l;
                }
                t.rotate();
            }
            unsolved.push(t.clone())
        }
        self.unsolved = unsolved;
    }
}
