fn main() {
    let mut s1 = String::from("hello");
    let len = calcute_length(&s1);
    //地址传递时，当引用离开其值指向的作用域后也不会被丢弃
    println!("s1 = {}", s1);
    println!("len = {}", len);
    println!("&s1 = {:p}", &s1);
    
    let ms = &mut s1;
    modify_s(ms);
    //借用后不能同时使用s1, ms ;只能使用一个，可变借用不能与不可变引用共存
    // println!("s1 = {}", s1);
    println!("ms = {}", ms);
}


fn calcute_length(s: &String) -> usize {
    println!("&s = {:p}", s);
    s.len()
}

//借用 ： &mut
fn modify_s(s: &mut String) {
    s.push_str(", world");
}