use std::{env, path::PathBuf};
use cc::Build;
use bindgen;

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let root_dir = project_dir.parent().unwrap();
    let vendor = root_dir.join("vendor");
    let src = vendor.join("lvgl").join("src");

    // TODO: Make it configurable! Needs to be linked to final proj defs, define as an env var.
    let lvgl_config_path = vendor.join("lv_sim_eclipse_sdl");

    let mut cfg = Build::new();

    cfg.file(src.parent().unwrap().join("lvgl.h"))
        .define("LV_CONF_INCLUDE_SIMPLE", Some("1"))
        .include(&src)
        .warnings(false)
        .include(&lvgl_config_path)
        .compile("lvgl");

    let cc_args = ["-DLV_CONF_INCLUDE_SIMPLE=1", "-I", lvgl_config_path.to_str().unwrap()];
    bindgen::Builder::default()
        .header(src.parent().unwrap().join("lvgl.h").to_str().unwrap())
        .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]")
        .clang_args(&cc_args)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(root_dir.join("lvgl-sys").join("src").join("lib.rs"))
        .expect("Can't write bindings!");
}
