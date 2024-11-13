mod basket;
mod stack;

use basket::Basket;
use stack::Stack;

fn main() {
    do_basket();
    do_stack();
}

fn do_basket() {
    println!();
    let mut b1 = Basket::new(String::from("Casey"));
    println!("{:?}", b1.get());

    b1.put(String::from("Casey Bo Trent"));
    println!("{:?}", b1.get());

    println!("{:?}", b1.is_empty());
    println!();

    println!();
    let mut b2 = Basket::new(17);
    println!("{:?}", b2.get());

    println!();
    let mut b3 = Basket::new(false);
    println!("{:?}", b3.get());
}

fn do_stack() {
    let s1 = Stack::new(vec![String::from("Casey Bo Trent")]);
}
