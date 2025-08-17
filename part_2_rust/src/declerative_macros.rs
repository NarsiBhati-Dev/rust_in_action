macro_rules! eval {
    ($x: expr) => {
        $x
    };
}

macro_rules! vector {
    ($ ($x: expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}
fn main() {
    let ans = eval!(2 + 3 * 6);
    let ans_vec = vector!(2, 3, 4, 5, 6);
    println!("{}", ans);
    println!("{ans_vec :?}")
}
