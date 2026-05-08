// SPDX-License-Identifier: BSD-3-Clause

use iced::Border;
use iced_widget::button::Status;

use crate::{theme::{Styles, Theme}, widgets::tabBar::{Catalog, Style, StyleFn}};

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(header)
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
	{
		class(self, status)
	}
}

/// Styler for when a TabBar is used as the program header
pub fn header(theme: &Theme, status: Status) -> Style
{
	// Extract the styling information and construct a base style
	let styles = theme.styles();
	let base = headerBase(styles);

	// Customise the base as required by the status being rendered for
	match status
	{
		// TabButton is just pressable but not pressed
		Status::Active => base,
		// TabButton is actively selected
		Status::Pressed => Style
		{
			tabTextColor: styles.header.tab.button.selected.colour,
			..base
		},
		// TabButton is being hovered over
		Status::Hovered => Style
		{
			tabTextColor: styles.header.tab.button.hover.colour,
			..base
		},
		// TabButtons cannot be disabled
		Status::Disabled => unreachable!(),
	}
}

/// Creates a base style for the tab bar when used in the program header
fn headerBase(styles: &Styles) -> Style
{
	Style
	{
		background: Some(iced::Background::Color(styles.header.background)),
		titleColor: styles.header.programName.colour,
		tabTextColor:  styles.header.tab.button.normal.colour,
		tabNumberColor: styles.header.tab.button.number.colour,
		seperatorColour: styles.header.tab.seperator,
		border: Border
		{
			color: styles.header.tab.border,
			width: 1.0,
			radius: 0.0.into(),
		},
	}
}
