// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};

use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist
{
	name: String,
	entries: Vec<PathBuf>,

	#[serde(skip)]
	currentEntry: usize,
}

impl Playlist
{
	pub fn new(name: String) -> Self
	{
		Self
		{
			name,
			entries: Vec::new(),
			currentEntry: 0,
		}
	}

	pub fn name(&self) -> &str
	{
		self.name.as_str()
	}

	pub fn add(&mut self, fileName: &Path)
	{
		self.entries.push(fileName.to_path_buf());
	}

	pub fn replaceWith(&mut self, fileName: &Path)
	{
		self.entries.clear();
		self.currentEntry = 0;
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

	pub fn entry(&self, index: usize) -> &Path
	{
		self.entries[index].as_path()
	}

	pub fn nextEntry(&mut self, index: usize)
	{
		self.currentEntry = index;
	}

	pub fn currentEntry(&self) -> usize
	{
		self.currentEntry
	}

	pub fn next(&mut self) -> Option<PathBuf>
	{
		// If there are no entries in this playlist, we're done.. nothing comes next
		if self.entries.is_empty()
		{
			return None;
		}
		// If there are entries, figure out how many vs currentEntry
		let count = self.entries.len();
		if self.currentEntry < count
		{
			// Increment the current entry counter if there's room to
			self.currentEntry += 1;
		}
		// Now check if we're done
		if self.currentEntry >= count
		{
			return None;
		}
		// Finally, we get to the happy path - give them what they want, a new entry from the playlist!
		Some(self.entries[self.currentEntry].clone())
	}
}
