// lifetimes

fn main() {
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(&str1, &str2);
    }
    println!("{}", longest_str);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}
