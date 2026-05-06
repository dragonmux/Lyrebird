// SPDX-License-Identifier: BSD-3-Clause
use iced::theme::palette;
use iced::{Background, Border, Color, Shadow, Theme, theme};
use iced::widget::button::Status;

use crate::messages::Message;

/// A widget that draws a set of tabs providing equidistant space by default
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TabBar
{
	/// The index of the selected tabs
	selected: Option<usize>,
	/// Should we show the divider before the first tab?
	firstTabDivider: bool,
	/// Should we show the divider after the last tab?
	lastTabDivider: bool,
}

pub trait TabBarEnum
where Self:
	Sized
{
	type Type;

	fn tabs<'a>() -> &'a[Self::Type];
	fn name(&self) -> &'static str;
	fn message_for(&self) -> Message;
}

// Functions for TabBar that care about the lifetime component
impl TabBar
{
	/// Construct a new tab bar
	pub fn new() -> Self
	{
		// Construct a tab bar state with defaults for everything else
		Self
		{
			selected: None,
			firstTabDivider: false,
			lastTabDivider: false,
		}
	}

	/// Set which tab is selected
    #[must_use = "method moves the value of self and returns the modified value"]
	pub fn select<T: Into<Option<usize>>>(mut self, selected: T) -> Self
	{
		self.selected = selected.into();
		self
	}

	/// Sets whether to show a divider before the first tab
    #[must_use = "method moves the value of self and returns the modified value"]
	pub fn firstTabDivider(mut self, show: bool) -> Self
	{
		self.firstTabDivider = show;
		self
	}

	/// Sets the string to use as the divider between tabs (defaults to a line drawing vertical line)
    #[must_use = "method moves the value of self and returns the modified value"]
	pub fn lastTabDivider(mut self, show: bool) -> Self
	{
		self.lastTabDivider = show;
		self
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Style
{
    pub background: Option<Background>,
    pub text_color: Color,
    pub border: Border,
    pub shadow: Shadow,
}

impl Default for Style
{
	fn default() -> Self
	{
		Self
		{
			background: None,
			text_color: Color::BLACK,
			border: Border::default(),
			shadow: Shadow::default(),
		}
	}
}

trait Catalog
{
	type Class<'a>;
	fn tabButtonDefault<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn tabButtonDefault<'a>() -> Self::Class<'a>
	{
		Box::new(normalTabButton)
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
	{
		class(self, status)
	}
}

fn normalTabButton(theme: &Theme, status: Status) -> Style
{
	let palette = theme.extended_palette();
	let base = styled(palette.primary.base);

	match status
	{
		Status::Active => base,
		Status::Pressed => Style
		{
			background: Some(Background::Color(palette.primary.weak.color)),
			..base
		},
		Status::Hovered => Style
		{
			background: Some(Background::Color(palette.primary.strong.color)),
			..base
		},
		Status::Disabled => unreachable!(),
	}
}

fn styled(pair: palette::Pair) -> Style
{
	Style
	{
		background: Some(Background::Color(pair.color)),
		text_color: pair.text,
		..Style::default()
	}
}
