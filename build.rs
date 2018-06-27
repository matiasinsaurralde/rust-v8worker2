extern crate cc;
extern crate pkg_config;

use std::process::Command;

fn main() {
    let pkg_config_file = "v8.pc".to_string();

    let mut lib = pkg_config::probe_library(&pkg_config_file);
    if lib.is_err() {
        // V8 build step:
        let status = Command::new("python")
            .args(&["build.py", "--use_ccache"])
            .status()
            .expect("failed to build V8");

        if !status.success() {
            panic!("Couldn't build V8");
        }
    }
    println!("Successful V8 build, probe v8.pc");

    // V8 library lookup:
    lib = pkg_config::probe_library(&pkg_config_file);
    if lib.is_err() {
        panic!("Couldn't find v8.pc via pkg-config");
    }
    println!("v8.pc found");
    let v8 = lib.unwrap();
    let incl_path = v8.include_paths[0].to_str().unwrap();
    let lib_path = str::replace(incl_path, "v8/include", "out/v8build/obj");

    println!("Binding build");
    // Build settings:
    let mut build = cc::Build::new();
    build.cpp(true)
        .warnings_into_errors(true)
        .warnings(false)
        .include(incl_path)
        .flag("-std=c++11")
        .file("src/binding.cc")
        .compile("binding");

    // Linker settings:
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=v8_monolith");
}
