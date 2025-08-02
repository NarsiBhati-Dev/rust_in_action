use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..500000000 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..50000000 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    let x = 1;
    {
        let v = vec![1, 2, 3, 4];
        thread::spawn(move || println!("{v:?}"));
    }

    println!("{x}");
}
