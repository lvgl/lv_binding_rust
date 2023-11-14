#[cfg(feature = "library")]
use cc::Build;
#[cfg(feature = "drivers")]
use std::collections::HashSet;
use std::{
    env,
    path::{Path, PathBuf},
};

static CONFIG_NAME: &str = "DEP_LV_CONFIG_PATH";

// See https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
#[cfg(feature = "drivers")]
#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);
#[cfg(feature = "drivers")]
impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    let project_dir = canonicalize(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()));
    let shims_dir = project_dir.join("shims");
    let vendor = project_dir.join("vendor");
    println!("cargo:rerun-if-env-changed={}", CONFIG_NAME);
    let lv_config_dir = get_conf_path(&vendor);
    let font_extra_src: Option<PathBuf> = get_font_extra_dir();
    if let Some(p) = &font_extra_src {
        println!("cargo:rerun-if-changed={}", p.to_str().unwrap())
    }

    let conf = BuildConf {
        lv_config_dir: lv_config_dir.as_path(),
        vendor: vendor.as_path(),
        shims_dir: &shims_dir,
        font_extra_src: font_extra_src.as_ref().map(PathBuf::as_path),
    };

    #[cfg(feature = "library")]
    compile_library(&conf);

    generate_bindings(&conf);
}

fn get_font_extra_dir() -> Option<PathBuf> {
    if let Ok(v) = env::var("PWD") {
        let current_dir = canonicalize(PathBuf::from(v));
        if let Ok(p) = env::var("LVGL_FONTS_DIR") {
            Some(canonicalize(PathBuf::from(p)))
        } else if current_dir.join("fonts").exists() {
            Some(current_dir.join("fonts"))
        } else {
            None
        }
    } else {
        None
    }
}

struct BuildConf<'a> {
    lv_config_dir: &'a Path,
    vendor: &'a Path,
    shims_dir: &'a Path,
    font_extra_src: Option<&'a Path>,
}

#[cfg(feature = "library")]
fn compile_library(conf: &BuildConf) {
    let vendor = conf.vendor;

    let lvgl_src = vendor.join("lvgl").join("src");
    #[cfg(feature = "rust_timer")]
    let timer_shim = vendor.join("include").join("timer");

    // Some basic defaults; SDL2 is the only driver enabled in the provided
    // driver config by default
    #[cfg(feature = "drivers")]
    let incl_extra =
        env::var("LVGL_INCLUDE").unwrap_or("/usr/include,/usr/local/include".to_string());
    #[cfg(feature = "drivers")]
    let link_extra = env::var("LVGL_LINK").unwrap_or("SDL2".to_string());

    #[cfg(feature = "drivers")]
    let drivers = vendor.join("lv_drivers");

    #[cfg(feature = "drivers")]
    {
        println!("cargo:rerun-if-env-changed=LVGL_INCLUDE");
        println!("cargo:rerun-if-env-changed=LVGL_LINK");
    }

    let mut cfg = Build::new();
    if let Some(p) = conf.font_extra_src {
        add_c_files(&mut cfg, p)
    }
    add_c_files(&mut cfg, &lvgl_src);
    add_c_files(&mut cfg, conf.shims_dir);
    #[cfg(feature = "drivers")]
    add_c_files(&mut cfg, &drivers);

    cfg.define("LV_CONF_INCLUDE_SIMPLE", Some("1"))
        .include(&lvgl_src)
        .include(&vendor)
        .warnings(false)
        .include(conf.lv_config_dir);
    if let Some(p) = conf.font_extra_src {
        cfg.includes(p);
    }
    #[cfg(feature = "rust_timer")]
    cfg.include(&timer_shim);
    #[cfg(feature = "drivers")]
    cfg.include(&drivers);
    #[cfg(feature = "drivers")]
    cfg.includes(incl_extra.split(','));

    cfg.compile("lvgl");

    #[cfg(feature = "drivers")]
    link_extra.split(',').for_each(|a| {
        println!("cargo:rustc-link-lib={a}");
        //println!("cargo:rustc-link-search=")
    });
}
fn generate_bindings(conf: &BuildConf) {
    let mut cc_args = vec![
        "-DLV_CONF_INCLUDE_SIMPLE=1",
        "-I",
        conf.lv_config_dir.to_str().unwrap(),
        "-I",
        conf.vendor.to_str().unwrap(),
        "-fvisibility=default",
    ];

    // Set correct target triple for bindgen when cross-compiling
    let target = env::var("CROSS_COMPILE").map_or_else(
        |_| env::var("TARGET").expect("Cargo build scripts always have TARGET"),
        |c| c.trim_end_matches('-').to_owned(),
    );
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    if target != host {
        cc_args.push("-target");
        cc_args.push(target.as_str());
    }

    let mut additional_args = Vec::new();
    if target.ends_with("emscripten") {
        match env::var("EMSDK") {
            Ok(em_path) =>
        {
            additional_args.push("-I".to_string());
            additional_args.push(format!(
                "{}/upstream/emscripten/system/include/libc",
                em_path
            ));
            additional_args.push("-I".to_string());
            additional_args.push(format!(
                "{}/upstream/emscripten/system/lib/libc/musl/arch/emscripten",
                em_path
            ));
            additional_args.push("-I".to_string());
            additional_args.push(format!(
                "{}/upstream/emscripten/system/include/SDL",
                em_path
            ));
        }
        Err(_) => panic!("The EMSDK environment variable is not set. Has emscripten been properly initialized?")
        }
    }

    #[cfg(feature = "drivers")]
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings =
        bindgen::Builder::default().header(conf.shims_dir.join("lvgl_sys.h").to_str().unwrap());
    let bindings = add_font_headers(bindings, conf.font_extra_src);
    #[cfg(feature = "drivers")]
    let bindings = bindings
        .header(conf.shims_dir.join("lvgl_drv.h").to_str().unwrap())
        .parse_callbacks(Box::new(ignored_macros));
    //#[cfg(feature = "rust_timer")]
    //let bindings = bindings.header(shims_dir.join("rs_timer.h").to_str().unwrap());
    let bindings = bindings
        .generate_comments(false)
        .derive_default(true)
        .layout_tests(false)
        .use_core()
        .ctypes_prefix("cty")
        .clang_args(&cc_args)
        .clang_args(&additional_args)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Can't write bindings!");
}

