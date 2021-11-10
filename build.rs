fn main() {
    tonic_build::compile_protos("proto/dds.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
