// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, RwLock};

use iced::{Length, Padding};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Row, scrollable};

use crate::library::MusicLibrary;
use crate::messages::{self, Message};
use crate::track::{AlbumID, TrackID};
use crate::widgets::groupBox::GroupBox;
use crate::widgets::listView::ListItem;
use crate::widgets::{Element, listView::ListView};

pub struct AlbumList
{
	library: Arc<RwLock<MusicLibrary>>,
	selectedAlbum: Option<AlbumID>
}

struct AlbumEntry
{
	id: AlbumID,
	name: String,
}

struct TrackEntry
{
	id: TrackID,
	name: String,
}

impl AlbumList
{
	pub fn new(musicLibrary: Arc<RwLock<MusicLibrary>>) -> Self
	{
		Self
		{
			library: musicLibrary,
			selectedAlbum: None
		}
	}

	pub fn update(&mut self, message: messages::AlbumList)
	{
		use messages::AlbumList;

		match message
		{
			AlbumList::SelectAlbum(albumID) =>
			{
				self.selectedAlbum = Some(albumID)
			},
		}
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let library = &self.library.read()
			.expect("Failed to lock library for read");
		let albumList = AlbumEntry::forLibrary(&library);
		let trackList = TrackEntry::forAlbum(self.selectedAlbum, &library);

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Albums",
				scrollable
				(
					ListView::new(&albumList)
						.width(Length::Fill)
						.onClick(|albumID| Message::AlbumList(messages::AlbumList::SelectAlbum(albumID)))
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
				"Tracks",
				scrollable
				(
					ListView::new(&trackList)
						.width(Length::Fill)
						.onDoubeClick(|nodeID| Message::PlayNow(nodeID))
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
			.padding(Padding
			{
				top: 5.0,
				bottom: 5.0,
				right: 5.0,
				left: 5.0,
			})
			.into()
	}
}

impl AlbumEntry
{
	pub fn forLibrary(library: &MusicLibrary) -> Vec<Self>
	{
		// Set up some storage to recieve the aritst list into
		let mut albums = Vec::new();
		for &albumID in library.albums()
		{
			let album = library.albumFor(albumID);
			albums.push(Self
			{
				id: albumID,
				name: album.name().into(),
			});
		}

		albums.sort_by(|a, b| a.name.cmp(&b.name));
		albums
	}
}

impl ListItem for AlbumEntry
{
	type ItemID = AlbumID;

	fn nodeID(&self) -> AlbumID
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
	pub fn forAlbum(albumID: Option<AlbumID>, library: &MusicLibrary) -> Vec<Self>
	{
		// Set up tracking for the tracks associated with the selected album
		let mut tracks = Vec::new();
		if let Some(albumID) = albumID
		{
			// Extract the album and loop through their tracks
			let album = library.albumFor(albumID);
			for &trackID in album.tracks()
			{
				let track = library.trackFor(trackID);
				tracks.push(Self
				{
					id: track.id(),
					name: track.title().into()
				});
			}
		}
		tracks.sort_by(|a, b| a.name.cmp(&b.name));
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
