use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("1st"),
            String::from("2nd"),
            String::from("3rd"),
            String::from("4th"),
        ];
        // in this send, the ownership of $val has been sent 
        // out of this scope
        let mut count : i32 = 0;
        // it seems that even we only sent the first two value
        // from values to out of scope, the entire ownership of 
        // $vals has been sent to the main thread.
        for val in vals {
            if count >= 2 {
                break;
            } else {
                count += 1;
            }
            
            tx.send(val).unwrap();
        }
        //tx.send(vals[0]).unwrap();
        //tx.send(vals[1]).unwrap();

        //println!("{:?}", vals[2]);
    });

    //let received = rx.recv().unwrap();
    //println!("Got {} from thread", received);
    for received in rx {
        println!("Got {} from thread", received);
    }



    // Example, use clone to enable multiple sender send message
    // to one receiver
    // Only the sender can be cloned, the sender can't be cloned
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1st"),
            String::from("2nd"),
            String::from("3rd"),
            String::from("4th"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("c-1st"),
            String::from("c-2nd"),
            String::from("c-3rd"),
            String::from("c-4th"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {} from thread", received);
    }

}
