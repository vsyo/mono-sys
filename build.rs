use bindgen::Builder;

const WRAPPER_HEADER_PATH: &str = "wrapper.h";
const OUTPUT_FILENAME: &str = "bindings.rs";

fn main() {
    // Look for mono library using pkg-config
    let libmono_2 = pkg_config::Config::default()
        .probe("mono-2")
        .expect("could not find mono library");

    println!("found mono library!");
    println!("{:#?}", libmono_2);

    // Tell cargo to tell rustc to link the system libraries.
    println!("cargo:rustc-link-lib=mono-2.0");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=z");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={WRAPPER_HEADER_PATH}");

    let bindings = Builder::default()
        .header(WRAPPER_HEADER_PATH)
        .clang_args(
            libmono_2.include_paths
                .iter()
                .map(|path| "-I".to_owned() + path.to_str().unwrap()),
        )
        .generate()
        .expect("failed to generate bindings");

    let out_dir = match std::env::var("OUT_DIR").ok() {
        Some(dir) => dir,
        None => "src".to_owned(),
    };

    let out_path = std::path::PathBuf::from(out_dir);

    bindings
        .write_to_file(out_path.join(OUTPUT_FILENAME))
        .expect("failed to write bindings to file");
}
