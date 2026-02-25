use rand::{self, prelude::*};

fn main() {
    let mut rng = rand::thread_rng();

    // 1. Случайное число (целое или с плавающей точкой)
    let n: u32 = rng.gen_range(0..100);
    println!("n: {}", n) ;

    let x: f64 = rng.r#gen(); // от 0.0 до 1.0
    println!("x: {}", x) ;

    // 2. Случайный выбор из списка
    let choices = ["Rust", "Go", "C++"];
    if let Some(&lang) = choices.choose(&mut rng) {
        println!("Выбран язык: {}", lang);
    }

    // 3. Перемешивание вектора
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("{:?}", nums) ;
}