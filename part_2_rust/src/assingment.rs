fn filter_and_map(nums: Vec<i32>) -> Vec<i32> {
    let new_iter = nums.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);

    let new_vec: Vec<i32> = new_iter.collect();
    return new_vec;
}

// fn main() {
//     let nums = vec![1, 2, 3, 4, 5, 6];

//     let ans = filter_and_map(nums);
//     println!("{:?}", ans);
// }
