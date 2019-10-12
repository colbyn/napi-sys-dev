extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=include/node/js_native_api.h");
    println!("cargo:rerun-if-changed=include/node/js_native_api_types.h");
    println!("cargo:rerun-if-changed=include/node/node_api.h");
    println!("cargo:rerun-if-changed=include/node/node_api_types.h");

    let bindings = bindgen::Builder::default()
        .header("include/node/js_native_api.h")
        .header("include/node/js_native_api_types.h")
        .header("include/node/node_api.h")
        .header("include/node/node_api_types.h")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}