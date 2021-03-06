//#![feature(conservative_impl_trait)]
//fn add_n(n : i32) -> impl Fn(i32) -> i32 {
//    move |x| n + x
//}


fn add_n(n : i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |x| n + x)
}
 
fn main() {
    let adder = add_n(40);
    println!("The answer to life is {}.", adder(2));
}
