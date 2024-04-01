use gui::*;
// 1.对泛型类型使用trait bound编译器进行的方式是单态化处理，单态化的代码进行的是静态分发。
// 2.使用trait对象时，Rust必须使用动态分发。编译器无法知道所有可能用于trait对象代码的类型，
// 所以它也不知道应该调用哪个类型的哪个方法实现，Rust在运行时使用trait对象中的指针来知道需要调用哪个方法。
// trait对象要求对象安全
// 只有对象安全（object safe）的trait才可以组成trait对象。trait的方法满足以下两条要求才是对象安全的：
// a.返回值类型不为self
// b.方法没有任何泛型类型参数
fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            })
        ]
    };

    s.run();
}
