// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;

use crate::track::TrackID;

pub struct Playlist
{
	name: String,
	entries: BTreeMap<PlaylistEntryID, TrackID>,
	nextEntryID: PlaylistEntryID,
	currentEntry: PlaylistEntryID,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlaylistEntryID(usize);

impl Playlist
{
	pub fn new<S: Into<String>>(name: S) -> Self
	{
		Self
		{
			name: name.into(),
			entries: BTreeMap::new(),
			nextEntryID: PlaylistEntryID::default(),
			currentEntry: PlaylistEntryID::default(),
		}
	}

	pub fn name(&self) -> &str
	{
		&self.name
	}

	pub fn add(&mut self, trackID: TrackID)
	{
		let entryID = self.nextEntryID;
		self.nextEntryID.0 += 1;
		self.entries.insert(entryID, trackID);
	}

	pub fn replaceWith(&mut self, trackID: TrackID)
	{
		self.entries.clear();
		self.nextEntryID = PlaylistEntryID::default();
		self.currentEntry = PlaylistEntryID::default();
		self.add(trackID);
	}

	pub fn entry(&self, index: PlaylistEntryID) -> Option<TrackID>
	{
		self.entries.get(&index).copied()
	}

	pub fn nextEntry(&mut self, index: PlaylistEntryID)
	{
		self.currentEntry = index;
	}

	pub fn currentEntry(&self) -> PlaylistEntryID
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
		if self.currentEntry.0 < count
		{
			// Increment the current entry counter if there's room to
			self.currentEntry.0 += 1;
		}
		// Now check if we're done
		if self.currentEntry.0 >= count
		{
			return None;
		}
		// Finally, we get to the happy path - give them what they want, a new entry from the playlist!
		Some(self.entries[&self.currentEntry])
	}
}
