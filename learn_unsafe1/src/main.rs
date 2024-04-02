// 1.Rust在编译时会强制执行的内存安全保证。不会强制执行这类内存安全的保证，就是不安全的Rust
// 2.不安全的Rust存在的两大原因：
// a.静态分析的本质上是保守的，就意味着某些代码可能是合法的，但是Rust也会拒绝。在此情况下，可以使用
// 不安全的代码。
// b.底层计算机硬件固有的不安全性。如果Rust不允许进行不安全的操作，有些任务根本就完成不了。
// 3.不安全的Rust具有超级力量
// Rust会通过unsafe关键字切换到不安全的Rust。不安全的Rust具有以下超级力量：
// a.解引用裸指针
// b.调用不安全的函数或者方法
// c.访问或修改可变静态变量
// d.实现不安全的trait

fn main() {
    // 解引用裸指针
    // 1.允许忽略借用规则，可以同时拥有不可变和可变的指针，或者是多个指向相同位置的可变指针
    // 2.不保证指向的内存是有效的
    // 3.允许为空
    // 4.不能实现任何自动清理的功能
    let mut num = 5;
    //创建可变和不可变的裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    unsafe {
        dangerous();
        println!("abs(-3) = {}", abs(-3));
    }

    unsafe {
        HELLO_WORLD = "aaa";
        println!("{}",HELLO_WORLD);
    }

    let a = Bar();
    a.foo();
    
}

// 调用不安全的函数或者方法
unsafe fn dangerous() {
    println!("do something dangerous")
}
// 调用c语言函数
extern "C" {
    fn abs(input: i32) -> i32;
}

// 访问或者修改可变静态变量
static mut HELLO_WORLD: &str = "hello, world";

// 实现不安全的trait
unsafe trait Foo {
    fn foo(&self);
}
struct Bar();

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}