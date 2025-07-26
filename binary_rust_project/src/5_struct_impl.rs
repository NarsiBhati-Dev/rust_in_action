struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn debug() -> u32 {
        return 1;
    }
}

// fn main() {
//     let rect1 = Rect {
//         width: 24,
//         height: 23,
//     };

//     println!("area of rectangle is: {}", rect1.area());
//     println!("area of perimeter is: {}", rect1.perimeter());
//     println!("static function : {}", Rect::debug())
// }
