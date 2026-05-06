use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set by cargo"));
    let obj_path = out_dir.join("boot.o");

    let status = Command::new("nasm")
        .args([
            "-felf32",
            "tools/build/boot.s",
            "-o",
            obj_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run nasm");

    if !status.success() {
        panic!("nasm failed");
    }

    println!("cargo:rustc-link-arg={}", obj_path.display());
    println!("cargo:rerun-if-changed=tools/build/boot.s");
}
