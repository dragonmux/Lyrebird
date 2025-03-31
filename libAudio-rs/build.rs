// SPDX-License-Identifier: BSD-3-Clause
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use cc::Build;
use meson_next as meson;

fn main()
{
	// Figure out where the build is to go into
	let buildPath = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
	let buildDir = buildPath.to_str().unwrap();

	// Set up the build options to not build the Python bindings and to statically link libAudio
	let mut options = HashMap::new();
	options.insert("bindings", "false");
	options.insert("default_library", "static");
	options.insert("wrap_mode", "forcefallback");

	// Build a Meson configuration for this
	let config = meson::Config::new().options(options);

	// Tell Cargo how/where to find the build results
	emitLinkOptions(buildPath.as_path());
	// Tell Cargo what constitutes a need to re-run
	println!("cargo::rerun-if-changed=build.rs");
	println!("cargo::rerun-if-changed=clib");

	// Ask cc to figure out where the heck the C++ stdlib is and emit linkage to it
	Build::new()
		.cpp(true)
		.std("c++17")
		.file("dummy.cxx")
		.compile("dummy");

	// Ask Meson to run the build
	meson::build("clib", buildDir, config);

	// Figure out which version of OptimFROG to use and put it onto the search path
	let targetOS = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str()
	{
		"linux" => "Linux",
		"macos" => "OSX",
		"windows" => "Win",
		_ => panic!("Unable to build and link with OptimFROG on this OS"),
	};
	let targetArch = match env::var("CARGO_CFG_TARGET_ARCH")
		.unwrap()
		.as_str()
	{
		"x86_64" => "x64",
		_ => panic!("Unable to build and link with OptimFROG on this CPU architecture"),
	};

	let manifestPath = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let depsPath = manifestPath.join("clib/deps");
	let optimFROGPath = depsPath.join(format!("OptimFROG_{targetOS}_{targetArch}_5100/SDK/Library"));
	emitSearchPath(optimFROGPath.clone());

	// Now copy the library object to the build directory
	let _ = fs::copy(
		optimFROGPath.join("libOptimFROG.so.0"),
		buildPath
			.parent()
			.unwrap()
			.join("libOptimFROG.so.0"),
	)
	.unwrap();

	// Copy libmpc's common library so we can make proper use of it (OpenAL has one too and that shadows this)
	let _ = fs::copy(
		buildPath.join("deps/libmpc/common/libcommon.a"),
		buildPath.join("deps/libmpc/common/libmpccommon.a"),
	)
	.unwrap();
}

fn emitLinkOptions(buildDir: &Path)
{
	// Output link libraries needed to make things happy and work
	println!("cargo::rustc-link-lib=Audio");
	println!("cargo::rustc-link-lib=substrate");
	println!("cargo::rustc-link-lib=OpenAL");
	println!("cargo::rustc-link-lib=faac_drm");
	println!("cargo::rustc-link-lib=faad");
	println!("cargo::rustc-link-lib=FLAC");
	println!("cargo::rustc-link-lib=id3tag");
	println!("cargo::rustc-link-lib=mad");
	println!("cargo::rustc-link-lib=mp3lame");
	println!("cargo::rustc-link-lib=vorbisenc");
	println!("cargo::rustc-link-lib=vorbisfile");
	println!("cargo::rustc-link-lib=vorbis");
	println!("cargo::rustc-link-lib=opusenc");
	println!("cargo::rustc-link-lib=opusfile");
	println!("cargo::rustc-link-lib=opus");
	println!("cargo::rustc-link-lib=ogg");
	println!("cargo::rustc-link-lib=mp4v2");
	println!("cargo::rustc-link-lib=mpcdec");
	println!("cargo::rustc-link-lib=mpccommon");
	println!("cargo::rustc-link-lib=wavpack");
	println!("cargo::rustc-link-lib=z");
	println!("cargo::rustc-link-lib=OptimFROG");
	// Output where to find all those moving pieces
	emitSearchPath(buildDir.join("libAudio"));
	emitSearchPath(buildDir.join("deps/substrate/impl"));
	emitSearchPath(buildDir.join("deps/openal-soft"));
	emitSearchPath(buildDir.join("deps/faac"));
	emitSearchPath(buildDir.join("deps/faad2/libfaad"));
	emitSearchPath(buildDir.join("deps/flac-1.4.2/src/libFLAC"));
	emitSearchPath(buildDir.join("deps/libid3tag"));
	emitSearchPath(buildDir.join("deps/libmad"));
	emitSearchPath(buildDir.join("deps/lame-3.100/libmp3lame"));
	emitSearchPath(buildDir.join("deps/libogg-1.3.5/src"));
	emitSearchPath(buildDir.join("deps/libopusenc"));
	emitSearchPath(buildDir.join("deps/libvorbis-1.3.7/lib"));
	emitSearchPath(buildDir.join("deps/opus/src"));
	emitSearchPath(buildDir.join("deps/opusfile"));
	emitSearchPath(buildDir.join("deps/mp4v2"));
	emitSearchPath(buildDir.join("deps/libmpc/common"));
	emitSearchPath(buildDir.join("deps/libmpc/libmpcdec"));
	emitSearchPath(buildDir.join("deps/wavpack"));
	emitSearchPath(buildDir.join("deps/zlib-1.2.13"));
}

fn emitSearchPath(path: PathBuf)
{
	println!("cargo::rustc-link-search=native={}", path.to_str().unwrap());
}
