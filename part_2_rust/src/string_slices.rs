// string and slices
// fn main() {
//     let mut name = String::from("narsi");
//     name.push_str(" bhati");
//     println!("{}", name);
//     // name.replace_range(5..name.len(), "");
//     println!("name is : {}", name);
//     first_word(&name);
//     println!("{}", name);
// }

fn first_word(str: &String) -> &str {
    let mut index = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        index += 1;
    }
    return &str[0..index];
}
