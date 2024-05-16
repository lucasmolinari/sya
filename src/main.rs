#[cfg(test)]
mod tests;

mod sya;
mod tokenizer;

use sya::Sya;

fn main() -> Result<(), String> {
    let mut sya = Sya::new("1 + 2 * 4 - 3")?;
    match sya.calculate() {
        Ok(_) => println!("Result: {}", sya.out.unwrap()),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
