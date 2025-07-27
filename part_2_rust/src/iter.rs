// fn main() {
//     // let mut nums = vec![1, 2, 3, 4, 5, 6];

//     // let mut nums_iter = nums.iter();

//     // while let Some(value) = nums_iter.next() {
//     //     println!("{}", value)
//     // }

//     // for num in nums_iter {
//     //     println!("got {}", num);
//     // }

//     // let nums_iter_mut = nums.iter_mut();
//     // for value in nums_iter_mut {
//     //     *value = *value + 1
//     // }

//     // println!("{:?}", nums);

//     /*
//     consumer adaptor
//     */
//     // let v1 = vec![1, 2, 3];
//     // let v1_iter = v1.iter();

//     // let sum: i32 = v1_iter.sum();
//     // println!("{}", sum);

//     // println!("{:?}", v1);

//     let v1 = vec![1, 2, 3, 4, 5];
//     let v1_iter = v1.iter();

//     let v1_iter2 = v1_iter.map(|x| x + 2);
//     println!("{:?}", v1_iter2);

//     let iter2 = v1_iter2.filter(|x| *x % 2 == 0);
//     println!("{:?}", iter2);
// }
