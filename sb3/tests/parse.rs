use sb3::Project;

#[test]
fn parse_scratch_manifest() {
    let input = include_str!("./project.json");
    Project::parse(input).expect("Error while parsing Scratch manifest");
}
