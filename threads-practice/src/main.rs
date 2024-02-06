use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
        }
    });

    handle.join().unwrap();



}
