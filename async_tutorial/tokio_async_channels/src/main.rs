use tokio::sync::mpsc::{self, Receiver};
use tokio::time;

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;

    while let Some(_) = input.recv().await {
        count += 1;
        let _ = time::sleep(time::Duration::from_millis(50));
        println!("Received {count} pings");
    }

    println!("ping_handler complete");
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
        sender.send(()).await.expect("failed to send ping");
        println!("Sent {} pings so far", i+1);
    }

    drop(sender);
    ping_handler_task.await.expect("Something went wrong in ping handler task.");
}

