use std::env;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    first_number: String,
    operation: char,
    second_number: String,

}

impl Cli {

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
}

fn main() {

    let commands = Cli {
        first_number: std::env::args().nth(1).unwrap(),
        operation: std::env::args().nth(2).unwrap().chars().next().unwrap(),
        second_number: std::env::args().nth(3).unwrap()
    };
    

    let first_number = commands.first_number.parse::<f32>().unwrap();
    let second_number = commands.second_number.parse::<f32>().unwrap();
    

    let result = operate(commands.operation, first_number, second_number );
    println!("Result: {}", result);

    
}
 