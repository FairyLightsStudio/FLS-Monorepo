use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().expect("Failed to get workspace root");
    let idl_dir = workspace_root.join("idl");
    let domain_v1_dir = idl_dir.join("terapanel").join("domain").join("v1");

    println!("test {}", domain_v1_dir.display());

    // Tell Cargo to rerun this build script if the IDL directory changes
    println!("cargo:rerun-if-changed={}", idl_dir.display());
    println!("cargo:rerun-if-changed={}", domain_v1_dir.display());

    // 收集所有 .proto 文件
    let mut proto_files = Vec::new();

    for entry in fs::read_dir(&domain_v1_dir).expect("Failed to read domain/v1 directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().map(|ext| ext == "proto").unwrap_or(false) {
            println!("cargo:rerun-if-changed={}", path.display());
            proto_files.push(path);
        }
    }

    if proto_files.is_empty() {
        panic!("No .proto files found in {:?}", domain_v1_dir);
    }

    println!(
        "Compiling {} .proto files from {:?}",
        proto_files.len(),
        domain_v1_dir
    );

    pilota_build::Builder::pb()
        .include_dirs(vec![idl_dir])
        .split_generated_files(true)
        .change_case(true)
        .ignore_unused(false)
        .with_comments(true)
        .with_descriptor(true)
        .compile(
            &proto_files,
            pilota_build::Output::File(out_dir.join("proto_gen.rs")),
        );
}
