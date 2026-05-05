// SPDX-License-Identifier: BSD-3-Clause

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

// Trait so that default construction works
impl Default for TabBar
{
	fn default() -> Self
	{
		Self::new()
	}
}

// Functions for the tab bar that are agnostic of the lifetime component
impl TabBar
{
}
