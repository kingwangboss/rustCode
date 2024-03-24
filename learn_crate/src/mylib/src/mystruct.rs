pub mod mod_a {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A{
            A {
                number: 1,
                name: String::from("A"),
            }
        }

        pub fn print_a(self) {
            println!("number: {}, name: {}", self.number, self.name);
        }
    }
}