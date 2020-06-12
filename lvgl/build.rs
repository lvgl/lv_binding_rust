use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, path};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gen_code = out_path.join("bindings.rs");

    let code = invoke_command("../target/debug/lvgl-codegen", &[""], &project_dir);
    fs::write(&gen_code, code).unwrap();

    // // Format generated code
    // let _ = invoke_command(
    //     "cargo",
    //     &["fmt", "-p", gen_code.to_str().unwrap()],
    //     &project_dir,
    // );
}

fn invoke_command<C, I, S, D>(command: C, args: I, cur_dir: D) -> String
where
    C: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    D: AsRef<path::Path>,
{
    Command::new(command)
        .current_dir(cur_dir)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                panic!(
                    "{}",
                    String::from_utf8_lossy(&output.stderr).trim().to_string()
                );
            }
        })
        .unwrap()
}
