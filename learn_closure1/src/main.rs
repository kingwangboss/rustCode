fn main() {
    let use_closure = || {
        println!("This is a closure");
    };
    use_closure();

    let add_one_v2 = |x: u32| -> u32 {
        x + 1
    };
    
    let add_one_v3 = |x| {x + 1};
    
    let add_one_v4 = |x| x + 1;
    println!("v1 = {}, v2 = {}, v3 = {}, v4 = {}", add_one_v1(2), add_one_v2(2), add_one_v3(2), add_one_v4(2));

}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

