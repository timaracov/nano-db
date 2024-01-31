mod dsl;

fn main() {
    let mut lx = dsl::tokenizer::Lexer {
        pos: 0,
        current_char: String::from("byba"),
    };
    lx.tokenize("select * from table;".to_string());
}
