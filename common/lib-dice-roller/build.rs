use flatc_rust;

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=flatbuffer_schemata/roll_request.fbs");
    println!("cargo:rerun-if-changed=flatbuffer_schemata/roll_response.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[
            Path::new("flatbuffer_schemata/roll_request.fbs"),
            Path::new("flatbuffer_schemata/roll_response.fbs"),
        ],
        out_dir: Path::new("flatbuffer_generated/"),
        ..Default::default()
    }).expect("flatc");
}
