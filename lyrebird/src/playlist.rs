// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};

use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

	pub fn add(&mut self, fileName: &Path)
	{
		self.entries.push(fileName.to_path_buf());
	}

	pub fn replaceWith(&mut self, fileName: &Path)
	{
		self.entries.clear();
		self.add(fileName);
	}

	pub fn contents(&self) -> impl Iterator<Item = ListItem>
	{
		self.entries
			.iter()
			.map
			(
				|fileName| ListItem::new(fileName.to_string_lossy())
			)
	}
}
