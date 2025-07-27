// fn main() {
//     // let mut vec = Vec::new();
//     // vec.push(1);
//     // vec.push(2);
//     // vec.push(3);
//     // vec.push(4);

//     let mut vec = vec![1, 2, 3, 4, 5, 6];

//     println!("{:?}", vec);
//     println!("{:?}", even_filter(&vec));
//     even_values(&mut vec);
//     println!("{:?}", vec);
// }

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut even = Vec::new();

    for value in vec {
        if value % 2 == 0 {
            even.push(*value);
        }
    }

    return even;
}

fn even_values(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 == 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
