struct User<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("narsi bhati");
    let user = User { name: &name };

    println!("username : {}", user.name);
}
