// SPDX-License-Identifier: BSD-3-Clause
use itertools::Itertools;
use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Style, Styled},
	symbols,
	text::{Line, Span},
	widgets::Widget,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A widget that draws a set of tabs providing equidistant space by default
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TabBar<'a>
{
	/// Content of the various tabs to display
	tabs: Vec<Line<'a>>,
	/// The index of the selected tabs
	selected: Option<usize>,
	/// The style used to draw the text
	style: Style,
	/// Style to apply to the selected item
	highlightedStyle: Style,
	/// Tab divider
	divider: Span<'a>,
	/// Should we show the divider before the first tab?
	firstTabDivider: bool,
	/// Should we show the divider after the last tab?
	lastTabDivider: bool,
}

// Functions for TabBar that care about the lifetime component
impl<'a> TabBar<'a>
{
	/// Construct a new tab bar
	pub fn new<Iter>(tabs: Iter) -> Self
	where
		Iter: IntoIterator,
		Iter::Item: Into<Line<'a>>,
	{
		// Turn the tab title contents into
		let tabs = tabs.into_iter().map(Into::into).collect_vec();
		let selected = if tabs.is_empty() { None } else { Some(0) };

		// Construct a tab bar state with defaults for everything else
		Self {
			tabs,
			selected,
			style: Style::default(),
			highlightedStyle: Style::default(),
			divider: Span::raw(symbols::line::VERTICAL),
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

	/// Sets the normal style for the tabs
	#[must_use = "method moves the value of self and returns the modified value"]
	pub fn style<S: Into<Style>>(mut self, style: S) -> Self
	{
		self.style = style.into();
		self
	}

	/// Sets the highlighted tab style for the tabs
	#[must_use = "method moves the value of self and returns the modified value"]
	pub fn highlightedStyle<S: Into<Style>>(mut self, style: S) -> Self
	{
		self.highlightedStyle = style.into();
		self
	}

	/// Sets the string to use as the divider between tabs (defaults to a line drawing vertical line)
	#[must_use = "method moves the value of self and returns the modified value"]
	pub fn divider<T>(mut self, divider: T) -> Self
	where
		T: Into<Span<'a>>,
	{
		self.divider = divider.into();
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

// Trait so that default construction works
impl Default for TabBar<'_>
{
	fn default() -> Self
	{
		Self::new(Vec::<Line>::new())
	}
}

// Trait so we can construct a TabBar from an iterator of Line-able items
impl<'a, Item> FromIterator<Item> for TabBar<'a>
where
	Item: Into<Line<'a>>,
{
	fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self
	{
		Self::new(iter)
	}
}

// Trait so that ratatui styling works
impl Styled for TabBar<'_>
{
	type Item = Self;

	fn style(&self) -> Style
	{
		self.style
	}

	fn set_style<S: Into<Style>>(self, style: S) -> Self::Item
	{
		self.style(style)
	}
}

// Trait so that ratatui widget rendering works (moved object variant)
impl Widget for TabBar<'_>
{
	fn render(self, area: Rect, buf: &mut Buffer)
	{
		Widget::render(&self, area, buf);
	}
}

// Trait so that ratatui widget rendering works (borrowed object variant)
impl Widget for &TabBar<'_>
{
	fn render(self, area: Rect, buf: &mut Buffer)
	{
		// Set the tab bar's main style on the buffer and then defer to our internal renderTabs call
		buf.set_style(area, self.style);
		self.renderTabBar(area, buf);
	}
}

// Functions for the tab bar that are agnostic of the lifetime component
impl TabBar<'_>
{
	/// Compute how wide the divider span is in blocks
	fn dividerWidth(&self) -> u16
	{
		// Get the actual string content of the divider
		let string = self.divider.content.as_ref();
		// Now turn that into graphemes
		let graphemes = UnicodeSegmentation::graphemes(string, true);
		// Now we have a bunch of graphemes, figure out if they're visible or not,
		// and count wide the whole lot is in total
		graphemes
			.filter(|symbol| !symbol.contains(|char: char| char.is_control()))
			.map(|symbol| symbol.width() as u16)
			.filter(|width| *width > 0)
			.reduce(|a, b| a + b)
			.unwrap_or(0)
	}

	/// Render the tab bar to the given surface area of the console
	fn renderTabBar(&self, area: Rect, buf: &mut Buffer)
	{
		// Check if we have any area to draw into
		if area.is_empty()
		{
			return;
		}

		// Count how many tabs we have to display
		let tabCount = self.tabs.len();
		// Extract the bounds of the area we have to work in
		let mut left = area.left();
		let mut right = area.right();
		// Check we have enough room for all the tabs
		if tabCount > (right - left) as usize
		{
			return;
		}
		// Compute how wide a divider is in blocks
		let dividerWidth = self.dividerWidth();

		// Deal with the first tab divider
		if self.firstTabDivider
		{
			let remainingArea = right.saturating_sub(left);
			let pos = buf.set_span(left, area.top(), &self.divider, remainingArea);
			// The same amount of space will be taken by the other divider if used
			if self.lastTabDivider
			{
				right -= dividerWidth;
			}
			left = pos.0;
		}

		// Now we have the exact bounds of the area we can use, subtract out the dividers
		let totalArea = (right - left).saturating_sub((tabCount - 1) as u16 * dividerWidth);
		// Now compute how wide each tab can be
		let tabArea = totalArea.saturating_div(tabCount as u16);

		// Loop through all the tabs
		for (idx, tab) in self.tabs.iter().enumerate()
		{
			// Draw out the tab into its space
			buf.set_line(left, area.top(), tab, tabArea);
			// Check if this is the selected tab, and if it is.. use the highlighted style
			if self.selected == Some(idx)
			{
				buf.set_style(
					Rect {
						x: left,
						y: area.top(),
						width: tabArea,
						height: 1,
					},
					self.highlightedStyle,
				);
			}
			left += tabArea;

			// If this is the last tab, exit the loop early
			if idx == tabCount - 1
			{
				break;
			}

			// Otherwise, draw the divider for this tab
			buf.set_span(left, area.top(), &self.divider, dividerWidth);
			left += dividerWidth;
		}

		// Deal with the last tab divider
		if self.lastTabDivider
		{
			let remainingArea = right.saturating_sub(left);
			buf.set_span(left, area.top(), &self.divider, remainingArea);
		}
	}
}
