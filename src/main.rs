#![feature(let_chains)]

use adamath::lexer::{Lexer, TokenType};

fn main() {
    let to_parse = "5 <= 35*test^2 <= 35";

    let mut lexer = Lexer::new(to_parse);

    loop {
        match lexer.scan_token() {
            Ok(Some(token)) => {
                if token.token_type == TokenType::EndOfExpression {
                    return;
                }
                println!("{:?}", token.token_type);
            }
            Err(error) => eprintln!("{error}"),

            Ok(None) => return,
        }
    }
}
