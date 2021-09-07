extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["src/cp_model.proto", "src/sat_parameters.proto"],
        &["src/"],
    )
    .unwrap();

    let ortools_prefix = std::env::var("ORTOOLS_PREFIX")
        .ok()
        .unwrap_or("/opt/ortools".into());
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("src/cp_sat_wrapper.cpp")
        .include(&[&ortools_prefix, "/include"].concat())
        .compile("cp_sat_wrapper.a");

    println!("cargo:rustc-link-lib=dylib=ortools");
    println!("cargo:rustc-link-search=native={}/lib", ortools_prefix);
}