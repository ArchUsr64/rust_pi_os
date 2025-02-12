use std::env;
use std::process::Command;

const LINKER_SCRIPT: &'static str = "linker_script.ld";
const CC_TOOLCHAIN: &'static str = "aarch64-linux-gnu-";
const OUT_BIN: &'static str = "out.bin";

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	dbg!(env::vars());
	let bin_name = env::var("CARGO_PKG_NAME").unwrap();

	println!("cargo::rerun-if-changed={LINKER_SCRIPT}");
	println!("cargo::rustc-link-arg=--script={LINKER_SCRIPT}");

	// TODO: move this to build.rs for another package in the project with this
	// package as its dependency
	// [BROKED]
	dbg!(Command::new(&format!("{CC_TOOLCHAIN}objcopy"))
		.args(&["-O", "binary"])
		.arg(&format!("{out_dir}/{bin_name}"))
		.arg(&format!("{out_dir}/{OUT_BIN}")))
	.status()
	.unwrap();
}
