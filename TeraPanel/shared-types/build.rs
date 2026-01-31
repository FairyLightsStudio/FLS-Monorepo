use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().expect("Failed to get workspace root");
    let idl_dir = workspace_root.join("idl");
    
    println!("cargo:rerun-if-changed={}", idl_dir.display());

    let terapanel_dir = idl_dir.join("terapanel");
    let mut proto_files = Vec::new();
    fn collect_proto_files(dir: &PathBuf, proto_files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_proto_files(&path, proto_files);
                } else if path.extension().map(|ext| ext == "proto").unwrap_or(false) {
                    proto_files.push(path);
                }
            }
        }
    }
    collect_proto_files(&terapanel_dir, &mut proto_files);

    println!("cargo:warning=Proto files: {:?}", proto_files);

    pilota_build::Builder::pb()
        .include_dirs(vec![idl_dir])
        .split_generated_files(true)
        .change_case(true)
        .ignore_unused(false)
        .with_comments(true)
        .with_descriptor(true)
        // .with_field_mask(true)
        .compile(
            &proto_files,
            pilota_build::Output::File(out_dir.join("proto_gen.rs")),
        );
}
