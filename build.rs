use std::env;
extern crate cc;

fn main() {
    let v8_include_path = env::var("V8_INCLUDE").unwrap();
    let v8_build_path = env::var("V8_BUILD").unwrap();

    cc::Build::new()
        .cpp(true)
        .cpp_link_stdlib("c++")
        .warnings_into_errors(true)
        .warnings(false)
        .include(v8_include_path)
        .flag("-std=c++11")
        .file("src/binding.cc")
        .compile("binding");

    
    println!("cargo:rustc-link-search=native={}", v8_build_path);
    println!("cargo:rustc-link-lib=static=v8_monolith");
}