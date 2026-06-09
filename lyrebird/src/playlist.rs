// SPDX-License-Identifier: BSD-3-Clause

use crate::track::TrackID;

pub struct Playlist
{
	name: String,
	entries: Vec<TrackID>,
	currentEntry: usize,
}

impl Playlist
{
	pub fn new<S: Into<String>>(name: S) -> Self
	{
		Self
		{
			name: name.into(),
			entries: Vec::new(),
			currentEntry: 0,
		}
	}

	pub fn name(&self) -> &str
	{
		&self.name
	}

	pub fn add(&mut self, trackID: TrackID)
	{
		self.entries.push(trackID);
	}

	pub fn replaceWith(&mut self, trackID: TrackID)
	{
		self.entries.clear();
		self.currentEntry = 0;
		self.add(trackID);
	}

	pub fn entry(&self, index: usize) -> TrackID
	{
		self.entries[index]
	}

	pub fn nextEntry(&mut self, index: usize)
	{
		self.currentEntry = index;
	}

	pub fn currentEntry(&self) -> usize
	{
		self.currentEntry
	}

	pub fn next(&mut self) -> Option<TrackID>
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
		Some(self.entries[self.currentEntry])
	}
}
