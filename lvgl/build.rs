use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, path};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let widgets_rs_path = manifest_dir.join("src/widgets/generated.rs");

    if !widgets_rs_path.exists() {
        println!("Generating `src/widgets/generated.rs`");
        let status = Command::new(manifest_dir.join("../target/debug/lvgl-codegen"))
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
