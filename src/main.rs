mod dsl;

fn main() {
    let mut lx = dsl::tokenizer::Lexer::new(
        &String::from("select * from table where name = \"Jhon\"\norder by id;")
    );
    lx.tokenize();
}
