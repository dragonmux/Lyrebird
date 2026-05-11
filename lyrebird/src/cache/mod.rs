// SPDX-License-Identifier: BSD-3-Clause

use std::{fs::File, path::Path};

use color_eyre::eyre::Result;

use crate::cache::library::MusicLibrary;

pub mod library;

pub fn loadLibrary(cacheFile: &Path) -> Result<MusicLibrary>
{
	let cache = File::open(cacheFile)?;
	Ok(serde_json::from_reader(cache)?)
}
