//1.迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。
//2.创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
//3.每个迭代器都实现了iterator trait, iterator trait定义在标准库中。
trait iterator {
    type Item;
    // next 是 Iterator被要求实现的唯一的一个方法，next一次返回一个元素，当迭代器结束的时候，返回None.
    fn next(&mut self) -> Option<Self::Item>; //type Item 和Self::Item这种方法叫做定义trait的关联类型
}
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("val = {}", val);
    // }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }else {
        println!("End");
    }

    // 迭代器可变引用
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    println!("v2 = {:?}", v2);

    //消费适配器
    let v3 = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum();
    println!("total = {}", total);

    //迭代适配器
    let v4 = vec![1, 2, 3];
    let v5: Vec<_> = v4.iter().map(|x| x+1).collect();
    println!("v5 = {:?}", v5);

    //迭代过滤适配器
    let v6 = vec![1, 12, 3, 45];
    let v7: Vec<_> = v6.into_iter().filter(|x| *x > 5).collect();
    println!("v7 = {:?}", v7);
}
