use std::vec::Vec;
use std::fmt;


#[derive(Debug)]
pub enum TokenType {
    KEYWORD,
    LITERAL,
    STAR,
    SPACE,
    COMA,
    LBRACK,
    RBRACK,
    SEMICOL,
    // ILLEGAL 
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

static KEYWORDS: &'static [&str] = &[
    "select",
    "update",
    "delete",
    "from",
];

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

impl Lexer {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while usize::from(self.pos) < self.source_string.len() {
            let start_pos = self.pos;
            let c = self.next_char();
            let tok = match c {
               '*' => Token {
                   t_type: TokenType::STAR,
                   value: "*".to_string(),
                   start_pos: self.pos,
                   end_pos: self.pos
                },
               ' ' => Token {
                   t_type: TokenType::SPACE,
                   value: c.to_string(),
                   start_pos: self.pos,
                   end_pos: self.pos
                },
               ',' => Token {
                   t_type: TokenType::COMA,
                   value: c.to_string(),
                   start_pos: self.pos,
                   end_pos: self.pos
                },
               ';' => Token {
                   t_type: TokenType::SEMICOL,
                   value: c.to_string(),
                   start_pos: self.pos,
                   end_pos: self.pos
                },
               '(' => Token {
                   t_type: TokenType::LBRACK,
                   value: c.to_string(),
                   start_pos: self.pos,
                   end_pos: self.pos
                },
               ')' => Token {
                   t_type: TokenType::RBRACK,
                   value: c.to_string(),
                   start_pos: self.pos,
                   end_pos: self.pos
                },
               _ => {
                if c.is_alphabetic() {
                    let literal = self.build_literal();

                    let tt;
                    if KEYWORDS.contains(&literal.as_str()) {
                        tt = TokenType::KEYWORD;
                    } else {
                        tt = TokenType::LITERAL;
                    }

                    Token {
                        t_type: tt,
                        value: literal,
                        end_pos: self.pos,
                        start_pos
                    }
                } else {
                    panic!("\n\tInvalid character '{}' at position: {}", c, self.pos);
                    // Token {
                    //     t_type: TokenType::ILLEGAL,
                    //     value: "ill".to_string(),
                    //     start_pos: self.pos,
                    //     end_pos: self.pos
                    // }
                }
               }
            };
            println!(".({}, '{}', {}-{})", tok.t_type, tok.value, tok.start_pos, tok.end_pos);
            tokens.push(tok);
            self.pos += 1;
        }
        return tokens;
    }

    fn build_literal(&mut self) -> String {
        let mut literal = String::new();
        let mut c = self.next_char();

        while c.is_alphabetic() {
            literal.push(c);
            self.pos += 1;
            c = self.next_char();
        }
        self.pos -= 1;
        return literal;
    }

    fn next_char(&self) -> char {
        self.source_string
            .chars()
            .nth(self.pos.into())
            .unwrap()
    }
}

// select * from ...
