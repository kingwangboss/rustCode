fn main() {
    let ref_s = dangle();
    println!("ref_s = {}", ref_s);
}

//1.有了可变引用之后不能再有不可变引用
//2.引用必须有效
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");
    s
}