mod basket;
mod stack;
mod containers;
use basket::Basket;
use stack::Stack;

use crate::containers::Container;

fn add_string<T: Container<String>>(c: &mut T, s: String){
    c.put(s);
}
fn main() {

    let mut b1 = Basket::new(String::from("hi there"));//this is stuck with string values

    let b2 = Basket::new(10);//this is stuck with integer values

    let b3 = Basket::new(true); //now this is stuck with  bool values

    let mut s1 = Stack::new(vec![String::from("hi")]);

    let s2 = Stack::new(vec![1,2,3]);

    add_string(&mut b1, String::from("hi hello"));
    add_string(&mut s1, String::from("hello"));
}
