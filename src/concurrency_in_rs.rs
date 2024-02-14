use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn all_concurrency_exam() {
    // Creating a New Thread with spawn
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number from the spawn thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Waiting for All Threads to Finish Using join Handles.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number from the spawn thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Waiting for All Threads to Finish Using join Handles
    handle.join().unwrap();

    for i in 1..14 {
        println!("Number from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // // Waiting for All Threads to Finish Using join Handles
    // handle.join().unwrap();

    //  Using move Closures with Threads
    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle2.join().unwrap();
}

// Using Message Passing to Transfer Data Between Threads
pub fn messages_passing_trn() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("Sending message");
        let val = String::from("Hi Coded");
        tx.send(val).unwrap();
    });

    let receive = rx.recv().unwrap();
    println!("Got: {}", receive);
}

// Sending Multiple Values and Seeing the Receiver Waiting
pub fn multiple_messages_tr() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("Sending message");
        let vals = vec![
            String::from("Hi Coded 1"),
            String::from("Hi Coded 2"),
            String::from("Hi Coded 3"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for receive in rx {
        println!("Got: {}", receive);
    }
}
