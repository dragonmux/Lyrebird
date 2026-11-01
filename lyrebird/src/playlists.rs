// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::sync::{Arc, RwLock};

use iced::{Length, Padding};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Row, scrollable};

use crate::library::MusicLibrary;
use crate::messages::Message;
use crate::playlist::Playlist;
use crate::track::TrackID;
use crate::widgets::listView::{ListItem, ListView};
use crate::widgets::{Element, groupBox::GroupBox};

pub struct Playlists
{
	library: Arc<RwLock<MusicLibrary>>,
	lists: BTreeMap<PlaylistID, Playlist>,
	selectedPlaylist: PlaylistID,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlaylistID(u64);

struct ListEntry
{
	id: PlaylistID,
	name: String,
}

struct TrackEntry
{
	id: TrackID,
	name: String,
}

impl Playlists
{
	pub fn new(musicLibrary: Arc<RwLock<MusicLibrary>>) -> Self
	{
		let mut result = Self
		{
			library: musicLibrary,
			lists: BTreeMap::new(),
			// Default "Now Playing" to being selected
			selectedPlaylist: PlaylistID::nowPlaying(),
		};

		result.lists.insert(PlaylistID::nowPlaying(), Playlist::new("Now Playing"));

		result
	}

	pub fn nowPlaying(&mut self) -> &mut Playlist
		{ self.lists.get_mut(&PlaylistID::nowPlaying()).expect("Now playing playlist is always valid") }

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let library = &self.library.read()
			.expect("Failed to lock library for read");
		let playlistList = ListEntry::forPlaylists(&self.lists);
		let selectedPlaylist = &self.lists[&self.selectedPlaylist];
		let playlistTracks = TrackEntry::forPlaylist(selectedPlaylist, &library);

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Playlists",
				scrollable
				(
					ListView::new(&playlistList)
						.width(Length::Fill)
				)
					.width(Length::Fill)
					.height(Length::Fill)
					.spacing(5.0)
					.direction(Direction::Vertical(Scrollbar::default()))
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
				.into(),
			GroupBox::new
			(
				selectedPlaylist.name(),
				scrollable
				(
					ListView::new(&playlistTracks)
						.width(Length::Fill)
				)
					.width(Length::Fill)
					.height(Length::Fill)
					.spacing(5.0)
					.direction(Direction::Vertical(Scrollbar::default()))
			)
				.width(Length::FillPortion(2))
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
				.into(),
		]);

		layout
			.width(Length::Fill)
			.height(Length::Fill)
			.spacing(5.0)
			.padding(5.0)
			.into()
	}
}

impl PlaylistID
{
	/// Constant special playlist ID for the Now Playing playlist
	pub const fn nowPlaying() -> Self
	{
		Self(0)
	}
}

impl Display for PlaylistID
{
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		self.0.fmt(fmt)
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

impl TrackEntry
{
	pub fn forPlaylist(playlist: &Playlist, library: &MusicLibrary) -> Vec<Self>
	{
		let mut tracks = Vec::new();
		for &trackID in playlist.tracks()
		{
			let track = library.trackFor(trackID);
			tracks.push(Self
			{
				id: trackID,
				name: track.title().into()
			});
		}

		tracks
	}
}

impl ListItem for TrackEntry
{
	type ItemID = TrackID;

	fn nodeID(&self) -> TrackID
	{
		self.id
	}

	fn displayText(&self) -> String
	{
		self.name.clone()
	}
}
