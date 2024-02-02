mod dsl;

#[test]
fn valid_simple_query() {
    let sql_query = "select * from table_name;";
    let mut lex = dsl::tokenizer::Lexer::new(&String::from(sql_query));
    let toks = lex.tokenize();
    let expected_tokens = Vec::from([
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::LITERAL, 
            value: "select".to_string(),
            start_pos: 0,
            end_pos: 5,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::SPACE,
            value: " ".to_string(),
            start_pos: 6,
            end_pos: 6,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::STAR,
            value: "*".to_string(),
            start_pos: 7,
            end_pos: 7,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::SPACE,
            value: " ".to_string(),
            start_pos: 8,
            end_pos: 8,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::LITERAL, 
            value: "from".to_string(),
            start_pos: 9,
            end_pos: 12,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::SPACE,
            value: " ".to_string(),
            start_pos: 13,
            end_pos: 13,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::LITERAL, 
            value: "table_name".to_string(),
            start_pos: 14,
            end_pos: 23,
        },
        dsl::tokenizer::Token {
            t_type: dsl::tokenizer::TokenType::SEMICOL,
            value: ";".to_string(),
            start_pos: 24,
            end_pos: 24,
        },
    ]);

    assert_eq!(toks, expected_tokens);
}
