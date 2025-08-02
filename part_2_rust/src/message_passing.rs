use std::{sync::mpsc, thread::spawn};

fn main() {
    let (tx, rx) = mpsc::channel();
    spawn(move || tx.send(String::from("Hello,World")));

    let value = rx.recv();
    match value {
        Ok(value) => println!("{value}"),
        Err(err) => println!("error : {err}"),
    }
}
