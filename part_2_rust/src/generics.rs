// Generics

// fn main() {
//     println!("largest number : {}", largest_number(1, 2));
//     println!("largest  char : {}", largest_char('a', 'b'));

//     println!("largest char : {}", largest('a', 'b'));
//     println!("largest number : {}", largest(1, 2));
// }

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn largest_number(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn largest_char(a: char, b: char) -> char {
    if a > b { a } else { b }
}
