use std::collections::HashMap;

// fn main() {
//     let mut users = HashMap::new();

//     users.insert(String::from("narsi"), 24);
//     users.insert(String::from("harkirat"), 32);
//     users.insert(String::from("raman"), 30);

//     match users.get("narsi") {
//         Some(age) => println!("age is : {}", age),
//         None => println!("User not found in db"),
//     };
// }

// fn main() {
//     let pairs = vec![(String::from("narsi"), 24), (String::from("harkirat"), 32)];

//     let grouped_pairs = group_values_by_keys(pairs);

//     println!("{:?}", grouped_pairs)
// }

fn group_values_by_keys(pairs: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    for (key, value) in pairs {
        hm.insert(key, value);
    }
    return hm;
}
