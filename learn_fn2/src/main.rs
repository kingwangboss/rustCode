fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn takes_ownership1(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}
fn main() {
    let s: String = String::from("hello");
    takes_ownership(s);
    //println!("{}", s); 所有权变更不能使用s

    let s1: String = String::from("world");
    let s2: String = takes_ownership1(s1);
    println!("{}", s2); //通过函数返回使用

    let x = 5;
    makes_copy(x);
    println!("i = {}", x);
}
