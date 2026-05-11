// SPDX-License-Identifier: BSD-3-Clause

use std::time::Duration;

use iced::{Alignment, Background, Border, Color, Length, Padding, Point, Rectangle, Shadow, Size, border::Radius, mouse::Cursor};
use iced_core::{Layout, Widget, layout, renderer, widget::Tree};
use iced_widget::{container, progress_bar, row, text};

use crate::{messages::Message, playback::Song, theme::{self, Theme}};

pub struct TrackProgress<'a, Theme, Renderer = iced::Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
	width: Length,
	height: Length,
	class: Theme::Class<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style
{
	pub textColour: Color,
	pub background: Option<Background>,
	pub seperatorColour: Color,
	pub border: Border,
}

pub trait Catalog
{
	type Class<'a>;
	fn default<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

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
	Theme: Catalog + 'a,
	Renderer: iced_core::text::Renderer + 'a
{
	pub fn new(track: Option<&'a Song>) -> Self
	{
		let children = vec!
		[
			container(text(track.map_or_else(|| "Nothing playing", |song| &song.description())))
				.style(theme::container::transparent)
				.width(Length::FillPortion(4))
				.align_x(Alignment::Start)
				.align_y(Alignment::Center)
				.padding(Padding {
					top: 2.0,
					bottom: 2.0,
					right: 5.0,
					left: 5.0,
				})
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
				.width(Length::FillPortion(1))
				.align_y(Alignment::Center)
				.padding(Padding {
					top: 2.0,
					bottom: 2.0,
					right: 5.0,
					left: 5.0,
				})
				.spacing(5.0)
				.into(),
			container
			(
				progress_bar
				(
					track.and_then
					(
						|song| song.songDuration()).map_or_else(|| 0.0..=1.0, |duration| 0.0..=duration.as_secs_f32()
					),
					track.map_or_else(|| 0.0, |song| song.playedDuration().as_secs_f32()),
				)
					.style(playbackProgressStyle)
					.girth(Length::Fixed(10.0))
			)
				.style(theme::container::transparent)
				.padding(Padding {
					top: 2.0,
					bottom: 2.0,
					right: 5.0,
					left: 5.0,
				})
				.into(),
		];

		Self
		{
			children,
			width: Length::Fill,
			height: Length::Shrink,
			class: <Theme as Catalog>::default(),
		}
	}
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for TrackProgress<'a, Theme, Renderer>
where
	Theme: Catalog,
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
		_style: &renderer::Style,
		layout: Layout<'_>,
		cursor: Cursor,
		viewport: &Rectangle,
	)
	{
		// Extract widget bounds and styling information
		let bounds = layout.bounds();
		let barStyle = theme.style(&self.class);
		let style = renderer::Style
		{
			text_color: barStyle.textColour,
		};

		// Draw in the background for the widget
		renderer.fill_quad
		(
			renderer::Quad
			{
				bounds: bounds,
				border: Border::default(),
				shadow: Shadow::default(),
				snap: false
			},
			barStyle.background.unwrap_or_else(|| Background::Color(Color::TRANSPARENT)),
		);

		// Draw in the chunk seperators
		for index in 0..self.children.len() - 1
		{
			// Get the bounds of the widgets left and right of this seperator position
			let boundLeft = layout.child(index).bounds();
			let boundRight = layout.child(index + 1).bounds();
			let topLeft = Point::new(boundLeft.x + boundLeft.width , boundLeft.y);
			let gapWidth = boundRight.x - topLeft.x;
			// Calculate a new bounds that fills that gap
			let bounds = Rectangle::new
			(
				topLeft,
				Size::new(gapWidth, boundLeft.height)
			);
			// Draw a box in that spot using the seperator colour
			renderer.fill_quad
			(
				renderer::Quad
				{
					bounds: bounds,
					border: Border::default(),
					shadow: Shadow::default(),
					snap: false
				},
				Background::Color(barStyle.seperatorColour)
			);
		}

		// Draw in the track progress sub-widgets
		for ((child, tree), layout) in self.children
			.iter()
			.zip(&tree.children)
			.zip(layout.children())
		{
			child.as_widget().draw(tree, renderer, theme, &style, layout, cursor, viewport);
		}
	}
}

impl<'a, Theme, Renderer> From<TrackProgress<'a, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(trackProgress: TrackProgress<'a, Theme, Renderer>) -> Self
	{
		Self::new(trackProgress)
	}
}

fn playbackProgressStyle(theme: &Theme) -> progress_bar::Style
{
	let class = <Theme as Catalog>::default();
	let style = Catalog::style(theme, &class);

	progress_bar::Style
	{
		background: style.background.unwrap_or_else(|| Background::Color(Color::TRANSPARENT)),
		bar: Background::Color(style.textColour),
		border: Border
		{
			color: style.seperatorColour,
			width: 1.0,
			radius: Radius::new(5.0)
		},
	}
}
