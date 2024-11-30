use std::thread;

fn thread_func(id: i32) {
    println!("hello, {}", id);
}

fn main() {
    // Spawn thread with ID 1
    let handle1 = thread::spawn(|| {
        let id = 1;
        thread_func(id);
    });
    // Spawn thread with ID 12
    let handle2 = thread::spawn(|| {
        let id = 2;
        thread_func(id);
    });
    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();
}

