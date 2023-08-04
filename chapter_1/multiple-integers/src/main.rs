use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // Integer on the stack
    let a = 10;

    //Integer on the heap, also known as a boxed integer
    let b = Box::new(20);

    //Boxed integer wrapped within a reference counter
    let c = Rc::new(Box::new(30));

    //Integer wrapped in an atomic reference counter and protected by a mutual exclusion lock
    let d = Arc::new(Mutex::new(40));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
