enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
    Square(f64),
}

// fn main() {
//     let circle = Shape::Circle(5.4);
//     let square = Shape::Square(4.3);
//     let rectangle = Shape::Rectangle(3.0, 6.9);

//     println!("area of circle : {}", calculate_area(circle));
//     println!("area of square : {}", calculate_area(square));
//     println!("area of rectangle : {}", calculate_area(rectangle));
// }

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(w, h) => w * h,
    }
}
