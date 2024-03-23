fn main() {
    //1.创建一个空的vector: Vec<T>
    let mut v: Vec<i32> = Vec::new();

    //2.创建包含初始值的vector
    let v1 = vec![1, 2, 3];

    //3.丢弃vector
    {
        let v2 = vec![1, 2, 3];
    }

    //4.读取元素
    let one: &i32 = &v1[0];
    println!("one = {}", *one);

    //读取推荐的方式，数组越界可以处理none
    match v1.get(1) {
        Some(value) => {
            println!("value = {}", value);
        }
        _ => {
            println!("None");
        }
    }

    //5.更新元素
    let mut v3: Vec<i32> = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    //6.遍历 不可变遍历
    for i in &v3 {
        println!("i = {}", i);
    }
    //6.遍历 可变的遍历
    for i in &mut v3 {
        *i += 1;
        println!("i = {}", i);
    }

    //7.使用枚举
    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32)
    }

    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001)
    ];
    for value in c {
        println!("value = {:?}", value);
    }

}
