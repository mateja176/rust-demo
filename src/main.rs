use std::rc::Rc;
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

    #[derive(Debug)]
    enum RcList {
        Cons(i32, Rc<RcList>),
        Nil,
    }
    let rc_list = RcList::Cons(
        1,
        Rc::new(RcList::Cons(
            2,
            Rc::new(RcList::Cons(3, Rc::new(RcList::Nil))),
        )),
    );
    println!("RcList: {:?}", rc_list);

    let a = Rc::new(RcList::Cons(1, Rc::new(RcList::Nil)));
    println!("RcList count a: {}", Rc::strong_count(&a));
    let b = RcList::Cons(2, Rc::clone(&a));
    println!("RcList count b: {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(3, Rc::clone(&a));
        println!("RcList count c: {}", Rc::strong_count(&a));
    }
    println!("RcList count: {}", Rc::strong_count(&a));

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

    // 'a refers to the smallest lifetime
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() >= y.len() {
            x
        } else {
            y
        }
    }

    let x = "x";
    let z: &str;
    {
        let y = "yz";
        z = longest(x, y);
    }
    println!("Lifetime longest: {}", z);

    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn print(&self) -> String {
            format!("x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point { x: 1.0, y: 2.0 };
    println!("Trait Point::format: {}", point.print());

    fn collatz(i: i128) -> i128 {
        if i % 2 == 0 {
            i / 2
        } else {
            (i * 3) + 1
        }
    }
    fn collatz_n<F>(collatz_fn: F, n: i128) -> i128
    where
        F: Fn(i128) -> i128,
    {
        match n {
            1 => n,
            _ => {
                println!("Callback collatz: {}", n);
                let m = collatz_fn(n);
                collatz_n(collatz_fn, m)
            }
        }
    }
    collatz_n(collatz, 3);

    fn pow_n(n: i128) -> Box<dyn Fn(u32) -> i128> {
        if n == 1 {
            Box::new(|_: u32| 1)
        } else {
            Box::new(move |m: u32| -> i128 { i128::pow(n, m) })
        }
    }
    let pow_1 = pow_n(1);
    let pow_2 = pow_n(2);
    println!("Curry 1??: {}", pow_1(2));
    println!("Curry 2??: {}", pow_2(2));

    let int = 1;
    let int_ref = &int;
    let int_deref = *int_ref;
    println!("Deref integer {}", int_deref);
}
