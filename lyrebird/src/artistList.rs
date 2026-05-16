// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, RwLock};

use iced::{Length, Padding};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Row, scrollable};

use crate::library::MusicLibrary;
use crate::messages::{self, Message};
use crate::track::{ArtistID, TrackID};
use crate::widgets::groupBox::GroupBox;
use crate::widgets::listView::ListItem;
use crate::widgets::{Element, listView::ListView};

pub struct ArtistList
{
	library: Arc<RwLock<MusicLibrary>>,
	selectedArtist: Option<ArtistID>
}

struct ArtistEntry
{
	id: ArtistID,
	name: String,
}

struct TrackEntry
{
	id: TrackID,
	name: String,
}

impl ArtistList
{
	pub fn new(musicLibrary: Arc<RwLock<MusicLibrary>>) -> Self
	{
		Self
		{
			library: musicLibrary,
			selectedArtist: None
		}
	}

	pub fn update(&mut self, message: messages::ArtistList)
	{
		use messages::ArtistList;

		match message
		{
			ArtistList::SelectArtist(artistID) =>
			{
				self.selectedArtist = Some(artistID)
			},
		}
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let library = &self.library.read()
			.expect("Failed to lock library for read");
		let artistList = ArtistEntry::forLibrary(&library);
		let trackList = TrackEntry::forArtist(self.selectedArtist, &library);

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Artists",
				scrollable
				(
					ListView::new(&artistList)
						.width(Length::Fill)
						.onClick(|artistID| Message::ArtistList(messages::ArtistList::SelectArtist(artistID)))
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

impl ArtistEntry
{
	pub fn forLibrary(library: &MusicLibrary) -> Vec<Self>
	{
		// Set up some storage to recieve the aritst list into
		let mut artists = Vec::new();
		for &artistID in library.artists()
		{
			let artist = library.artistFor(artistID);
			artists.push(Self
			{
				id: artistID,
				name: artist.name().into(),
			});
		}

		artists.sort_by(|a, b| a.name.cmp(&b.name));
		artists
	}
}

impl ListItem for ArtistEntry
{
	type ItemID = ArtistID;

	fn nodeID(&self) -> ArtistID
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
	pub fn forArtist(artistID: Option<ArtistID>, library: &MusicLibrary) -> Vec<Self>
	{
		// Set up tracking for the tracks associated with the selected artist
		let mut tracks = Vec::new();
		if let Some(artistID) = artistID
		{
			// Extract the artist and loop through their tracks
			let artist = library.artistFor(artistID);
			for &trackID in artist.tracks()
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
