use std::env;
use clap::Parser;

#[derive(Parser)]
struct Calc {
    first_number: u32,
    operation: String,
    second_number: u32,

}

fn main() {
    let args = Calc::parse();
    // let first_number = std::env::args().nth(1).expect("no number is given");
    // let operator = std::env::args().nth(2).expect("no operation selected");
    // let second_number = std::env::args().nth(3).expect("no number is given");

}
