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
    // let commands = Calc {
    //     first_number: std::env::args().nth(1).expect("no number is given"),
    //     operator: std::env::args().nth(2).expect("no operation selected"),



    // }
    // let Calc::first_number = 


    // let first_number = std::env::args().nth(1).expect("no number is given");
    let operator: String = std::env::args().nth(2).expect("no operation selected");
    let second_number: u32 = std::env::args().nth(3).expect("no number is given");

}
