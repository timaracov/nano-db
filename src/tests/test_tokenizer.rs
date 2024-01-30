#[test]
fn valid_simple_query_0() {
    let sql_query = "select * from table_name;";
    let lex = Lexer::new();
    assert!(lex.tokenize(sql_query) == [
        Token {
            t_type: TokeType.LITERAL, 
            value: "select",
            start_pos: 0,
            end_post: 5,
        },
        Token {
            t_type: TokeType.SPACE,
            value: " ",
            start_pos: 6,
            end_post: 6,
        },
        Token {
            t_type: TokeType.STAR,
            value: "*",
            start_pos: 7,
            end_post: 7,
        },
        Token {
            t_type: TokeType.SPACE,
            value: " ",
            start_pos: 8,
            end_post: 8,
        },
        Token {
            t_type: TokeType.LITERAL, 
            value: "from",
            start_pos: 9,
            end_post: 12,
        },
        Token {
            t_type: TokeType.SPACE,
            value: " ",
            start_pos: 13,
            end_post: 13,
        },
        Token {
            t_type: TokeType.LITERAL, 
            value: "table_name",
            start_pos: 14,
            end_post: 23,
        },
        Token {
            t_type: TokeType.SEMICOL,
            value: ";",
            start_pos: 24,
            end_post: 24,
        },
    ]);
}

