fn main() {
    let m = std::fs::read_to_string("./manifest.json").unwrap();
    let m = manifest_scratch::Manifest::parse(&m).unwrap();

    println!("{:#?}", m);
}
