// traits

trait Summary {
    fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("The Name is {} and the age is {}", self.name, self.age);
    }
}

fn notify(u: impl Summary) {
    println!("{}", u.summarise())
}

fn main() {
    let user = User {
        name: String::from("nasri"),
        age: 24,
    };

    println!("{}", user.summarise());
    notify(user);
}
