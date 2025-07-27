fn main() {
    let mut s1 = String::from("nasri");
    do_something(&mut s1);
    println!("{}", s1)
}

fn do_something(s2: &mut String) {
    s2.push_str(" bhati");
    println!("{}", s2)
}
