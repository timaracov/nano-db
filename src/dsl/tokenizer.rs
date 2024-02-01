use std::collections::HashMap;
use std::vec::Vec;


pub enum TokenType {
    LITERAL, // 'any word'
    STAR,    // '*'
    SPACE,   // ' '
    COMA,    // ','
    LBRACK,  // '('
    RBRACK,  // ')'
    SEMICOL, // ';'
    ILLEGAL  // "ill"
}

pub struct Token {
    t_type: TokenType,
    value: String,
    start_pos: u8,
    end_pos: u8,
}

pub struct Lexer {
    pub pos: u8,
    pub current_char: String,
    pub source_string: String,
}

let TOKEN_TABLE: HashMap<&str, TokenType> = HashMap::from([
    ("*", TokenType::STAR),
]);

impl Lexer {
    pub fn tokenize(&mut self) {
        let mut tokens = Vec::new();
        for c in self.source_string.chars() {
            let start_pos = self.pos;
            if c.is_alphabetic() {
                let literal = self.build_literal();
                tokens.push(Token {
                    t_type: TokenType::LITERAL,
                    value: literal,
                    end_pos: self.pos,
                    start_pos
                });
            } else {
                tokens.push(Token {
                    t_type: TokenType::ILLEGAL,
                    value: "illegal".to_string(),
                    end_pos: self.pos,
                    start_pos
                })
            }
        }
    }

    fn build_literal(&mut self) -> String {
        let mut literal = String::new();
        let mut c = self.next_char();

        while c.is_alphabetic() {
            let prep_c = self.char_to_str(c);
            literal.push_str(prep_c);
            self.pos += 1;
            c = self.next_char();
        }
        return literal;
    }

    fn next_char(self) -> char {
        return self.source_string.chars().nth(self.pos.into()).unwrap();
    }

    fn char_to_str(self, c: char) -> &'static str {
        return c.to_string().as_str();
    }
}

// select * from ...
