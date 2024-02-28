use std::env;
use std::fs;
use std::path;

use manifest_scrapt::Manifest as ScraptManifest;

fn main() {
    let mut args = env::args().skip(1);

    let project_path = args.next().unwrap();
    let project_path = path::PathBuf::from(project_path);

    let manifest_path = project_path.join("project.toml");
    let manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path).unwrap()).unwrap();

    println!("{:#?}", manifest);
}
