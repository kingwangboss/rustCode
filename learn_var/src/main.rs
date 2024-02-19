const MAX_POINT: u32 = 1000000;
fn main() {
    //1.定义变量
    //定义变量用let,如果变量没用mut,那么是不可变的
    let a  = 1;
    println!("a = {}", a);

    let mut b: u32 = 1;
    println!("b = {}", b);

    b = 2;
    println!("b = {}",b);

    //2.隐藏
    let b: f32 = 1.1;
    println!("b = {}",b);

    //3.常量
    println!("MAX_POINT = {}", MAX_POINT);

}
