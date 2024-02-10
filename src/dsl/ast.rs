use super::tokenizer::{Lexer, TokenType};

#[derive(Debug)]
pub struct AbstractTree {}

pub struct Parser {
    lex: Lexer
}

impl Parser {
    pub fn new(lex: Lexer) -> Parser {
        return Self{lex}
    }

    pub fn parse(&mut self) -> AbstractTree { 
        let token = self.lex.next();
        println!("{}", token.t_type);
        AbstractTree {}
    }

    pub fn match_pattern(&mut self, pattern: &[TokenType]) {
        for tt in pattern {
            let tok = self.lex.next();
            if tt != &tok.t_type {
                break;
            }
        }
    }
}
