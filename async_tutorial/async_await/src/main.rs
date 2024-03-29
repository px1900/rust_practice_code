use futures::executor::block_on;
use std::time::Duration;
use std::thread;

async fn count_to(count: i32) {
    for i in 1..=count {
        let tid = thread::current().id();
        thread::sleep(Duration::from_secs(1));
        println!("thread_id = {:?}, count = {}", tid, i);
        
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    block_on(async_main(10));
}
