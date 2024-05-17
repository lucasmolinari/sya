#[cfg(test)]
mod tests;

mod number;
mod sya;
mod tokenizer;

use sya::Sya;

fn main() -> Result<(), String> {
    let mut sya = Sya::new("5 * 3.5")?;
    match sya.calculate() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
    println!("RPN: {}", sya.rpn_formatted());
    println!("Result: {:?}", sya.out.unwrap());
    Ok(())
}
