mod print;

use print::greet;

fn main() {
    greet::en("Rust");

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("BoxList: {:?}", list);

    let mut vector = vec![1, 2, 3];
    println!("Vec last: {:?}", vector.last());
    vector.push(4);
    println!("Vec last after push: {:?}", vector.last());
    vector.remove(vector.len() - 1);
    println!("Vec last after remove: {:?}", vector.last());

    let product = vector
        .iter()
        .fold(0, |acc, element| acc + i32::pow(2, *element));
    println!("Vec sum of powers of 2: {}", product);
}
