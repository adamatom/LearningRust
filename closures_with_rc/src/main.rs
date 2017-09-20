use std::rc::Rc;
use std::cell::RefCell;

fn get_func() -> Box<Fn(i32) -> i32> {
    let n = Rc::new(RefCell::new(0));
    let m = n.clone(); // have to make one to move
    let func = Box::new(move |x| *m.borrow() + x);
    *n.borrow_mut() = 2;

    func
}
 
fn main() {
    let func = get_func();
    println!("The answer to life is {}.", func(40));
}
