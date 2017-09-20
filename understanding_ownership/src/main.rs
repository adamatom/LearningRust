
fn main() {
    // Assignment to a literal returned from a function
    let s_lit = get_literal();
    println!("string literal from function: {}", s_lit);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("mutated string: {}", s);

    let s2 = s.clone();
    println!("cloned string: {}", s2);

    // try out something that moves and invalidates our local
    takes_ownership(s2); // s2 moved, no longer valid

    // stack objects are pass by copy, no reference needed
    let i: i32 = 5;
    makes_copy(i);
    println!("still alive: {}", i);

    // try out a function that allocates but then moves
    let s3 = gives_ownership();
    println!("allocated elsewhere, but ours now: {}", s3);

    // create a mutable copy and then give to a function that borrows and mutates
    let mut s4 = s3.clone();
    let s4_len = calculate_length(&mut s4); // cant put this inline on next line without borrow error
    println!("mutated borrow: {}:{}", s4, s4_len);

    // Create a immutable string, slice it to str, copy to a String, borrow and mutate.
    let words: String = String::from("Hello and goodbye");
    let mut copy_first: String = first_word(&words).to_string();
    let copy_len: usize = calculate_length(&mut copy_first);
    println!("around the ownership barn: {}:{}", copy_first, copy_len);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("MUTATED");
    s.len()
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// I was curious, can a function return a string literal?
// Yes, this really returns an address in the binary, with a len (str is pointer + len?)
fn get_literal() -> &'static str {
    "hello"
}
