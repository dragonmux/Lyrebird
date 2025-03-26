// SPDX-License-Identifier: BSD-3-Clause
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use meson_next as meson;

fn main()
{
	// Figure out where the build is to go into
	let buildPath = PathBuf::from(env::var("OUT_DIR").unwrap())
		.join("build");
	let buildDir = buildPath.to_str().unwrap();

	// Set up the build options to not build the Python bindings and to statically link libAudio
	let mut options = HashMap::new();
	options.insert("bindings", "false");
	options.insert("default_library", "static");
	options.insert("force_fallback_for", "substrate,mp4v2");

	// Build a Meson configuration for this
	let config = meson::Config::new().options(options);

	// Tell Cargo how/where to find the build results
	println!("cargo::rustc-link-lib=Audio");
	println!("cargo::rustc-link-search=native={}", buildPath.join("libAudio").to_str().unwrap());
	// Tell Cargo what constitutes a need to re-run
	println!("cargo::rerun-if-changed=build.rs");
	println!("cargo::rerun-if-changed=clib");

	// Ask Meson to run the build
	meson::build("clib", buildDir, config);
}
