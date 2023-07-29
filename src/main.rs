use std::fs;

use unscratch::scratch;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("Expected path to JSON file");

    let file = fs::read_to_string(path).unwrap();

    let manifest: scratch::Manifest = serde_json::from_str(&file).unwrap();

    println!("{:#?}", manifest);
}
