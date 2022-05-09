use std::env;

struct Calc {
    n1: u32,
    operation: String,
    n2: u32,

}

fn main() {
    let first_number = std::env::args().nth(1).expect("no number is given");
    let operator = std::env::args().nth(2).expect("no operation selected");
    let second_number = std::env::args().nth(3).expect("no number is given");

}
