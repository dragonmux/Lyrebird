// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, RwLock};

use iced::{Length, Padding};
use iced_widget::scrollable::{Direction, Scrollbar};
use iced_widget::{Container, Row, scrollable};

use crate::library::MusicLibrary;
use crate::messages::Message;
use crate::theme::Theme;
use crate::widgets::Element;
use crate::widgets::groupBox::GroupBox;

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
}
