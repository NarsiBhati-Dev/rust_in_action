fn create_string() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
}

// fn main() {
//     create_string();
// }

fn do_something(s2: &String) {
    print!("{}", s2)
}
