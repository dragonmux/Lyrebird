// SPDX-License-Identifier: BSD-3-Clause

use crate::playlist::Playlist;

pub struct Playlists
{
	nowPlaying: Playlist,
	#[expect(clippy::struct_field_names, reason = "naming is hard, okay")]
	playlists: Vec<Playlist>,
}

impl Playlists
{
	pub fn new() -> Self
	{
		Self
		{
			nowPlaying: Playlist::new("Now Playing".into()),
			playlists: Vec::new(),
		}
	}

	pub fn nowPlaying(&mut self) -> &mut Playlist
		{ &mut self.nowPlaying }
}
