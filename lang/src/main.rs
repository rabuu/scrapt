use chumsky::{input::Input, Parser};

fn main() {
    let code = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (tokens, tok_errs) = scrapt_lang::lexer().parse(&code).into_output_errors();

    if let Some(toks) = tokens {
        let (var_header, errs) = scrapt_lang::headers::vars_header_parser()
            .map_with(|header, e| (header, e.span()))
            .parse(toks.as_slice().spanned((code.len()..code.len()).into()))
            .into_output_errors();

        if let Some((header, _)) = var_header {
            for (ident, value) in header {
                println!("{ident} => {value:?}");
            }
        }

        eprintln!("HEADER ERRS --------------");
        for err in errs {
            eprintln!("{err:?}");
        }
    }

    eprintln!("ERRORS -------------");
    for err in tok_errs {
        eprintln!("{:?}", err);
    }
}
