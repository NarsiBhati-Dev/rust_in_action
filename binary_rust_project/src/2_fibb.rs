fn fibb(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }

    if num == 1 {
        return 1;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

// use
// fn main() {
//     let range: u32 = 5;

//     for i in 0..range + 1 {
//         print!("{} ", i);
//         println!("{}", fibb(i))
//     }
// }
