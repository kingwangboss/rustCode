
fn other_fun() {
    println!("This is a function");
}

fn other_fun1(a: i32, b: i32) {
    println!("a = {}, b = {}", a, b);
}

fn other_fun2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

fn other_fun3(a: i32, b: i32) -> i32 {
    let result = a + b;
    result
}

fn other_fun4(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    other_fun();
    other_fun1(1, 2);

    let result = other_fun2(2, 4);
    println!("result = {}", result);

    let result1 = other_fun3(3, 6);
    println!("result1 = {}", result1);

    let result2 = other_fun4(4, 8);
    println!("result2 = {}", result2);
}
