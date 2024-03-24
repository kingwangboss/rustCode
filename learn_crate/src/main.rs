// use mylib::factory::produce_refrigerator;
// use mylib::factory::produce_refrigerator as A;
// use mylib::factory::{produce_refrigerator, produce_washing_maching};
use mylib::factory::*;
use mylib::mystruct::*;

fn main() {
    produce_refrigerator::produce_re();
    // A::produce_re();

    let a = mod_a::A::new_a();
    a.print_a();
}
