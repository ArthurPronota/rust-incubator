fn main() {
    println!("Implement me!");

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            // This comparison works because we constrained T to require the 'PartialOrd' trait
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // This works for integers:
    let number_list = vec![34, 50, 25, 100, 65];
    let f = &number_list ;
    let result = largest(&number_list); // T becomes i32

    // And it works for characters:
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list); // T becomes char

}
