extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    let pkg_config_library = pkg_config::Config::new()
        .print_system_libs(false)
        .statik(false)
        .probe("fribidi")
        .unwrap();
    let include_paths = pkg_config_library.include_paths;

    println!("cargo:rustc-link-lib=fribidi");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(include_paths.iter().map(|e| "-I".to_owned() + e.to_str().unwrap()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("fribidi_bindings.rs"))
        .expect("Couldn't write bindings!");
}