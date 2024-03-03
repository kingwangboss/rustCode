//1.字符串slice是String中一部分值的引用
//2.字面值就是slice
//3.其他类型slice
fn main() {
    let s = String::from("hello world");
    // let h = &s[0..5];
    // let h = &s[0..=4];
    // let h = &s[..=4];
    let h = &s[..5];
    println!("h = {}", h);

    // let w = &s[6..11];
    // let w = &s[6..=10];
    let w = &s[6..];
    println!("w = {}", w);

    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("sss = {:?}", sss);
}
