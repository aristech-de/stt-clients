fn main() {
    let proto_file = "protos/stt_service.proto";

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&[proto_file], &["protos"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
