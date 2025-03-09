use std::path::PathBuf;
use std::env;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy the runtime code and the linker script to the target directory.
    std::fs::copy("crt0.S", out.join("crt0.S")).unwrap();
    std::fs::copy("neorv32.ld", out.join("neorv32.ld")).unwrap();

    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    // Check if the #[cfg()] options are correctly used, refer to:
    // https://doc.rust-lang.org/rustc/check-cfg.html#specifying-expected-names-and-values
    println!("cargo:rustc-check-cfg=cfg(native)");
    println!("cargo:rustc-check-cfg=cfg(neorv32)");

    // Activate the neorv32 cfg if it's available.
    if target == host {
        println!("cargo:rustc-cfg=native");
    } else if target.starts_with("riscv32") && target.ends_with("unknown-none-elf") {
        println!("cargo:rustc-cfg=neorv32");
    }

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=crt0.S");
    println!("cargo:rerun-if-changed=neorv32.ld");
}
