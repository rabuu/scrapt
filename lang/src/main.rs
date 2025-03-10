use chumsky::input::Input;
use chumsky::Parser;

fn main() {
    let code = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (tokens, tok_errs) = scrapt_lang::lexer().parse(&code).into_output_errors();

    if let Some(toks) = tokens {
        let (headers, errs) = scrapt_lang::headers::Headers::parser()
            .map_with(|headers, e| (headers, e.span()))
            .parse(
                toks.as_slice()
                    .map((code.len()..code.len()).into(), |(t, s)| (t, s)),
            )
            .into_output_errors();

        if let Some(headers) = headers {
            println!("{headers:#?}");
        }

        eprintln!("HEADER ERRS --------------");
        for err in errs {
            eprintln!("{err:?}");
        }
    }

    eprintln!("LEXER ERRS ----------------");
    for err in tok_errs {
        eprintln!("{err:?}");
    }
}
