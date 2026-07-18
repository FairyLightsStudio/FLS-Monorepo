fn main() {
    connectrpc_build::Config::new()
        .files(&["testproto/greet.proto"])
        .includes(&["testproto/"])
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
