# SYA
SYA is a [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) wrote in Rust with a Token Parser (Lexer) included.

The Algorithm first converts the Expression into  [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) (**RPN**):

<picture>
  <img alt="Expression to RPN" src="https://ucarecdn.com/6dbfbb2b-7e98-4756-825a-29a17429481e/" width="450">
</picture>


The algorithm then traverses through the **RPN**, everytime an operator is found the operation is applied in the two previous numbers:

<picture>
  <img alt="Traversing RPN" src="https://ucarecdn.com/652213b7-005f-478e-bb57-abb48a10700e/" width="450">
</picture>

>(2+4) ⋅ (4+6) with RPN 24 + 46 + ⋅

The remaining number in the stack should be the result of the expression.

## Usage
First, you can clone the project with `git clone https://github.com/lucasmolinari/sya`. This will create a folder called "sya".

Then you can just use `cargo run` or `cargo run --release` inside the sya directory. (For the Rust/Cargo installation, please refer to  the [Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html))

When the program starts, you can start writting mathematical expressions. Examples:
```
> (2 + 4) * (4 + 6) 
RPN: 2 4 + 4 6 + *
Result: Integer(60)
```
```
> 5 * - (2 + 3)  
RPN: 5 2 3 + u- *
Result: Integer(-25)
```
```
> 3 * 2.5 + 2 ^ 3
RPN: 3 2.5 * 2 3 ^ +
Result: Float(15.5)
```
```
> (1 * 100) - - (3 ^ (4 / 2 + 1) + (2)) * + 30 ^ 1
RPN: 1 100 * 3 4 2 / 1 + ^ 2 + u- 30 u+ 1 ^ * -
Result: Integer(970)
```
```
> 2 / 0
Error calculating expression:
Tried to divide by zero
```

```
> 5 + a
Error parsing input:
Invalid Character 'a'
```

🦀
