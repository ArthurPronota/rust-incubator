// пример укорачивания времени жизни для y
fn implicit_lifetime<'a, 'b>(x: &'a i32, y: &'b i32) ->&'a i32 
    where 'b: 'a,   // 'b живёт не меньше чем 'a
{
    y
}

fn main() {
    println!("Implement me!");
}
