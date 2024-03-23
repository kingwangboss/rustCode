fn main() {
    //1.创建一个空String
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);
    
    //2.通过字面值创建String
    let s1 = String::from("init some thing");
    // let s1 = "init some thing".to_string();
    println!("s1 = {}", s1);

    //3.更新字符串
    let mut s2 = String::from("hello");
    s2.push_str(", world");
    println!("s2 = {}", s2);
    let ss = "!".to_string();
    s2.push_str(&ss);
    println!("ss = {}", ss);

    let mut s2 = String::from("tea");
    s2.push('m');
    println!("s2 = {}", s2);

    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    // let s3 = s1.clone() + &s2;
    println!("s3 = {}", s3);
    // println!("s1 = {}", s1);

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");
    let ss4 = format!("{}-{}-{}", ss1, ss2, ss3);
    println!("ss4 = {}", ss4);

    //4.索引
    let hello = "你好";
    let h = &hello[0..3];
    println!("h = {}", h);

    //5.遍历chars
    for c in hello.chars() {
        println!("c = {}", c);
    }

    //5.遍历bytes
    for b in hello.bytes() {
        println!("b = {}", b);
    }

}
