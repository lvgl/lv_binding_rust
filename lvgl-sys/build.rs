use bindgen;
use cc::Build;
use std::{env, path::Path, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let root_dir = project_dir.parent().unwrap();
    let vendor = root_dir.join("vendor");
    let lvgl_sys_src = root_dir.join("lvgl-sys").join("src");
    let src = vendor.join("lvgl").join("src");

    // TODO: Make it configurable! Needs to be linked to final proj defs, define as an env var.
    let lvgl_config_path = vendor.join("lv_sim_eclipse_sdl");

    let mut cfg = Build::new();

    add_c_files(&mut cfg, src.join("lv_core"));
    add_c_files(&mut cfg, src.join("lv_draw"));
    add_c_files(&mut cfg, src.join("lv_font"));
    add_c_files(&mut cfg, src.join("lv_hal"));
    add_c_files(&mut cfg, src.join("lv_misc"));
    add_c_files(&mut cfg, src.join("lv_objx"));
    add_c_files(&mut cfg, src.join("lv_themes"));

    cfg.define("LV_CONF_INCLUDE_SIMPLE", Some("1"))
        .include(&src)
        .warnings(false)
        .include(&lvgl_config_path)
        .compile("lvgl");

    let cc_args = [
        "-DLV_CONF_INCLUDE_SIMPLE=1",
        "-I",
        lvgl_config_path.to_str().unwrap(),
    ];
    bindgen::Builder::default()
        .header(src.parent().unwrap().join("lvgl.h").to_str().unwrap())
        .clang_args(&cc_args)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(lvgl_sys_src.join("bindings.rs"))
        .expect("Can't write bindings!");
}

fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    for e in path.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            // skip dirs for now
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}
