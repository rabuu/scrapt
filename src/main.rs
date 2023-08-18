// use std::fs;

// use unscratch::manifest;

fn main() {
    // let mut args = std::env::args().skip(1);
    // let path = args.next().expect("Expected path to JSON file");

    // let file = fs::read_to_string(path).unwrap();

    // let manifest: manifest::unscratch::Manifest = toml::from_str(&file).unwrap();

    // println!("{:#?}", manifest);

    let example_file_path: String = format!(
        "{}/examples/untitled-project/stage.uscr",
        env!("CARGO_MANIFEST_DIR")
    );

    let file = std::fs::read_to_string(example_file_path).unwrap();
    let tokens = unscratch::scratchscript::lex::lex(&file);

    println!("{:#?}", tokens);
}
