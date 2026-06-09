// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;

use crate::playlist::Playlist;

pub struct Playlists
{
	lists: BTreeMap<PlaylistID, Playlist>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PlaylistID(u64);

impl Playlists
{
	pub fn new() -> Self
	{
		let mut result = Self
		{
			lists: BTreeMap::new(),
		};

		result.lists.insert(PlaylistID(0), Playlist::new("Now Playing"));

		result
	}

	pub fn nowPlaying(&mut self) -> &mut Playlist
		{ self.lists.get_mut(&PlaylistID(0)).expect("Now playing playlist is always valid") }
}
