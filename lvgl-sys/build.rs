use cc::Build;
use std::{env, path::Path, path::PathBuf};

static CONFIG_NAME: &str = "DEP_LV_CONFIG_PATH";

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let shims_dir = project_dir.join("shims");
    let vendor = project_dir.join("vendor");
    let vendor_src = vendor.join("lvgl").join("src");

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

    let mut cfg = Build::new();
    add_c_files(&mut cfg, vendor_src.join("lv_core"));
    add_c_files(&mut cfg, vendor_src.join("lv_draw"));
    add_c_files(&mut cfg, vendor_src.join("lv_font"));
    add_c_files(&mut cfg, vendor_src.join("lv_gpu"));
    add_c_files(&mut cfg, vendor_src.join("lv_hal"));
    add_c_files(&mut cfg, vendor_src.join("lv_misc"));
    add_c_files(&mut cfg, vendor_src.join("lv_themes"));
    add_c_files(&mut cfg, vendor_src.join("lv_widgets"));
    add_c_files(&mut cfg, &lv_config_dir);
    add_c_files(&mut cfg, &shims_dir);

    cfg.define("LV_CONF_INCLUDE_SIMPLE", Some("1"))
        .include(&vendor_src)
        .include(&vendor)
        .file("string.c")
        .warnings(false)
        .include(&lv_config_dir)
        .compile("lvgl");

    let cc_args = [
        "-DLV_CONF_INCLUDE_SIMPLE=1",
        "-I",
        lv_config_dir.to_str().unwrap(),
        "-I",
        vendor.to_str().unwrap(),
    ];

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header(shims_dir.join("lvgl_sys.h").to_str().unwrap())
        .layout_tests(false)
        .use_core()
        .rustfmt_bindings(true)
        .ctypes_prefix("cty")
        .clang_args(&cc_args)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
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
