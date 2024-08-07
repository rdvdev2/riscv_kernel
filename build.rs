fn main() {
    cc::Build::new()
        .file("src/entry.S")
        .asm_flag("-march=rv64gc")
        .compile("entry");

    println!("cargo::rustc-link-arg=-Tsrc/linker.ld");
}
