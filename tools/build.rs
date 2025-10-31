use std::{path::Path, process::Command};

fn main() {
	println!("cargo:rerun-if-changed=tools/boot.s");

	let out_dir = "tools";
	let obj_path = Path::new(&out_dir).join("boot.o");

	let status = Command::new("nasm")
		.args([
			"-felf32",
			"tools/boot.s",
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
