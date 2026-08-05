use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap().to_path_buf();
    let idl_dir = workspace_root.join("idl");

    println!("cargo:rerun-if-changed={}", idl_dir.display());

    // Collect proto files from all IDL subdirectories
    let mut proto_files = Vec::new();
    fn collect(dir: &PathBuf, out: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect(&path, out);
                } else if path.extension().map(|e| e == "proto").unwrap_or(false) {
                    out.push(path);
                }
            }
        }
    }
    collect(&idl_dir, &mut proto_files);

    connectrpc_build::Config::new()
        .files(&proto_files)
        .includes(&[idl_dir.clone()])
        .emit_register_fn(false)
        .include_file("proto_gen.rs")
        .compile()
        .unwrap();
}
