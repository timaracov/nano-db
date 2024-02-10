use super::tokenizer::Lexer;

struct QueryExpr {}

struct SelectQuery {
    columns: &str[]
}

pub struct AbstractTree {
    query: QueryExpr
}

pub struct Parser {
    lex: Lexer
}

impl Parser {
    pub fn new(lex: Lexer) -> Parser {
        return Self{lex}
    }

    pub fn parse(&mut self) -> AbstractTree { 
        let tokens = self.lex.tokenize();
        for token in tokens {
            println!(
                ".({}, {}-{})",
                token.t_type, token.start_pos, token.end_pos
            );
        }
        AbstractTree {
            query: QueryExpr {}
        }
    }
}
