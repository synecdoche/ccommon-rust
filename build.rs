extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let _git_cmd_output = Command::new("git")
        .args(&["clone", "https://github.com/twitter/ccommon.git"])
        .output().unwrap();

    let path_to_ccommon = "ccommon";
    let mut dst = cmake::Config::new(path_to_ccommon)
        .build();
    dst.push("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());

    // Tell cargo to tell rustc to link ccommon.
    println!("cargo:rustc-link-lib=ccommon");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let include_path = out_path.join("include/ccommon-1.2");
    let config_h = out_path.join("build/config.h");
    // hack: copy config.h to the include directory.
    std::fs::copy(config_h, &include_path.join("config.h")).unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.display()))
        // Skip layout tests for now.
        .layout_tests(false)
        .blacklist_type("IPPORT_RESERVED")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
