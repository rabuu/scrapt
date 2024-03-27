#[test]
fn parse_minimal_example() {
    let minimal_example = r#"costumes { backdrop1: SVG; }"#;
    let toks = lang::lex::tokenize(minimal_example).unwrap();

    // TODO: also check integrity of result
    let _ = lang::parse::parse_target(toks).unwrap();
}
