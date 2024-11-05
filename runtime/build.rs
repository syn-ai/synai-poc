fn main() {
	#[cfg(feature = "std")]
	{
		substrate_wasm_builder::WasmBuilder::build_using_defaults();
	}
	// Tell Cargo that if the input files change, to rerun this build script.
	println!("cargo:rerun-if-changed=src/lib.rs");
	println!("cargo:rerun-if-changed=wasm");
}