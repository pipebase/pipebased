fn main() {
    tonic_build::configure()
        .out_dir("src/grpc")
        .compile(&["proto/daemon.proto"], &["proto"])
        .unwrap();
}
