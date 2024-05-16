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

The remaining number in the stack should be the result of the expression. 🦀
