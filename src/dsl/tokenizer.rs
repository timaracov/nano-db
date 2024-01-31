pub enum TokenType {
    LITERAL, // 'any word'
    STAR,    // '*'
    SPACE,   // ' '
    COMA,    // ','
    LBRACK,  // '('
    RBRACK,  // ')'
    SEMICOL, // ';'
}

pub struct Token {
    t_type: TokenType,
    value: String,
    start_pos: i8,
    end_pos: i8,
}

pub struct Lexer {
    pub pos: i8,
    pub current_char: String,
    pub source_string: String,
}

impl Lexer {
    pub fn tokenize(&mut self, src: String) {
        for c in src.chars() {
            if c.is_alphabetic() {
                let mut literal = String::new();
                while c.is_alphabetic() {
                    literal.push_str(c);
                    self.pos += 1;
                }
            }
        }
    }

    fn build_literal(self, c: char) -> String {
        let mut literal = String::new();
        while c.is_alphabetic() {
            literal.push_str(&String::from(c));
            self.pos += 1;
        }
        return literal;
    }

    fn next_char(self) -> char {
        return &self.source_string[self.pos];
    }
}

// select * from ...
