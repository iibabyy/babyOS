use std::{path::Path, process::Command};

fn main() {
	println!("cargo:rerun-if-changed=tools/build/boot.s");

	let out_dir = "tools/build";
	let obj_path = Path::new(&out_dir).join("boot.o");

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
}
