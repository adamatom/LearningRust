enum IpKind {
    V4(u8, u8, u8, u8),
    V6 { val: String },
}
impl IpKind {
    fn route(&self) {
        match *self {
            IpKind::V4(ref a, ref b, ref c, ref d) => println!("{}.{}.{}.{}", a, b, c, d),
            IpKind::V6 { ref val } => println!("{}", val),
        }
    }
    fn sillyv4(&self) -> Option<u8> {
        if let IpKind::V4(a, _, _, _) = *self {
            Some(a)
        } else {
            None
        }
    }
}

enum Coin {
    Penny = 1,
    Nickel = 5,
    Dime = 10,
    Quarter = 25,
}

impl Coin {
    // pretty silly to have this at all, just use "as isize"
    fn value(&self) -> isize {
        match *self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let four = IpKind::V4(1, 2, 3, 4);
    let six = IpKind::V6 { val: String::from("::1") };
    four.route();
    six.route();

    let some_string = Some("asdf");
    let not_some_string: Option<&str> = None;
    assert_eq!(some_string.is_some(), true);
    assert_eq!(not_some_string.is_some(), false);
    assert_eq!(four.sillyv4().is_some(), true);

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    let penny_val = Coin::Penny as isize;
    println!(
        "{} {} {} {} {}",
        penny.value(),
        penny_val,
        nickel.value(),
        dime.value(),
        quarter.value()
    );
}
