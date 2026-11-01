// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;
use std::ops::{Add, AddAssign};

use crate::track::TrackID;

pub struct Playlist
{
	name: String,
	entries: BTreeMap<PlaylistEntryID, TrackID>,
	nextEntryID: PlaylistEntryID,
	currentEntry: Option<PlaylistEntryID>,
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
			currentEntry: None,
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

	pub fn clear(&mut self)
	{
		self.entries.clear();
		self.nextEntryID = PlaylistEntryID::default();
		self.currentEntry = None;
	}

	pub fn entry(&self, index: PlaylistEntryID) -> Option<TrackID>
	{
		self.entries.get(&index).copied()
	}

	pub fn currentEntry(&self) -> PlaylistEntryID
	{
		self.currentEntry.unwrap_or_default()
	}

	pub fn tracks(&self) -> impl Iterator<Item = &TrackID>
	{
		self.entries.values()
	}

	pub fn next(&mut self) -> Option<TrackID>
	{
		// If there are no entries in this playlist, we're done.. nothing comes next
		if self.entries.is_empty()
		{
			return None;
		}
		// Try finding the next entry, if we're not already at the end
		loop
		{
			let currentEntry = match self.currentEntry
			{
				// If the current entry is None, pick the first entry in the list
				None => PlaylistEntryID::default(),
				// Otherwise if there are entries, try and find the next one
				Some(currentEntry) => if currentEntry < self.nextEntryID
				{
					// Increment the current entry counter if there's room to
					currentEntry + 1
				}
				else
				{
					currentEntry
				}
			};
			self.currentEntry = Some(currentEntry);
			// If this landed on a valid entry, then we want to emit it, and be done
			if self.entries.contains_key(&currentEntry)
			{
				return Some(self.entries[&currentEntry]);
			}
			// Now check if we're done and there's nothing left to give
			else if currentEntry >= self.nextEntryID
			{
				return None;
			}
		}
	}
}

impl Add<usize> for PlaylistEntryID
{
	type Output = Self;

	fn add(self, rhs: usize) -> Self
	{
		PlaylistEntryID(self.0 + rhs)
	}
}

impl AddAssign<usize> for PlaylistEntryID
{
	fn add_assign(&mut self, rhs: usize)
	{
		self.0 += rhs;
	}
}
