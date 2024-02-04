use std::fmt;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    KEYWORD(String),    // keyword
    LITERAL(String),    // literal
    STAR(String),       // *
    SPACE(String),      // ' '
    COMA(String),       // ,
    LPAREN(String),     // (
    RPAREN(String),     // )
    SEMICOL(String),    // ;
    ENDLN(String),      // \n
    EQ(String),         // =
    LT(String),         // <
    GT(String),         // >
    NE(String),         // !=
    LTE(String),        // <=
    GTE(String),        // >=
    DQM(String)         // '"'
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

static KEYWORDS: &'static [&str] = &[
    "select",
    "delete",
    "from",
    "order",
    "group",
    "union",
    "not",
    "or",
    "is",
    // types
    "varchar",
    "date",
    "numeric",
    "text",
    // end
    "primary",
    "add",
    "constraint",
    "all",
    "alter",
    "column",
    "and",
    "any",
    "as",
    "asc",
    "backup",
    "database",
    "between",
    "case",
    "check",
    "column",
    "constraint",
    "database",
    "index",
    "table",
    "unique",
    "index",
    "create",
    "view",
    "database",
    "default",
    "desc",
    "distinct",
    "column",
    "constraint",
    "database",
    "default",
    "index",
    "drop",
    "view",
    "exists",
    "foreign",
    "key",
    "from",
    "full",
    "outer",
    "group",
    "having",
    "in",
    "index",
    "inner",
    "insert",
    "into",
    "insert",
    "into",
    "is",
    "null",
    "is",
    "not",
    "null",
    "left",
    "join",
    "like",
    "limit",
    "not",
    "not",
    "null",
    "or",
    "order",
    "by",
    "outer",
    "primary",
    "procedure",
    "a",
    "right",
    "rownum",
    "distinct",
    "into",
    "top",
    "set",
    "top",
    "truncate",
    "union",
    "union",
    "all",
    "update",
    "values",
    "view",
    "where",
];

#[derive(PartialEq, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub start_pos: u8,
    pub end_pos: u8,
}

pub struct Lexer {
    pos: u8,
    source_string: String,
}

impl Lexer {
    pub fn new(src: &String) -> Lexer {
        return Self {
            source_string: src.to_lowercase(),
            pos: 0,
        };
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while usize::from(self.pos) < self.source_string.len() {
            let start_pos = self.pos;
            let c = self.next_char();
            let tok = match c {
                '*' => Token {
                    t_type: TokenType::STAR(String::from("*")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                ' ' => Token {
                    t_type: TokenType::SPACE(String::from(" ")),
                    start_pos: self.pos,
                    end_pos: self.pos,

                },
                '"' => Token {
                    t_type: TokenType::DQM('"'.to_string()),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                ',' => Token {
                    t_type: TokenType::COMA(String::from(",")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                ';' => Token {
                    t_type: TokenType::SEMICOL(String::from(";")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                '(' => Token {
                    t_type: TokenType::LPAREN(String::from("(")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                ')' => Token {
                    t_type: TokenType::RPAREN(String::from(")")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                '\n' => Token {
                    t_type: TokenType::ENDLN(String::from("\n")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                '=' => Token {
                    t_type: TokenType::EQ(String::from("=")),
                    start_pos: self.pos,
                    end_pos: self.pos,
                },
                '>' => {
                    self.pos += 1;
                    let c = self.next_char();
                    match c {
                        '=' => Token { 
                            t_type: TokenType::GTE(String::from(">=")),
                            start_pos: self.pos-1,
                            end_pos: self.pos,
                        },
                        ' ' =>  {
                            self.pos -= 1;
                            Token { 
                                t_type: TokenType::GT(String::from(">")),
                                start_pos: self.pos,
                                end_pos: self.pos,
                            }
                        },

                        _ => {
                            self.token_panic(c);
                            panic!();
                        }
                    }
                },
                '<' => {
                    self.pos += 1;
                    let c = self.next_char();
                    match c {
                        '=' => Token { 
                            t_type: TokenType::LTE(String::from("<=")),
                            start_pos: self.pos-1,
                            end_pos: self.pos,
                        },
                        ' ' =>  {
                            self.pos -= 1;
                            Token { 
                                t_type: TokenType::LT(String::from("<")),
                                start_pos: self.pos,
                                end_pos: self.pos,
                            }
                        },

                        _ => {
                            self.pos -= 1;
                            self.token_panic(c);
                            panic!();
                        }
                    }
                },
                '!' => {
                    self.pos += 1;
                    let c = self.next_char();
                    match c {
                        '=' => Token { 
                            t_type: TokenType::NE(String::from("!=")),
                            start_pos: self.pos-1,
                            end_pos: self.pos,
                        },
                        _ => {
                            self.pos -= 1;
                            self.token_panic(c);
                            panic!();
                        }
                    }
                },
                _ => {
                    if c.is_alphabetic() {
                        let literal = self.build_literal();

                        let tt;
                        if KEYWORDS.contains(&literal.as_str()) {
                            tt = TokenType::KEYWORD(literal);
                        } else {
                            tt = TokenType::LITERAL(literal);
                        }

                        Token {
                            t_type: tt,
                            end_pos: self.pos,
                            start_pos,
                        }
                    } else {
                        self.token_panic(c);
                        panic!();
                    }
                }
            };
            println!(
                ".({}, {}-{})",
                tok.t_type, tok.start_pos, tok.end_pos
            );
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
        self.source_string.chars().nth(self.pos.into()).unwrap()
    }

    fn token_panic(&self, c: char) {
        println!("\"{}\"", self.source_string);
        for _ in 0..=self.pos {
            print!(" ");
        }
        print!("▲\n");
        for _ in 0..=self.pos {
            print!(" ");
        }
        print!("|\n");
        for _ in 0..=self.pos {
            print!(" ");
        }
        print!("Invalid character '{}' at position: {}\n\n", c, self.pos);
    }
}
