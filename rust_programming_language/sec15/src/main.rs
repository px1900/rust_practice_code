use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    // Example for Box<>
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3,
    Box::new(List::Nil))))));

    // Example for Deref
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // 5 is integer, y is &integer, so y need deref

    let x = 5;
    let y = Box::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5); // y is Box<integer>, so it need to deref before using

    // Example for MyBox Deref
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    // y is MyBox<integer>, so it need to deref before using
    // We need to implement deref for MyBox
    assert_eq!(*y, 5); 
    // The "*y" in fact was executed by rust as *(y.deref()), the y.deref() will
    // firstly been called to return the internal integer's reference, after that
    // the * will deref. In this way, we won't move MyBox's internal integer's 
    // ownership.

    // Example for automatically Deref
    let m = MyBox::new(String::from("rust"));
    hello(&m);
    // Firstly, &m will get MyBox's reference. Then automatically called MyBox's Deref to
    // convert &MyBox<String> to &String. Then automatically called String's Deref to convert 
    // &String to &str, which matches the function arguments.
    




}
