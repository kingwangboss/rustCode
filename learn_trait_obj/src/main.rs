use gui::*;

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
