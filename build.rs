fn main() {
    cc::Build::new()
        .file("src/entry.S")
        .asm_flag("-march=rv64gc")
        .compile("entry");
    println!("cargo::rerun-if-changed=src/entry.S");

    println!("cargo::rustc-link-arg=-Tsrc/linker.ld");
    println!("cargo::rerun-if-changed=src/linker.ld");
}
