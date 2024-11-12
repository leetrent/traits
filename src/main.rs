mod basket;
use basket::Basket;

fn main() {
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
