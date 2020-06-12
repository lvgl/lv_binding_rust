use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let widgets_rs_path = manifest_dir.join("src/widgets/generated.rs");
    let codegen_bin = manifest_dir
        .join("..")
        .join("target")
        .join("debug")
        .join("lvgl-codegen")
        .canonicalize()
        .unwrap();
    println!("cargo:rerun-if-changed={}", codegen_bin.to_string_lossy());

    if env::var("LVGL_FORCE_CODEGEN").is_ok() || !widgets_rs_path.exists() {
        println!("Generating `src/widgets/generated.rs`");
        let status = Command::new(codegen_bin)
            .spawn()
            .unwrap_or_else(|_| {
                panic!(
                    "Code generation failed because no codegen executable was found. \
                     Please run `cargo build --package lvgl-codegen` and then try again.",
                )
            })
            .wait()
            .unwrap();
        if !status.success() {
            panic!("Code generation failed");
        }
    }
}
