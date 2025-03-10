use std::{env, path::PathBuf, fs, io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;
    let host = env::var("HOST")?;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let root = PathBuf::from(&manifest_dir)
        .join("neorv32/sw/lib");

    // Check if the #[cfg()] options are correctly used, refer to:
    // https://doc.rust-lang.org/rustc/check-cfg.html#specifying-expected-names-and-values
    println!("cargo:rustc-check-cfg=cfg(native)");
    println!("cargo:rustc-check-cfg=cfg(neorv32)");

    // Activate the neorv32 cfg if it's available.
    if target == host {
        println!("cargo:rustc-cfg=native");
    } else if target.starts_with("riscv32") && target.ends_with("unknown-none-elf") {
        println!("cargo:rustc-cfg=neorv32");
        compile_static_library(&root)?;
        generate_bindings(&root)?;
    }

    Ok(())
}

fn compile_static_library(root: &PathBuf) -> Result<(), Box<dyn Error>>  {
    let source = root.join("source");
    let include = root.join("include");
    
    // Collect all .c files in the source directory
    let source_files = fs::read_dir(&source)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // Set up cc build configuration
    cc::Build::new()
        .no_default_flags(true)
        .compiler("/opt/riscv/bin/riscv32-unknown-elf-gcc")
        .flag("-march=rv32i_zicsr_zifencei")
        .flag("-mabi=ilp32")
        .flag("-Os")
        .flag("-Wall")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-nostartfiles")
        .flag("-mno-fdiv")
        .flag("-mstrict-align")
        .flag("-mbranch-cost=10")
        .flag("-Wl,--gc-sections")
        .flag("-ffp-contract=off")
        .flag("-Wl,--defsym,__neorv32_rom_size=16k")
        .flag("-Wl,--defsym,__neorv32_ram_size=8k")
        .flag("-g")
        .flag("-ggdb")
        .flag("-gdwarf-3")
        .flag("-lm")
        .flag("-lc")
        .flag("-lgcc")
        .include(&include)
        .files(&source_files)
        .static_flag(true)
        .warnings(false)
        .compile("neorv32");

    println!("cargo:rerun-if-changed={}", source.display());
    println!("cargo:rerun-if-changed={}", include.display());
    println!("cargo:rustc-link-lib=static=neorv32");

    Ok(())
}

fn generate_bindings(root: &PathBuf) -> Result<(), Box<dyn Error>>  {
    let path = root.join("include/neorv32.h");
    let header = path.to_str().ok_or("Could not get path string")?;

    let bindings = bindgen::Builder::default()
        .clang_args(&["-target", "riscv32"])
        .clang_args(&["-isystem", "/opt/riscv/riscv32-unknown-elf/include"])
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_file("/opt/riscv/riscv32-unknown-elf/include.*")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR")?);

    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
