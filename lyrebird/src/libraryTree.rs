// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use iced::{Length, Padding};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Container, Row, scrollable};

use crate::library::{Directory, DirectoryID, MusicLibrary};
use crate::messages::{self, Message};
use crate::theme::Theme;
use crate::widgets::Element;
use crate::widgets::groupBox::GroupBox;
use crate::widgets::treeView::{TreeItem, TreeView};

pub struct LibraryTree
{
	library: Arc<RwLock<MusicLibrary>>,
}

struct DirectoryTree
{
	id: DirectoryID,
	name: String,
	children: Vec<DirectoryTree>,
}

fn defaultTreeIcon() -> String
{
	"╰ ".to_string()
}

fn defaultLeafIcon() -> String
{
	"├ ".to_string()
}

impl LibraryTree
{
	pub fn new(musicLibrary: Arc<RwLock<MusicLibrary>>) -> Self
	{
		Self
		{
			library: musicLibrary,
		}
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let library = &self.library.read()
			.expect("Failed to lock library for read");
		let directoryTree = DirectoryTree::from(library.directories());
		let trackList: Option<Container<'a, Message, Theme>> = None;

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Directory Tree",
				scrollable
				(
					TreeView::new(&directoryTree)
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
				scrollable(trackList)
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

impl From<&BTreeMap<DirectoryID, Directory>> for DirectoryTree
{
	/// Convert a directory map into a DirectoryTree structure
	fn from(map: &BTreeMap<DirectoryID, Directory>) -> Self
	{
		Self::from_map(map, DirectoryID::new(0))
	}
}

impl TreeItem<Message> for DirectoryTree
{
	fn displayText(&self) -> String
	{
		self.name.clone()
	}

	fn children(&self) -> &[Self]
	{
		&self.children
	}

	fn selectMessage(&self) -> Message
	{
		Message::LibraryTree(messages::LibraryTree::SelectDirectory(self.id))
	}
}
