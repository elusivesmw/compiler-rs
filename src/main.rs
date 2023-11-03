use std::fs;
use std::error::Error;

mod lexer;
use lexer::Token;
use lexer::Lexer;

fn main() -> Result<(), &'static str> {
    let path = "./in/source.txt";
    let file = fs::read(path).unwrap();
    let source = String::from_utf8(file).unwrap();

    let mut lex = Lexer::new(source);
    let tokens = lex.get_tokens();
    for (i, token) in tokens.iter().enumerate() {
        //if i > 0 { print!(", "); }
        println!("{:?}", token);
    }
    Ok(())
}

