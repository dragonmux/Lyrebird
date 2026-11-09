// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, RwLock};

use iced::{Alignment, Length};
use iced_widget::{Column, Row, button, text, text_input};

use crate::{config::Config, messages::Message, widgets::Element};

pub struct OptionsPanel
{
	settings: Arc<RwLock<Config>>,
}

impl OptionsPanel
{
	pub fn new(settings: Arc<RwLock<Config>>) -> Self
	{
		Self
		{
			settings,
		}
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		let settings = self.settings
			.read()
			.expect("Concurrency error with settings object");

		let layout = Column::with_children
		([
			text("Library Path")
				.width(Length::Fill)
				.align_x(Alignment::Start)
				.into(),
			Row::with_children
			([
				text_input("<path>", &settings.libraryPath.to_string_lossy())
					.width(Length::Fill)
					.align_x(Alignment::Start)
					.into(),
				button("Rescan")
					.on_press(Message::LibraryDiscover)
					.into(),
			])
				.width(Length::Fill)
				.spacing(5.0)
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
