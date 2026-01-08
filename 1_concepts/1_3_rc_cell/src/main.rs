use std::rc::Rc ;
use std::sync::Mutex ;

pub struct GlobalStack<T> {
    data:   Rc<Mutex<Vec<T>>>
}

impl<T> GlobalStack<T> {
    fn new() ->Self {
        Self { data: Rc::new(Mutex::new(Vec::new())) }
    }

    fn push(&mut self, val: T) {
        let mut vec = self
            .data
            .lock()
            .unwrap()
            ;
        vec.push(val);
    }

    fn pop(&mut self) -> Option<T>{
        let mut vec = self
                    .data
                    .lock()
                    .unwrap() 
                    ;
        vec.pop()
    }
}

impl<T> Clone for GlobalStack<T> {
   fn clone(&self) -> Self {
       Self { data: Rc::clone(&self.data) }
   } 
}

fn main() {
    let mut my_stack = GlobalStack::<i32>::new() ;

    for n in [1,2,3] {
        println!("to stack: {}", n) ;
        my_stack.push(n);    
    }

    println!("From stack: {:?}", my_stack.pop()) ;
    println!("From stack: {:?}", my_stack.pop()) ;
    println!("From stack: {:?}", my_stack.pop()) ;
}
