// SPDX-License-Identifier: BSD-3-Clause
use serde::{Deserialize, Serialize};

use crate::{playback::Song, playlist::Playlist};

#[derive(Serialize, Deserialize)]
pub struct Playlists
{
	nowPlaying: Playlist,
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
}
