use bindgen::Builder;

const WRAPPER_HEADER_PATH: &str = "wrapper.h";
const OUTPUT_FILENAME: &str = "bindings.rs";

fn main() {
    // Look for mono library using pkg-config
    #[cfg(target_os = "linux")]
    let mono_include_paths = pkg_config::Config::default()
        .probe("mono-2")
        .expect("could not find mono library")
        .include_paths;
    
    #[cfg(target_os = "windows")]
    let mono_include_paths = vec![String::from("C:\\Program Files\\Mono\\include\\mono-2.0")];
    
    println!("found mono library!");
    println!("{:#?}", mono_include_paths);

    // Tell cargo to tell rustc to link the system libraries.
    println!("cargo:rustc-link-lib=mono-2.0");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=z");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={WRAPPER_HEADER_PATH}");

    let bindings = Builder::default()
        .header(WRAPPER_HEADER_PATH)
        .clang_args(
            mono_include_paths
                .iter()
                .map(|path| "-I".to_owned() + path.as_str()),
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
