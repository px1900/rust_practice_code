use std::thread;

fn main() {
    // Example 1

    let list = vec![1,2,3,4];
    println!("Before defining closure: {:?}", list);

    // When defining it, the closure will borrow list's immutable reference.
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    // Before calling only_borrows(), we can't modify list. Because its immutable 
    // reference is held by only_borrows()
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Example 2

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // We can't borrow list here, since its mutable reference is held by closure.
    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    // Here the "move" command moves $list's ownership to the closure. After here, we
    // can't use list variable.
    // The "move" is necessary here, since the thread may outlive the current main thread.
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
    //println!("After calling closure: {:?}", list);

    // Example 1 for iterator

    let v1: Vec<i32> = vec![1, 2, 3];

    let iter = v1.iter();

    let mut total : i32 = 0;
    for ele in iter {
        total += ele;
    }
    println!("total = {total}");

    // Example 2 for iterator

    let v1: Vec<i32> = vec![1, 2, 3];

    let iter = v1.iter();

    let total: i32 = iter.sum();
    println!("total = {total}");

    // Example 3 for iterator

    let v1: Vec<i32> = vec![1, 2, 3];

    let iter = v1.iter();
    
    let v2: Vec<i32> = iter.map(|t| t+1).collect();
    println!("v2 = {:?}", v2);


}
