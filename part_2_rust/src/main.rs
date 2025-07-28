// lifetimes

fn main() {
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(str1, str2);
    }
    println!("{}", longest_str);
}

fn longest(a: String, b: String) -> String {
    if a.len() > b.len() { a } else { b }
}
