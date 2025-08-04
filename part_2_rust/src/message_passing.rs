// use std::{sync::mpsc, thread::spawn};

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     spawn(move || tx.send(String::from("Hello,World")));

//     let value = rx.recv();
//     match value {
//         Ok(value) => println!("{value}"),
//         Err(err) => println!("error : {err}"),
//     }
// }

use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..100000000 {
                ans = ans + (i * 100000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }

    drop(tx);

    let mut ans: u64 = 0;
    for val in rx {
        ans = ans + val;
        println!("found value {val}");
    }
    println!("Ans is {ans}");
}
