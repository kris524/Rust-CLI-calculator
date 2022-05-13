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

    // let args = Cli::parse();
    
    
    let first: String = std::env::args().nth(1).unwrap(); 
    println!("The first number is: {}", first);
    let operator: char = std::env::args().nth(2).unwrap().chars().next().unwrap();
    println!("The operator is: {}", operator);
    let second: String = std::env::args().nth(3).unwrap();
    println!("The second number is: {}", second);

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    

    let result = operate(operator, first_number, second_number );
    println!("Result: {}", result);

    let m = Cli {
        first_number: 1.0,
        operation: '+',
        second_number: 4.0
    };
}
 