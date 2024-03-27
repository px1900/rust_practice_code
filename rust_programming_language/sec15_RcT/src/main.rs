// The Rc<T> are designed to let multiple parts share one variable.
// And it doesn't know which part will be the last one to use this variable, which
// is reasonable, because if rust know which part will lastly use this variable, it
// can let that part owns this variable, and other parts use its references.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // b -> 3 ->
    //          -> a -> 5 -> 10 -> Nil
    // c -> 4 ->
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        // Rc::clone() will only increase Rc's usage counter, it won't do deep copy
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after get out of scope = {}", Rc::strong_count(&a));
}
