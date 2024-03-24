//泛型
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut larger = list[0];
    for item in list.iter() {
        if *item > larger {
            larger = *item;
        }
    }
    larger
}

//在结构体中使用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![1, 2, 23, 45, 8, 102];
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);

    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);

    let integer = Point{x: 1, y: 2};
    println!("{:#?}", integer);
    let float = Point{x: 1.1, y: 2.33};
    println!("{:#?}", float);
    
    println!("integer.x = {}", integer.get_x());
}
