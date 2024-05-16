#[cfg(test)]
mod tests;

mod sya;
mod tokenizer;

use sya::Sya;

fn main() -> Result<(), String> {
    let mut sya = Sya::new("5 ^ 2")?;
    match sya.calculate() {
        Ok(_) => println!("Result: {}", sya.out.unwrap()),
        Err(e) => println!("Error: {}", e),
    }
    println!("{}", sya.rpn_formatted());
    Ok(())
}
