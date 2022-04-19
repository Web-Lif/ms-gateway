// use std::env;
// use std::path::Path;
// use std::path::PathBuf;

// fn get_output_path() -> PathBuf {
//     let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let build_type = env::var("PROFILE").unwrap();
//     let path = Path::new(&manifest_dir_string).join("target").join(build_type);
//     return PathBuf::from(path);
// }

// fn copy(source: &str, target: &str) {
//     let output_path = get_output_path();
//     let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(source);
//     let output_path = Path::new(&output_path).join(target);
//     std::fs::copy(input_path, output_path).unwrap();
// }

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
}