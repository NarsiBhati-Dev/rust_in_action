// fn main() {
//     // stack memory
//     let a: i32 = 1;
//     let b: i32 = 1;
//     let sum: i32 = find_sum(a, b);
//     println!("sum is {}", sum);

//     // heap memory
//     let mut name = String::from("Narsi");
//     println!("{}", name);

//     name.push_str(" Bhati");
//     println!("{}", name);
// }

fn find_sum(a: i32, b: i32) -> i32 {
    a + b
}