fn get_conf_path(vendor: &PathBuf) -> PathBuf {
    let conf_path = env::var(CONFIG_NAME)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            match std::env::var("DOCS_RS") {
                Ok(_) => {
                    // We've detected that we are building for docs.rs
                    // so let's use the vendored `lv_conf.h` file.
                    vendor.join("include")
                }
                Err(_) => {
                    #[cfg(not(feature = "use-vendored-config"))]
                    panic!(
                        "The environment variable {} is required to be defined",
                        CONFIG_NAME
                    );

                    #[cfg(feature = "use-vendored-config")]
                    vendor.join("include")
                }
            }
        });

    if !conf_path.exists() {
        panic!(
            "Directory {} referenced by {} needs to exist",
            conf_path.to_string_lossy(),
            CONFIG_NAME
        );
    }
    if !conf_path.is_dir() {
        panic!("{} needs to be a directory", CONFIG_NAME);
    }
    if !conf_path.join("lv_conf.h").exists() {
        panic!(
            "Directory {} referenced by {} needs to contain a file called lv_conf.h",
            conf_path.to_string_lossy(),
            CONFIG_NAME
        );
    }
    #[cfg(feature = "drivers")]
    if !conf_path.join("lv_drv_conf.h").exists() {
        panic!(
            "Directory {} referenced by {} needs to contain a file called lv_drv_conf.h",
            conf_path.to_string_lossy(),
            CONFIG_NAME
        );
    }

    println!(
        "cargo:rerun-if-changed={}",
        conf_path.join("lv_conf.h").to_str().unwrap()
    );
    #[cfg(feature = "drivers")]
    println!(
        "cargo:rerun-if-changed={}",
        conf_path.join("lv_drv_conf.h").to_str().unwrap()
    );
    conf_path
}

fn add_font_headers(bindings: bindgen::Builder, dir: Option<&Path>) -> bindgen::Builder {
    if let Some(p) = dir {
        let mut temp = bindings;
        for e in p.read_dir().unwrap() {
            let e = e.unwrap();
            let path = e.path();
            if !e.file_type().unwrap().is_dir()
                && path.extension().and_then(|s| s.to_str()) == Some("h")
            {
                temp = temp.header(path.to_str().unwrap());
            }
        }
        temp
    } else {
        bindings
    }
}

#[cfg(feature = "library")]
fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    for e in path.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            add_c_files(build, e.path());
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}

fn canonicalize(path: impl AsRef<Path>) -> PathBuf {
    let canonicalized = path.as_ref().canonicalize().unwrap();
    let canonicalized = &*canonicalized.to_string_lossy();

    PathBuf::from(canonicalized.strip_prefix(r"\\?\").unwrap_or(canonicalized))
}
