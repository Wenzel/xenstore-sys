#[cfg(not(feature = "manual"))]
use bindgen::{Builder, EnumVariation};

#[cfg(not(feature = "manual"))]
use std::{env, path::PathBuf};

#[cfg(not(feature = "manual"))]
fn auto_bind() {
    let xen_headers_wrapper: &str = "src/wrapper.h";

    println!("cargo:rerun-if-changed={}", xen_headers_wrapper);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(xen_headers_wrapper)
        // Generate bindings for Xen specific types
        // and functions only.
        .whitelist_function("xs_.*")
        // Keep C's enums as Rust's enums.
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        // Disable data layout tests.
        .layout_tests(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = {
        let out_path = env::var("OUT_DIR").expect("Unable to get OUT_DIR environment variable");
        PathBuf::from(out_path)
    };
    bindings
        .write_to_file(out_path.join("auto_bindings.rs"))
        .expect("Unable to write bindings!");
}

fn main() {
    #[cfg(not(feature = "manual"))]
    auto_bind();

    // what library to link with
    println!("cargo:rustc-link-lib={}={}", "dylib", "xenstore");
}
