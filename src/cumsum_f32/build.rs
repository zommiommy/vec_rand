// Example custom build script.
fn main() {
    for (key, value) in std::env::vars() {
        eprintln!("{}: {}", key, value);
    }
    let target_triple = std::env::var("TARGET").unwrap();
    let features_str = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap();
    let features = features_str
        .split("\n").collect::<Vec<_>>();

    if target_triple.starts_with("x86") && features.contains(&"avx2"){
        // Tell Cargo that if the given file changes, to rerun this build script.
        println!("cargo:rerun-if-changed=src/ccumsum.c");
        // Use the `cc` crate to build a C file and statically link it.

        cc::Build::new()
            .file("src/ccumsum.c")
            .flag("-march=native")
            .flag("-O3")
            .compile("ccumsum.a");
    }
}