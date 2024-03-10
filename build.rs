extern crate bindgen;
extern crate cmake;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

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
    let dst = cmake::Config::new("open62541")
        .define("UA_ENABLE_NODESETLOADER", "ON")
        .build();

    println!("cargo:rustc-link-lib=open62541");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include", dst.display()))
        .clang_arg("-lopen62541")
        .header("wrapper.h")
        // bindgen generates erroneous #doc-statements, which do contain
        // '~~'-strings, which make cargo test fail thus comments are disabled
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("Wrote {}/bindings.rs", env::var("OUT_DIR").unwrap());
}
