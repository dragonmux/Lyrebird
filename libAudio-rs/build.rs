// SPDX-License-Identifier: BSD-3-Clause
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use meson_next as meson;
use cc::Build;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TargetOS
{
	Linux,
	Windows,
	MacOS,
	Unknown
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TargetArch
{
	AMD64,
	Unknown
}

struct PathResolver
{
	searchPath: Option<Vec<PathBuf>>,
}

impl PathResolver
{
	/// Construct a new PathResolver for $PATH
	pub fn new() -> Self
	{
		// Extract $PATH and turn it into an Option<String>
		let envPath = env::var("PATH").ok();
		// Now have it split into a search path and construct the resolver
		Self
		{
			searchPath: envPath
				.as_deref()
				.map(OsStr::new)
				.map(|value| env::split_paths(value).collect())
		}
	}

	/// Find a tool on $PATH if possible
	pub fn find(&self, tool: &str) -> Option<PathBuf>
	{
		let check_executable = |toolPath: PathBuf| -> Option<PathBuf>
		{
			toolPath.exists().then_some(toolPath)
		};

		self.searchPath
			.as_ref()
			.and_then(|searchPath|
			{
				searchPath
					.iter()
					.filter_map(|path| check_executable(path.join(tool)))
					.nth(0)
			})
	}
}

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
	options.insert("wrap_mode", "forcefallback");

	// Ask CC for a Build object to use and figure out what toolchain is there
	let mut cxxBuild = Build::new();
	let cxx = cxxBuild.get_compiler();

	// If it's a GCC-like compiler
	if cxx.is_like_gnu() {
		// If possible, let's try and find Clang and use that
		let pathResolver = PathResolver::new();
		let clang = pathResolver.find("clang");
		let clangpp = pathResolver.find("clang++");

		// If we found clang(++), set  CC to use it
		if let Some(clang) = clang && let Some(clangpp) = clangpp {
			unsafe
			{
				env::set_var("CC", clang);
				env::set_var("CXX", clangpp);
			}
		} else {
			// Turn LTO off, at least that way the build'll work
			options.insert("b_lto", "false");
		}
	}

	// Build a Meson configuration for this
	let config = meson::Config::new().options(options);

	// Convert the target OS and architecture values into enum values
	let targetOS = TargetOS::from(env::var("CARGO_CFG_TARGET_OS").unwrap().as_str());
	let targetArch = TargetArch::from(env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str());

	// Tell Cargo how/where to find the build results
	emitLinkOptions(buildPath.as_path(), targetOS, targetArch);
	// Tell Cargo what constitutes a need to re-run
	println!("cargo::rerun-if-changed=build.rs");
	println!("cargo::rerun-if-changed=clib");

	// Ask cc to figure out where the heck the C++ stdlib is and emit linkage to it
	cxxBuild.cpp(true)
		.std("c++17")
		.file("dummy.cxx")
		.compile("dummy");

	// Ask Meson to run the build
	meson::build("clib", buildDir, config);

	// Figure out which version of OptimFROG to use (if any) and put it onto the search path
	if targetOS != TargetOS::Unknown && targetArch != TargetArch::Unknown
	{
		let manifestPath = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
		let depsPath = manifestPath.join("clib/deps");
		let optimFROGPath = depsPath.join(format!("OptimFROG_{targetOS}_{targetArch}_5100/SDK/Library"));
		emitSearchPath(optimFROGPath.clone());

		// Now copy the library object to the build directory
		let _ = match targetOS
		{
			TargetOS::Linux => fs::copy
			(
				optimFROGPath.join("libOptimFROG.so.0"),
				buildPath.parent().unwrap().join("libOptimFROG.so.0"),
			),
			TargetOS::MacOS => fs::copy
			(
				optimFROGPath.join("libOptimFROG.0.dylib"),
				buildPath.parent().unwrap().join("libOptimFROG.0.dylib"),
			),
			_ => Ok(0),
		}.unwrap();
	}

	// Copy libmpc's common library so we can make proper use of it (OpenAL has one too and that shadows this)
	let _ = fs::copy
	(
		buildPath.join("deps/libmpc/common/libcommon.a"),
		buildPath.join("deps/libmpc/common/libmpccommon.a"),
	).unwrap();
}

