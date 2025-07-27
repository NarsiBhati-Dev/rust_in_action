fn find_first_a(s: String) -> Option<i32> {
    for (index, charactor) in s.chars().enumerate() {
        if charactor == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

// fn main() {
//     let my_string = String::from("Narsi");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is founded at index : {}", index + 1),
//         None => println!("The letter 'a' is not founded in the string"),
//     }

//     let my_string = String::from("sdsgdsr");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is founded at index : {}", index + 1),
//         None => println!("The letter 'a' is not founded in the string"),
//     }
// }
