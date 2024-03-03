#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32
}

impl Dog {
    fn get_name(&self) -> &str {
        &(self.name[..])
    }
    fn get_weight(&self) -> f32 {
        self.weight
    }
    fn get_height(&self) -> f32 {
        self.height
    }
    //实例方法
    fn show(&self) {
        println!("wang wang wang");
    }
    //静态方法
    // fn show() {
    //     println!("wang wang wang");
    // }
}

fn main() {
    let dog = Dog {
        name: String::from("wangcai"),
        weight: 100.00,
        height:70.50
    };

    println!("dog = {:#?}", dog);
    println!("name = {}", dog.get_name());
    println!("weight = {}", dog.get_weight());
    println!("height = {}", dog.get_height());

    dog.show();
}
