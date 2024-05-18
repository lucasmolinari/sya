#[cfg(test)]
mod tests;

mod number;
mod sya;
mod tokenizer;

use std::io::{self, Write};

use sya::Sya;

fn main() {
    let mut sya = Sya::new("").expect("Should construct");
    println!("q! for exit");
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read input");

        if input.trim() == "q!".to_string() {
            break;
        }

        match sya.new_input(&input.trim()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error parsing input: {}", e);
                continue;
            }
        };

        match sya.calculate() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error calculating expression: {}", e);
                continue;
            }
        }
        println!("RPN: {}", sya.rpn_formatted());
        println!("Result: {:?}", &sya.out.as_ref().unwrap());
    }
}
