use std::thread;
use std::time::Duration;

fn main() {
    let vec = vec![1, 2, 3, 4];
    let thread_handler = thread::spawn(move || {
        println!("vec = {:?}", vec);
        for i in 1..10 {
            println!("in thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("in main: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    thread_handler.join().unwrap();

}
