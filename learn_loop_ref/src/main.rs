use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// 弱引用Weak<T>
// 1.弱引用通过Rc::downgrade传递Rc实例的引用，调用Rc::downgrade会得到Weak<T>类型的智能指针，
// 同时将weak_count加1.
// 2.区别在于weak_count无需计数为0就能使Rc实例被清理。只要strong_count为0就可以了
// 3.可以通过Rc::upgrade方法返回Option<Rc<T>>对象。
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("a tail = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    

    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    println!("a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("b tail = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    println!("a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("a tail = {:?}", a.tail());

}
