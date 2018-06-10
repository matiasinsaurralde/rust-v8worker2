use std::process::Command;
use std::env;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-lib=static=v8worker");
    Command::new("g++").args(&["src/binding.cc",
                        "-I/Users/matias/dev/v8worker2/v8/include",
                        "-std=c++11", "-lc++", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/binding.o", out_dir))
                       .status().unwrap();
    
    Command::new("ar").args(&["crus", "libbinding.a", "binding.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    Command::new("libtool").args(&["-static", "-o", "libv8worker.a", "libbinding.a","/Users/matias/dev/rust/experiment/libv8_monolith.a"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();    

    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-flags=-l v8worker -L {}", out_dir);

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-flags=-l c++");
}