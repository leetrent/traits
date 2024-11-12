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
}
