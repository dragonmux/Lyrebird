// SPDX-License-Identifier: BSD-3-Clause
use serde::{Deserialize, Serialize};

use crate::window::Operation;
use crate::playlist::Playlist;

#[derive(Serialize, Deserialize)]
pub struct Playlists
{
	nowPlaying: Playlist,
	#[expect(clippy::struct_field_names, reason = "naming is hard, okay")]
	playlists: Vec<Playlist>,
	#[serde(skip)]
	activeSide: Side,
}

#[derive(Clone, Copy, Default)]
enum Side
{
	#[default]
	Playlists,
	PlaylistContents,
}

impl Playlists
{
	pub fn new() -> Self
	{
		Self
		{
			nowPlaying: Playlist::new("Now Playing".into()),
			playlists: Vec::new(),
			activeSide: Side::Playlists,
		}
	}

	pub fn nowPlaying(&mut self) -> &mut Playlist
		{ &mut self.nowPlaying }

	const fn moveLeft(&mut self)
		{ self.activeSide = Side::Playlists; }

	const fn moveRight(&mut self)
		{ self.activeSide = Side::PlaylistContents; }

	fn moveUp(&mut self)
	{
		match self.activeSide
		{
			Side::Playlists =>
			{
			}
			Side::PlaylistContents =>
			{
			}
		}
	}

	fn moveDown(&mut self)
	{
		match self.activeSide
		{
			Side::Playlists =>
			{
			}
			Side::PlaylistContents =>
			{
			}
		}
	}

	fn makeSelection(&mut self) -> Operation
	{
		match self.activeSide
		{
			Side::Playlists => Operation::None,
			Side::PlaylistContents =>
			{
				Operation::None
			}
		}
	}
}
