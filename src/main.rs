mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

fn main() {
    do_basket();
    do_stack();
}

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn do_basket() {
    println!();
    let mut b1 = Basket::new(String::from("Casey"));
    // let b2 = Basket::new(17);
    // let b3 = Basket::new(false);

    add_string(&mut b1, String::from(" Bo Trent"));
}

fn do_stack() {
    let mut s1 = Stack::new(vec![String::from("Casey")]);
    // let s2 = Stack::new(vec![1,2,3]);
    add_string(&mut s1, String::from(" Bo Trent"));
}
