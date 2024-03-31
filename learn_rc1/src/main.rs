use std::rc::Rc;
use crate::List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //打印引用计数
    println!("count after creating a count = {}", Rc::strong_count(&a));
    // 共享所有权的时候需要用到智能指针rc
    let b = Cons(3, Rc::clone(&a));
    // let b = Cons(3, a.clone());
    println!("count after bind to b, a count = {}", Rc::strong_count(&a));
    {
        // let c = Cons(4, Rc::clone(&a));
        let c = Cons(4, a.clone());
        println!("count after bind to c, a count = {}", Rc::strong_count(&a));
    }
    println!("count an end, a count = {}", Rc::strong_count(&a));
    // println!("a = {:#?}", a);
    // println!("b = {:#?}", b);
    // println!("c = {:#?}", c);
}
