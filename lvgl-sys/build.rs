use std::{env, path::PathBuf};
use cc::Build;

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let root_dir = project_dir.parent().unwrap();
    let vendor = root_dir.join("vendor");
    let src = vendor.join("lvgl").join("src");

    let mut cfg = Build::new();

    cfg.file(src.parent().unwrap().join("lvgl.h"))
        .define("LV_CONF_INCLUDE_SIMPLE", Some("1"))
        .include(&src)
        .warnings(false);

    // TODO: Make it configurable! Needs to be linked to final proj defs, define as an env var.
    cfg.include(vendor.join("lv_sim_eclipse_sdl"));

    cfg.compile("lvgl");
}
