// SPDX-License-Identifier: BSD-3-Clause
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Playlist
{
	name: String,
	entries: Vec<PathBuf>,
}

impl Playlist
{
	pub fn new(name: String) -> Self
	{
		Self
		{
			name,
			entries: Vec::new(),
		}
	}

	pub fn name(&self) -> &str
	{
		return self.name.as_str()
	}
}
