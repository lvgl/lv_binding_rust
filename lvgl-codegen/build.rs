use cc::Build;
use std::ffi::OsStr;
use std::process::{Command, Stdio};
use std::{env, fs, path::Path, path::PathBuf};

static CONFIG_NAME: &str = "DEP_LV_CONFIG_PATH";

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let lvgl_sys = project_dir.join("..").join("lvgl-sys");
    let vendor = lvgl_sys.join("vendor");
    let lvgl_top_path = vendor.join("lvgl");
    let lvgl_src_path = lvgl_top_path.join("src");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let lv_config_dir = {
        let raw_path = env::var(CONFIG_NAME).unwrap_or_else(|_| {
            panic!(
                "The environment variable {} is required to be defined",
                CONFIG_NAME
            );
        });
        let conf_path = PathBuf::from(raw_path);

        if !conf_path.exists() {
            panic!(format!(
                "Directory referenced by {} needs to exist",
                CONFIG_NAME
            ));
        }
        if !conf_path.is_dir() {
            panic!(format!("{} needs to be a directory", CONFIG_NAME));
        }
        if !conf_path.join("lv_conf.h").exists() {
            panic!(format!(
                "Directory referenced by {} needs to contain a file called lv_conf.h",
                CONFIG_NAME
            ));
        }

        println!(
            "cargo:rerun-if-changed={}",
            conf_path.join("lv_conf.h").to_str().unwrap()
        );
        conf_path
    };

    let build = Build::new();
    let tool = build.get_compiler();
    let preprocessed = Command::new(tool.path().to_str().unwrap())
        .args(&[
            "-E",
            "-std=c99",
            "-DLV_CONF_INCLUDE_SIMPLE",
            "-I",
            lvgl_top_path.to_string_lossy().as_ref(),
            "-I",
            lvgl_src_path.to_string_lossy().as_ref(),
            "-I",
            lv_config_dir.to_string_lossy().as_ref(),
            lvgl_top_path.join("lvgl.h").to_string_lossy().as_ref(),
        ])
        .output()
        .unwrap();

    let content = String::from_utf8(preprocessed.stdout).unwrap();
    fs::write(out_path.join("lvgl_full.c"), content).unwrap();
}
