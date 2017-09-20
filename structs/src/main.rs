struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}


#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn new(width: i32, height: i32) -> Rect {
        Rect { width, height }
    }
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    fn print(&self) {
        println!(
            "Hello, {} ({}), signed in: {} and active: {}",
            self.username,
            self.email,
            self.sign_in_count,
            self.active
        );
    }
}

struct Point(i32, i32, i32);
struct FPoint(f32, f32, f32);

fn test_tuple_struct() {
    let p1 = Point(1, 2, 3);
    let fp2 = FPoint(1.0, 2.0, 3.0);
    println!("{}, {}", p1.0, fp2.0);
}

fn main() {
    let mut new_user = User::new(String::from("Adam"), String::from("adamlabbe@gmail.com"));
    new_user.sign_in_count = 2;
    new_user.print();
    test_tuple_struct();

    let rect = Rect::new(10, 2);
    println!("{:?} area: {}", rect, rect.area());
    let big_rect = Rect::new(20, 3);
    println!("can hold?: {}", big_rect.can_hold(&rect))
}
