fn main() {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["src/grpc/proto/test.proto", "src/grpc/proto/email.proto"],
            &["src/grpc/proto"],
        )
        .unwrap();
}
