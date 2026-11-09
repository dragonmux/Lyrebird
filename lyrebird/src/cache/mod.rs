// SPDX-License-Identifier: BSD-3-Clause

use std::{fs::{File, create_dir_all}, path::Path};

use color_eyre::eyre::{OptionExt, Result};

pub use self::library::MusicLibrary;

mod library;

pub fn loadLibrary(cacheFile: &Path) -> Result<MusicLibrary>
{
	let cache = File::open(cacheFile)?;
	Ok(serde_json::from_reader(cache)?)
}

pub fn storeLibrary(library: MusicLibrary, cacheFile: &Path) -> Result<()>
{
	// Make sure the path to the cache file exists
	create_dir_all
	(
		cacheFile
			.parent()
			.ok_or_eyre("Failed to extract the path to the music library cache file")?
	)?;
	// Open the cache file for writing and serialise the cache into it
	let cache = File::create(cacheFile)?;
	Ok(serde_json::to_writer(cache, &library)?)
}
