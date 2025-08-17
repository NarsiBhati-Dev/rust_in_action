#[derive(Debug)]

struct User {
    name: String,
    age: u32,
}

// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(name is {}, age is {})", self.name, self.age)
//     }
// }

fn main() {
    let user = User {
        name: String::from("Narsi"),
        age: 24,
    };

    println!("{:?}", user);
    println!("name {} and age {}", user.name, user.age);
}

#[derive(Debug)] // Debug is a custom drive proc macro

struct User {
    name: String,
    age: u32,
}

fn main() {
    println!("hello"); // Declerative macro
}

#[post("/user/")] // attribute like proc macro
fn create_user() {
    sqlx::query!("INSERT INTO USERS VALUES ()") // function like proc macro
}
