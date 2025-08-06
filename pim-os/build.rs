use std::env;
use std::fs;
use std::path::PathBuf;

const LINKER_SCRIPT: &str = "aarch64-gem5.ld";

fn main() {
    // Put `aarch64-gem5.ld` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(LINKER_SCRIPT, out.join(LINKER_SCRIPT)).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={LINKER_SCRIPT}");
    println!("cargo:rustc-link-arg=-T{LINKER_SCRIPT}");
}
