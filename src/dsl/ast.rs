use super::tokenizer::Lexer;

enum Statement {Expression}


struct AbstractAstNode {
    value: Statement,
}

struct AbstractTree {}


impl AbstractTree {
    fn build(lex: &mut Lexer) -> AbstractTree {
        let mut lx = lex.tokenize();
        return Self{};
    }
}
