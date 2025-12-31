fn main() {

    {
        use std::cell::Cell;    // For types that implement Copy

        let c = Cell::new(1);

        c.set(2);
        println!("{:?}", c.get()) ;
    }

}