// SPDX-License-Identifier: BSD-3-Clause
use crate::playlist::Playlist;

pub struct Playlists
{
	playlists: Vec<Playlist>
}

impl Playlists
{
	pub fn new() -> Self
	{
		Self
		{
			playlists: Vec::new(),
		}
	}
}
