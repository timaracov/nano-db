mod dsl;
mod storage;


fn main() {
    let lx = dsl::tokenizer::Lexer::new(
        &String::from("select * from table where name = \"Jhon\"\norder by id;")
    );
    let mut ast_tree = dsl::ast::Parser::new(lx);
    let tree = ast_tree.parse();
    println!(":{:?}", tree);
}
