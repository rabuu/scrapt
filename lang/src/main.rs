use chumsky::Parser;

fn main() {
    let code = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let tokens = scrapt_lang::lexer().parse(&code).into_output();

    if let Some(toks) = tokens {
        for tok in toks {
            println!("{:?}", tok);
        }
    }
}
