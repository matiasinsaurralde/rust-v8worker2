extern crate cc;
extern crate pkg_config;

fn main() {
    let lib = pkg_config::probe_library("v8.pc");
    if lib.is_err() {
        panic!("Couldn't find V8 via pkg-config");
    }
    let v8 = lib.unwrap();
    let incl_path = v8.include_paths[0].to_str().unwrap();
    let lib_path = str::replace(incl_path, "v8/include", "out/v8build/obj");

    let mut build = cc::Build::new();
    build.cpp(true)
        .cpp_link_stdlib("c++")
        .warnings_into_errors(true)
        .warnings(false)
        .include(incl_path)
        .flag("-std=c++11")
        .file("src/binding.cc")
        .compile("binding");
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=v8_monolith");
}