use manifest_scrapt::Manifest;

#[test]
fn parse_scrapt_manifest() {
    let input = include_str!("./manifest.toml");
    Manifest::parse(input).expect("Error while parsing Scrapt manifest");
}
