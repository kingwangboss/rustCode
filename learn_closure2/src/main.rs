//实现一个缓存，只处理第一次传入的值并保存
struct Cacher<T> where T: Fn(u32) -> u32 {
    calcuation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calcuation: T) -> Cacher<T> {
        Cacher {
            calcuation,
            value: None,
        }
    }
    fn get_value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calcuation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {
    let mut c = Cacher::new(|x| x + 1);
    let v1 = c.get_value(1);

    println!("v1 = {}", v1);
}
