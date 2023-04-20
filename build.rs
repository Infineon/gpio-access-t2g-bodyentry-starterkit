//! This build script copies the `memory.x` file from the crate root into a
//! directory where the linker can always find it at build time. For many
//! projects this is optional, as the linker always searches the project root
//! directory (i.e. wherever `Cargo.toml` is). However, if you are using a
//! workspace or have a more complicated build setup, this build script becomes
//! mandatory. Additionally, by requesting that Cargo re-run the build script
//! whenever `memory.x` is changed, updating `memory.x` ensures a rebuild of
//! the application with the new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var_os("TARGET").unwrap();

    // Put `memory_cmY.x` in our output directory and ensure it's on the linker
    // search path ('Y' = 0 or 4, depending on which target is being compiled)
    let mut file_mem = File::create(out.join("memory.x")).unwrap();
    if target == "thumbv7em-none-eabihf" {
        file_mem.write_all(include_bytes!("memory_cm4.x")).unwrap();
        println!("cargo:rerun-if-changed=memory_cm4.x");
        println!("cargo:rustc-cfg=cm4");
		
    } else if target == "thumbv6m-none-eabi" {
        file_mem.write_all(include_bytes!("memory_cm0.x")).unwrap();
        println!("cargo:rerun-if-changed=memory_cm0.x");
        println!("cargo:rustc-cfg=cm0");
    } else {
        panic!("Unexpected target");
    }
   
    println!("cargo:rustc-link-search={}", out.display());
}
