// SPDX-License-Identifier: BSD-3-Clause

use std::{collections::BTreeMap, ops::AddAssign};
use std::ops::Add;

use crate::{messages::PlayPosition, track::TrackID};

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

	pub fn add(&mut self, trackID: TrackID, playPosition: PlayPosition)
	{
		let entryID = match playPosition
		{
			PlayPosition::Now => {
				// If the tune is to be played now, replace the playlist with it
				self.clear();
				self.nextEntryID.fetch_inc()
			}
			PlayPosition::Next => {
				// If it should go after the curently playing track, figure out what that is,
				// move everything else down, and then add the new track
				let entryID = self.currentEntry.map_or_default(|entryID| entryID + 1);
				// Extract the list of tracks that follow the new entry ID
				let tracks = self.entries.split_off(&entryID);
				// Add the new track and then re-add all the following tracks
				self.entries.insert(entryID, trackID);
				for (offset, track) in tracks.into_values().enumerate()
				{
					self.entries.insert(entryID + offset + 1, track);
				}
				entryID
			}
			PlayPosition::Last => {
				// If it should go on the end, figure out the ID of the end entry
				self.nextEntryID.fetch_inc()
			}
		};
		self.entries.insert(entryID, trackID);
	}

	pub fn clear(&mut self)
	{
		self.entries.clear();
		self.nextEntryID = PlaylistEntryID::default();
		self.currentEntry = None;
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

impl PlaylistEntryID
{
	pub fn fetch_inc(&mut self) -> Self
	{
		let result = *self;
		self.0 += 1;
		result
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
