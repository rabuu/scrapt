use chumsky::{input::Input, Parser};

fn main() {
    let code = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (tokens, _tok_errs) = scrapt_lang::lexer().parse(&code).into_output_errors();

    if let Some(toks) = tokens {
        let (headers, errs) = scrapt_lang::headers::Headers::parser()
            .parse(toks.as_slice().spanned((code.len()..code.len()).into()))
            .into_output_errors();

        if let Some(headers) = headers {
            println!("{headers:#?}");
        }

        eprintln!("HEADER ERRS --------------");
        for err in errs {
            eprintln!("{err:?}");
        }
    }
}
