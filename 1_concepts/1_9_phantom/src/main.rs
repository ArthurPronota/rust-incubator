use std::marker::PhantomData ;

// Расширение возможностей индексируемых списков, обеспечивающее методы 
// случайной выборки.
use rand::seq::IndexedRandom;

struct Fact<T> {
    facts:      Vec<&'static str>,
    _marker:    PhantomData<T>,
}

impl<T> Fact<T> {
    // загрузка фактов
    fn load_facts(facts: Vec<&'static str>) ->Self {
        Self { facts, _marker: PhantomData }
    }

    // получение рандомного факта
    fn fact(&self) ->&str { // автоматическое приведение &&str -> &str
        self
            .facts
            .choose(&mut rand::rng())
            .unwrap_or(&"no data on the facts")
    }

}

// реализация Fact<Vec<T>> (ковариантность)
impl<T> Fact<Vec<T>> {
    fn new() ->Self {
        Self::load_facts(vec![
            "Vec is heap-allocated.",
            "Vec may re-allocate on growing.",
        ])
    }
}

fn main() {
    let f = Fact::<Vec<i32>>::new() ;

    for _ in 0..5 {
        println!("Fact about Vec: {}", f.fact()) ;
    }
}

/*
// пример укорачивания времени жизни для y
fn implicit_lifetime<'a, 'b>(x: &'a i32, y: &'b i32) ->&'a i32 
    where 'b: 'a,   // 'b живёт не меньше чем 'a
{
    y
}
 */