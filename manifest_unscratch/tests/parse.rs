use manifest_unscratch::Manifest;

#[test]
fn parse_unscratch_manifest() {
    let input = include_str!("./manifest.toml");
    Manifest::parse(input).expect("Error while parsing Unscratch manifest");
}
