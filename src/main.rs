mod grammar;
mod parser;

use grammar::*;
use parser::Parser;

fn main() {
    let grammar = "<S> ::= \"(\" <L> \")\" | \"a\"
    <L> ::= <S> <L> | \"\"";
    let g = Grammar::from(grammar);
    println!("{:?}", g);
    let p = Parser::from(&g);
    println!("{:?}", p);
}
