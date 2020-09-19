use prost_build;

fn main() {
    generate_protobuf_files();
}

fn generate_protobuf_files() {
    println!("cargo:rerun-if-changed=protobuf_schemata/roll_request.proto");
    println!("cargo:rerun-if-changed=protobuf_schemata/roll_response.proto");

    let mut config = prost_build::Config::default();

    config.out_dir("src/roll_generated");

    config
        .compile_protos(
            &[
                "protobuf_schemata/roll_request.proto",
                "protobuf_schemata/roll_response.proto",
            ],
            &["protobuf_schemata/"],
        )
        .unwrap();
}
