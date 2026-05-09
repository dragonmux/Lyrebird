// SPDX-License-Identifier: BSD-3-Clause
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use color_eyre::eyre::{self, Result};
use iced::{Length, Padding};
use iced_widget::{Row, container, text};

use crate::library::MusicLibrary;
use crate::messages::Message;
use crate::theme::container::roundedBox;
use crate::widgets::Element;
use crate::window::Operation;

pub struct LibraryTree
{
	activeSide: Side,
	library: Arc<RwLock<MusicLibrary>>,
}

#[derive(Clone, Copy)]
enum Side
{
	DirectoryTree,
	Files,
}

impl LibraryTree
{
	pub fn new(musicLibrary: Arc<RwLock<MusicLibrary>>) -> Self
	{
		Self
		{
			activeSide: Side::DirectoryTree,
			library: musicLibrary,
		}
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let layout = Row::with_children
		([
			container
			(
				text("Directory Tree")
			)
				.style(roundedBox)
				.width(Length::FillPortion(1))
				.height(Length::Fill)
				.padding(Padding {
					top: 2.0,
					bottom: 2.0,
					right: 5.0,
					left: 5.0,
				})
				.into(),
			container
			(
				text("Files")
			)
				.style(roundedBox)
				.width(Length::FillPortion(2))
				.height(Length::Fill)
				.padding(Padding {
					top: 2.0,
					bottom: 2.0,
					right: 5.0,
					left: 5.0,
				})
				.into(),
		]);

		layout
			.width(Length::Fill)
			.height(Length::Fill)
			.spacing(5.0)
			.padding(Padding {
				top: 5.0,
				bottom: 5.0,
				right: 5.0,
				left: 5.0,
			})
			.into()
	}

	pub fn writeCache(&self) -> Result<()>
	{
		self.library.read()
			.map_err
			(
				|error|
					eyre::eyre!("While writing library cache: {}", error.to_string())
			)?
			.writeCache()
	}

	pub fn isDiscovering(&self) -> bool
	{
		self.library.read().expect("Library lock in bad state").isDiscovering()
	}

	pub async fn maybeJoinDiscovery(&self) -> Result<()>
	{
		MusicLibrary::maybeJoinDiscoveryThread(&self.library).await
	}

	const fn moveLeft(&mut self)
		{ self.activeSide = Side::DirectoryTree; }

	const fn moveRight(&mut self)
		{ self.activeSide = Side::Files; }

	fn moveUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	fn moveDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	fn movePageUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	fn movePageDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	/// If the currently sellected side is the directory listing, switch to that directory's file listing
	/// otherwise, if it's the file listing, figure out which one and make a `SongState` for it
	fn makeSelection(&mut self) -> Option<PathBuf>
	{
		match self.activeSide
		{
			Side::DirectoryTree => self.activeSide = Side::Files,
			Side::Files =>
			{
			}
		}
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
