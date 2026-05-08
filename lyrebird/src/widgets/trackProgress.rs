// SPDX-License-Identifier: BSD-3-Clause

use std::time::Duration;

use iced::{Length, Rectangle, Size, mouse::Cursor};
use iced_core::{Layout, Widget, layout, renderer, widget::Tree};

use crate::{messages::Message, playback::Song};

pub struct TrackProgress<'a>
{
	track: Option<&'a Song>,
	width: Length,
	height: Length,
}

impl<'a> TrackProgress<'a>
{
	pub fn new(track: Option<&'a Song>) -> Self
	{
		Self
		{
			track,
			width: Length::Fill,
			height: Length::Shrink
		}
	}
}

fn durationAsString(duration: Duration) -> String
{
	if duration.is_zero()
	{
		"--:--".to_string()
	}
	else
	{
		let seconds = duration.as_secs();
		let minutes = seconds / 60;
		let seconds = seconds % 60;
		format!("{minutes:2}:{seconds:02}")
	}
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for TrackProgress<'a>
where
	Renderer: iced_core::Renderer
{
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
		layout::Node::new(limits.min())
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
		//
	}
}

impl<'a, Theme, Renderer> From<TrackProgress<'a>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Theme: 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(tabBarWidget: TrackProgress<'a>) -> Self
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
