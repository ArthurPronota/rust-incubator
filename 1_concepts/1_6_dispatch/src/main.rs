use std::borrow::Cow ;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

fn main() {
    println!("Implement me!");
}
