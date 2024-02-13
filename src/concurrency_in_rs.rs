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
fn messages_passing_trn() {
    let (tx, rx) = mpsc::channel();
}
