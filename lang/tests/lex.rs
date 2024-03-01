use lang::lex::*;

#[test]
fn simple_tokenization() {
    use Token::*;

    let input = "+-=,/*:;::->(){}[]<>";
    assert_eq!(
        tokenize(input),
        vec![
            Plus,
            Minus,
            Equal,
            Comma,
            Slash,
            Asterisk,
            Colon,
            Semicolon,
            DoubleColon,
            Arrow,
            ParenL,
            ParenR,
            CurlyL,
            CurlyR,
            BracketL,
            BracketR,
            ChevronL,
            ChevronR,
            Eof
        ]
    );
}

#[test]
fn keyword_tokenization() {
    use Keyword::*;

    let input = "global vars lists broadcasts foo costumes if";
    assert_eq!(
        tokenize(input),
        vec![
            Token::Keyword(Global),
            Token::Keyword(Vars),
            Token::Keyword(Lists),
            Token::Keyword(Broadcasts),
            Token::Ident(String::from("foo")),
            Token::Keyword(Costumes),
            Token::Keyword(If),
            Token::Eof,
        ]
    );
}

#[test]
fn illegal_character() {
    let input = "ü";
    let tokens = tokenize(input);

    assert_eq!(
        tokens[0],
        Token::Illegal("ü".to_string(), "Illegal character")
    );
}

// TODO: more tests