fn emitLinkOptions(buildDir: &Path, targetOS: TargetOS, targetArch: TargetArch)
{
	// Output link libraries needed to make things happy and work
	println!("cargo::rustc-link-lib=static=Audio");
	println!("cargo::rustc-link-lib=static=substrate");
	match targetOS
	{
		TargetOS::Windows => println!("cargo::rustc-link-lib=static=OpenAL32"),
		TargetOS::MacOS =>
		{
			println!("cargo::rustc-link-lib=static=openal");
			// For now just assume that all frameworks we might depend on are present and required -
			// doing anything else is a giant pain and headache! >_<
			println!("cargo::rustc-link-lib=framework=CoreFoundation");
			println!("cargo::rustc-link-lib=framework=AudioToolbox");
			println!("cargo::rustc-link-lib=framework=CoreAudio");
		},
		_ => println!("cargo::rustc-link-lib=static=openal"),
	}
	println!("cargo::rustc-link-lib=static=fmt");
	println!("cargo::rustc-link-lib=static=faac_drm");
	println!("cargo::rustc-link-lib=static=faad");
	println!("cargo::rustc-link-lib=static=FLAC");
	println!("cargo::rustc-link-lib=static=id3tag");
	println!("cargo::rustc-link-lib=static=mad");
	println!("cargo::rustc-link-lib=static=mp3lame");
	println!("cargo::rustc-link-lib=static=vorbisenc");
	println!("cargo::rustc-link-lib=static=vorbisfile");
	println!("cargo::rustc-link-lib=static=vorbis");
	println!("cargo::rustc-link-lib=static=opusenc");
	println!("cargo::rustc-link-lib=static=opusfile");
	println!("cargo::rustc-link-lib=static=opus");
	println!("cargo::rustc-link-lib=static=ogg");
	println!("cargo::rustc-link-lib=static=mp4v2");
	println!("cargo::rustc-link-lib=static=mpcdec");
	println!("cargo::rustc-link-lib=static=mpccommon");
	println!("cargo::rustc-link-lib=static=wavpack");
	println!("cargo::rustc-link-lib=static=z");
	// Only include OptimFROG if it's an OS and architecture it can be used on
	if targetOS != TargetOS::Unknown && targetArch != TargetArch::Unknown
	{
		println!("cargo::rustc-link-lib=OptimFROG");
	}
	// Output where to find all those moving pieces
	emitSearchPath(buildDir.join("libAudio"));
	emitSearchPath(buildDir.join("deps/substrate/impl"));
	emitSearchPath(buildDir.join("deps/openal"));
	emitSearchPath(buildDir.join("deps/fmt-11.1.1"));
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
	emitSearchPath(buildDir.join("deps/zlib-1.3.1"));
}

#[expect(clippy::needless_pass_by_value, reason = "this should take &Path, but it's easier this way")]
fn emitSearchPath(path: PathBuf)
{
	println!("cargo::rustc-link-search=native={}", path.to_str().unwrap());
}

impl From<&str> for TargetOS
{
	fn from(value: &str) -> Self
	{
		match value
		{
			"linux" => TargetOS::Linux,
			"macos" => TargetOS::MacOS,
			"windows" => TargetOS::Windows,
			_ => TargetOS::Unknown,
		}
	}
}

impl Into<&'static str> for &TargetOS
{
	fn into(self) -> &'static str
	{
		match self
		{
			TargetOS::Linux => "Linux",
			TargetOS::MacOS => "OSX",
			TargetOS::Windows => "Win",
			_ => panic!("Unable to build and link with OptimFROG on this OS"),
		}
	}
}

impl Display for TargetOS
{
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		fmt.write_str(self.into())
	}
}

impl From<&str> for TargetArch
{
	fn from(value: &str) -> Self
	{
		match value
		{
			"x86_64" => TargetArch::AMD64,
			_ => TargetArch::Unknown,
		}
	}
}

impl Into<&'static str> for &TargetArch
{
	fn into(self) -> &'static str
	{
		match self
		{
			TargetArch::AMD64 => "x64",
			_ => panic!("Unable to build and link with OptimFROG on this CPU architecture"),
		}
	}
}

impl Display for TargetArch
{
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		fmt.write_str(self.into())
	}
}
