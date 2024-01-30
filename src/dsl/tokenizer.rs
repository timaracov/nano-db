pub enum TokenType{
    LITERAL, // 'any word'
    STAR,    // '*'
    SPACE,   // ' '
    COMA,    // ','
    LBRACK,  // '('
    RBRACK,  // ')'
    SEMICOL, // ';'
}

const tokens = {
    "*": TokenType::STAR,
}

pub struct Token {
    t_type: TokenType,
    value: String,
    start_pos: i8,
    end_pos: i8,
}
