// 1.Drop trait类似于其他语言的析构函数，当值离开作用域的时候执行此函数。
struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} drop", self.name);
    }
}
fn main() {
    let dog1 = Dog{
        name: String::from("wangcai"),
    };
    // 提前释放
    drop(dog1);
    {
        let _dog2 = Dog {
            name: String::from("dahuang"),
        };
    }

}
