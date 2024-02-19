fn main() {
    //if
    let y = 1;
    if y == 1 {
        println!("y = 1");
    }

    let y1 = 0;
    //if-else 
    if y1 == 1 {
        println!("y1 = 1");
    } else {
        println!("y1 != 1");
    }

    //if-else if-else

    let condition = true;
    let x = if condition {
        5
    } else {
        6
    };
    println!("x = {}", x);

    //loop
    let mut count = 1;
    loop {
        println!("in loop {}", count);
        if count == 10 {
            break;
        }
        count += 1;
    }

    let result = loop {
        count += 1;
        println!("count = {}", count);
        if count == 20 {
            break count*2;
        }
    };
    println!("result = {}", result);

    //while 
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    //for
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("element = {}", element);
    }
}
