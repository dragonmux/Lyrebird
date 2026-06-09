// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;

use iced::{Length, Padding};
use iced_widget::{Row, scrollable};

use crate::messages::Message;
use crate::playlist::Playlist;
use crate::widgets::listView::{ListItem, ListView};
use crate::widgets::{Element, groupBox::GroupBox};

pub struct Playlists
{
	lists: BTreeMap<PlaylistID, Playlist>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PlaylistID(u64);

struct ListEntry
{
	id: PlaylistID,
	name: String,
}

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

	pub fn view<'a>(&self) -> Element<'a, Message>
	{
		let playlistList = ListEntry::forPlaylists(&self.lists);

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Playlists",
				scrollable
				(
					ListView::new(&playlistList)
				)
			)
				.width(Length::FillPortion(1))
				.height(Length::Fill)
				.titleMargin(5.0)
				.titlePadding(5.0)
				.padding(Padding
				{
					top: 2.0,
					bottom: 2.0,
					right: 2.0,
					left: 5.0,
				})
				.into()
		]);

		layout
			.width(Length::Fill)
			.height(Length::Fill)
			.spacing(5.0)
			.padding(5.0)
			.into()
	}
}

impl ListEntry
{
	pub fn forPlaylists(lists: &BTreeMap<PlaylistID, Playlist>) -> Vec<Self>
	{
		let mut playlists = Vec::new();
		for (&playlistID, playlist) in lists
		{
			playlists.push(Self
			{
				id: playlistID,
				name: playlist.name().into(),
			});
		}

		playlists
	}
}

impl ListItem for ListEntry
{
	type ItemID = PlaylistID;

	fn nodeID(&self) -> PlaylistID
	{
		self.id
	}

	fn displayText(&self) -> String
	{
		self.name.clone()
	}
}
