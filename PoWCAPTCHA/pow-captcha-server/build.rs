use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let contracts_dir = std::env::var_os("POW_CAPTCHA_CONTRACTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.parent().unwrap().join("contracts"));
    let proto_files = [
        contracts_dir.join("pow_captcha/v1/pow_captcha_backend.proto"),
        contracts_dir.join("pow_captcha/v1/pow_captcha_frontend.proto"),
    ];

    println!("cargo:rerun-if-changed={}", contracts_dir.display());

    let mut include_dirs = vec![contracts_dir];
    if let Some(protobuf_include_dir) = std::env::var_os("PROTOBUF_INCLUDE_DIR") {
        include_dirs.push(PathBuf::from(protobuf_include_dir));
    }

    connectrpc_build::Config::new()
        .files(&proto_files)
        .includes(&include_dirs)
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
