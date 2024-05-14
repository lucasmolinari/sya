mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("5 + 1 * (1 - 3)");
    dbg!(tokenizer.parse().unwrap());
}
