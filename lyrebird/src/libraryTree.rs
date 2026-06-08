// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use iced::{Length, Padding, Task};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Row, scrollable};
use itertools::Itertools;

use crate::library::{Directory, DirectoryID, MusicLibrary};
use crate::messages::{self, Message};
use crate::track::TrackID;
use crate::widgets::Element;
use crate::widgets::groupBox::GroupBox;
use crate::widgets::listView::{ListItem, ListView};
use crate::widgets::treeView::{TreeItem, TreeView};

pub struct LibraryTree
{
	library: Arc<RwLock<MusicLibrary>>,
	selectedDirectory: DirectoryID,
}

struct DirectoryTree
{
	id: DirectoryID,
	name: String,
	children: Vec<DirectoryTree>,
}

struct TrackEntry
{
	id: TrackID,
	name: String,
}

impl LibraryTree
{
	pub fn new(musicLibrary: Arc<RwLock<MusicLibrary>>) -> Self
	{
		Self
		{
			library: musicLibrary,
			selectedDirectory: DirectoryID::new(0)
		}
	}

	pub fn update(&mut self, message: messages::LibraryTree) -> Task<Message>
	{
		use messages::LibraryTree;

		match message
		{
			LibraryTree::SelectDirectory(directoryID) =>
			{
				self.selectedDirectory = directoryID
			},
		};

		Task::none()
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let library = &self.library.read()
			.expect("Failed to lock library for read");
		let directoryTree = DirectoryTree::from(library.deref());
		let trackList = TrackEntry::forDirectory(self.selectedDirectory, &library);

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Directory Tree",
				scrollable
				(
					TreeView::new(&directoryTree, Some(self.selectedDirectory))
						.onSelect(|nodeID| Message::LibraryTree(messages::LibraryTree::SelectDirectory(nodeID)))
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
			.padding(5.)
			.into()
	}

	pub fn library(&self) -> Arc<RwLock<MusicLibrary>>
	{
		self.library.clone()
	}
}

impl DirectoryTree
{
	/// Convert the entry given by directoryID from the map into a DirectoryTree
	pub fn from_map(map: &BTreeMap<DirectoryID, Directory>, directoryID: DirectoryID) -> Self
	{
		// Find the directory to convert, and map all its subdirectories into DirectoryTree objects
		let directory = &map[&directoryID];
		let children = directory.subdirs()
			.into_iter()
			.map(|&directoryID| Self::from_map(map, directoryID))
			.sorted_by(|a, b| a.name.cmp(&b.name))
			.collect();
		// Calculate the name of this directory
		let path = directory.path();
		let name = if directory.id() == DirectoryID::new(0)
		{
			path.as_os_str()
		}
		else
		{
			path.file_name().unwrap_or_else(|| path.as_os_str())
		}
			.to_string_lossy()
			.to_string();
		// Turn the whole thing into a final DirectoryTree object
		Self
		{
			id: directoryID,
			name,
			children,
		}
	}
}

impl From<&MusicLibrary> for DirectoryTree
{
	/// Convert a directory map into a DirectoryTree structure
	fn from(library: &MusicLibrary) -> Self
	{
		let map = library.directories();
		// If the directory map is empty, synthesise a fake entry
		if map.is_empty()
		{
			Self
			{
				id: DirectoryID::new(0),
				name: library.libraryPath().to_string_lossy().into(),
				children: Vec::new(),
			}
		}
		// Otherwise turn the map into a DirectoryTree normally
		else
		{
			Self::from_map(map, DirectoryID::new(0))
		}
	}
}

impl TreeItem for DirectoryTree
{
	type ItemID = DirectoryID;

	fn nodeID(&self) -> DirectoryID
	{
		self.id
	}

	fn displayText(&self) -> String
	{
		self.name.clone()
	}

	fn children(&self) -> &[Self]
	{
		&self.children
	}
}

impl TrackEntry
{
	pub fn forDirectory(directoryID: DirectoryID, library: &MusicLibrary) -> Vec<Self>
	{
		// Set up tracking for the tracks in the directory selected, and ask the library for the dir
		let mut tracks = Vec::new();
		if !library.isEmpty()
		{
			let directory = library.directoryFor(directoryID);

			// Loop through making each track into a track list item
			for &trackID in directory.contents()
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
