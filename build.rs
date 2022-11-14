
fn main() {
    let proto_file = "./cab.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/proto_gen")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-change={}", proto_file);
}