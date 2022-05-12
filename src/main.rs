use std::env;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    first_number: f32,
    operation: char,
    second_number: f32,

}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {

    let result = match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operation")

    };
    return result
}

fn main() {

    let args = Cli::parse();
    let m = Cli {
        first_number: 1.0,
        operation: '+',
        second_number: 4.0
    };
    
    let first_number = std::env::args().nth(1).parse::<f32>();
    let operator = std::env::args().nth(1);
    let 
    // let operator: String = std::env::args().nth(2).expect("no operation selected");
    // let second_number = std::env::args().nth(3).expect("no number is given");
    let result = operate(first_number, Cli::first_number, second_number)

}
 