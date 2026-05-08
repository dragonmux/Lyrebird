// SPDX-License-Identifier: BSD-3-Clause

use std::time::Duration;

use iced::{Alignment, Length, Padding, Rectangle, Size, mouse::Cursor};
use iced_core::{Layout, Widget, layout, renderer, widget::Tree};
use iced_widget::{row, text};

use crate::{messages::Message, playback::Song, theme::Theme};

pub struct TrackProgress<'a, Theme, Renderer = iced::Renderer>
where
	Renderer: iced_core::Renderer,
{
	children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
	width: Length,
	height: Length,
}

fn durationAsString(duration: Option<Duration>) -> String
{
	if let Some(duration) = duration && !duration.is_zero()
	{
		let seconds = duration.as_secs();
		let minutes = seconds / 60;
		let seconds = seconds % 60;
		format!("{minutes:2}:{seconds:02}")
	}
	else
	{
		"--:--".to_string()
	}
}

impl<'a, Renderer> TrackProgress<'a, Theme, Renderer>
where
	Renderer: iced_core::text::Renderer + 'a
{
	pub fn new(track: Option<&'a Song>) -> Self
	{
		let children = vec!
		[
			text(track.map_or_else(|| "Nothing playing", |song| &song.description()))
				.into(),
			row
			(
				[
					text(durationAsString(track.map(|song| song.playedDuration())))
						.into(),
					text("/").into(),
					text(durationAsString(track.and_then(|song| song.songDuration())))
						.into(),
				]
			)
				.spacing(5.0)
				.into(),
		];

		Self
		{
			children,
			width: Length::Fill,
			height: Length::Shrink
		}
	}
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for TrackProgress<'a, Theme, Renderer>
where
	Renderer: iced_core::Renderer
{
	fn children(&self) -> Vec<Tree>
	{
		self.children.iter().map(Tree::new).collect()
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.diff_children(&self.children);
	}

	fn size(&self) -> Size<Length>
	{
		Size
		{
			width: self.width,
			height: self.height
		}
	}

	fn layout
	(
		&mut self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node
	{
		layout::flex::resolve
		(
			layout::flex::Axis::Horizontal,
			renderer,
			limits,
			self.width,
			self.height,
			Padding::ZERO,
			2.0,
			Alignment::Center,
			&mut self.children,
			&mut tree.children
		)
	}

	fn draw
	(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		style: &renderer::Style,
		layout: Layout<'_>,
		cursor: Cursor,
		viewport: &Rectangle,
	)
	{
		// Draw in the track progress sub-widgets
		for ((child, tree), layout) in self.children
			.iter()
			.zip(&tree.children)
			.zip(layout.children())
		{
			child.as_widget().draw(tree, renderer, theme, style, layout, cursor, viewport);
		}
	}
}

impl<'a, Theme, Renderer> From<TrackProgress<'a, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Theme: 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(tabBarWidget: TrackProgress<'a, Theme, Renderer>) -> Self
	{
		Self::new(tabBarWidget)
	}
}

// // Build a layout for the footer line
// let (footerLayout, footerSpacers ) = Layout::horizontal
// (
// 	[Constraint::Percentage(50), Constraint::Fill(1), Constraint::Fill(3)]
// )
// 	.flex(Flex::SpaceBetween)
// 	.spacing(1)
// 	.split_with_spacers(areas[2]);

// // Figure out what strings are to be displayed in the footer
// let currentlyPlaying = self.currentlyPlaying.as_ref()
// 	.map_or_else(|| String::from("Nothing playing"), |(song, _)| song.description());
// let songDuration = self.currentlyPlaying.as_ref()
// 	.and_then(|(song, _)| song.songDuration())
// 	.map_or_else
// 	(
// 		|| String::from("--:--"), durationAsString
// 	);
// let playedDuration = self.currentlyPlaying.as_ref()
// 	.map_or_else
// 	(
// 		|| String::from("--:--"),
// 		|(song, _)| durationAsString(song.playedDuration())
// 	);
// let errorState = self.errorState.as_ref().map_or_else
// (
// 	|| String::from("No errors"), Clone::clone
// );

// // Display the program footer - which song is currently playing, song runtime, and whether errors have occured
// Line::from_iter([String::from(" "), currentlyPlaying])
// 	.style(self.footer)
// 	.render(footerLayout[0], buf);
// Line::styled(format!("{playedDuration}/{songDuration}"), self.footer)
// 	.centered()
// 	.render(footerLayout[1], buf);
// Line::styled(errorState, self.footer).render(footerLayout[2], buf);

// // Render the spacers for all the components of the footer
// for spacerRect in footerSpacers.iter()
// {
// 	Line::styled("│", self.footer).render(*spacerRect, buf);
// }
