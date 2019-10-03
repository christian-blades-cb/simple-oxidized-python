extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(cbindgen::Config {
            enumeration: cbindgen::EnumConfig {
                rename_variants: Some(cbindgen::RenameRule::ScreamingSnakeCase),
                prefix_with_name: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_language(cbindgen::Language::C)
        .with_include_guard("SIMPLERS_H")
        .with_documentation(true)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("simple.h");
}
