// SPDX-License-Identifier: BSD-3-Clause
use std::path::PathBuf;

pub struct Playlist
{
	entries: Vec<PathBuf>,
}

impl Playlist
{
	fn new() -> Self
	{
		Self
		{
			entries: Vec::new(),
		}
	}
}
