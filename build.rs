fn main() {
    tonic_build::compile_protos("src/grpc/proto/test.proto").unwrap();
}
