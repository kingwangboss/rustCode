//trait直接作为参数写法
// fn print_information(item: impl GetInformation)
//trait_bound语法
// fn print_information<T: GetInformation>(item: T)

trait GetName {
    fn get_name(&self) -> &String;
}
trait GetAge {
    fn get_age(&self) -> u32;
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}
//使用trait_bound写法1
fn print_information<T: GetName + GetAge>(item: T) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}
//使用trait_bound写法2
// fn print_information<T>(item: T) where T: GetName + GetAge {
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    print_information(s);
}
