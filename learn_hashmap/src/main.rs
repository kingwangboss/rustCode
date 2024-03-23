use std::collections::HashMap;
fn main() {
    //1.创建HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("red".to_string(), 20);
    let value = scores.get("blue");
    println!("{:?}", value);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores1: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", scores1);

    //2.读取
    let k = String::from("Blue");
    if let Some(v) = scores1.get(&k) {
        println!("v = {}", v);
    }

    match scores1.get(&k) {
        Some(value) => {
            println!("v = {}", value);
        },
        None => {
            println!("none");
        }
    }

    //3.遍历
    for (key, value) in scores1.iter() {
        println!("key = {}, value = {}", key, value);
    }

    //4.更新
    scores.insert(String::from("blue"), 30);
    print!("{:?}", scores);
    //如果键不存在才插入
    scores.entry(String::from("yellow")).or_insert(1);
    print!("{:?}", scores);
    
}
