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

struct Rect {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }
}

trait Shape {
    fn area(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

fn main() {
    let r = Rect {
        width: 32,
        height: 23,
    };

    let s = Square { side: 20 };

    println!("{}", r.area());
    println!("{}", s.area());
}
