// SPDX-License-Identifier: BSD-3-Clause
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use color_eyre::eyre::{self, Result};
use iced::{Length, Padding};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Container, Row, scrollable};

use crate::library::MusicLibrary;
use crate::messages::Message;
use crate::theme::Theme;
use crate::widgets::Element;
use crate::widgets::groupBox::GroupBox;
use crate::window::Operation;

pub struct LibraryTree
{
	library: Arc<RwLock<MusicLibrary>>,
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
		let directoryTree: Option<Container<'a, Message, Theme>> = None;
		let trackList: Option<Container<'a, Message, Theme>> = None;

		let layout = Row::with_children
		([
			GroupBox::new
			(
				"Directory Tree",
				scrollable(directoryTree)
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

	// pub fn writeCache(&self) -> Result<()>
	// {
	// 	self.library.read()
	// 		.map_err
	// 		(
	// 			|error|
	// 				eyre::eyre!("While writing library cache: {}", error.to_string())
	// 		)?
	// 		.writeCache()
	// }

	pub fn isDiscovering(&self) -> bool
	{
		self.library.read().expect("Library lock in bad state").isDiscovering()
	}

	pub async fn maybeJoinDiscovery(&self) -> Result<()>
	{
		MusicLibrary::maybeJoinDiscoveryThread(&self.library).await
	}

	/// If the currently sellected side is the directory listing, switch to that directory's file listing
	/// otherwise, if it's the file listing, figure out which one and make a `SongState` for it
	fn makeSelection(&mut self) -> Option<PathBuf>
	{
		None
	}

	fn playSelection(&mut self) -> Operation
	{
		let selection = self.makeSelection();
		match selection
		{
			Some(selection) => Operation::Play(selection),
			None => Operation::None,
		}
	}
}
