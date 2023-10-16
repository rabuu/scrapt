use manifest_scratch::Manifest;

#[test]
fn parse_scratch_manifest() {
    let input = include_str!("./manifest.json");
    Manifest::parse(input).expect("Error while parsing Scratch manifest");
}
