//定义trait
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("红星学校")
    }
}

//实现特征
pub struct Student {
    pub name: String,
    pub age: u32,
}

//默认实现
impl SchoolName for Student {
    
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl SchoolName for Teacher {
    
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

//trait作为参数
fn print_information(item: impl GetInformation) {
    println!("name = {}, age = {}", item.get_name(), item.get_age());
}

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    let t = Teacher{name: "huanglaoshi".to_string(), age: 30, subject:"yuwen".to_string()};
    println!("student, name = {}, age = {}", s.name, s.age);
    println!("teacher, name = {}, age = {}", t.name, t.age);
    
    let s_school_name = s.get_school_name();
    println!("student school name = {}", s_school_name);
    let t_school_name = t.get_school_name();
    println!("tearch school name = {}", t_school_name);
    
    print_information(s);
    print_information(t);
}
