mod dsl;


fn main() {
    let lx = dsl::tokenizer::Lexer::new(
        &String::from("select * from table where name = \"Jhon\"\norder by id;")
    );
    let mut ast_tree = dsl::ast::Parser::new(lx);
    ast_tree.parse();
}
