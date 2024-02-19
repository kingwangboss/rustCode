use std::collections::btree_map::Values;

fn main() {
    //bool 
    let is_true: bool = true;
    let is_false: bool = false;
    println!("is_true = {}, is_false = {}", is_true, is_false);

    //char 在rust里面，char是32位
    let a = 'a';
    println!("a = {}", a);

    let b = '你';
    println!("b = {}", b);
    
    //i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64 
    let c: i8 = -111;
    println!("c = {}", c);

    let d: f32 = 0.0009;
    println!("d = {}", d);

    //自适应类型 isize, usize
    println!("max = {}", usize::max_value());

    //数组 [Type; size]
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0] = {}", arr[0]);

    show(arr);

    //元组
    let tup:(i32, f32, char) = (-3, 3.69, '好');
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    //元组拆解
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    
}

fn show(arr: [u32; 5]) {
    for (index, value) in arr.iter().enumerate() {
        println!("a[{}] = {}", index, value);
    }
}