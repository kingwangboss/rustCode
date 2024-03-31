use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("number {} in main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("v: {:?}", v);
    });
    handle1.join().unwrap();

    
}
