// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border};
use iced_widget::button::{Catalog, Status, Style, StyleFn};

use super::{Styles, Theme};

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(general)
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
	{
		class(self, status)
	}
}

pub fn general(theme: &Theme, status: Status) -> Style
{
	// Extract the styling information and construct a base style
	let styles = theme.styles();
	let base = generalBase(styles);

	match status
	{
		Status::Active => base,
		Status::Pressed => Style
		{
			text_color: styles.button.selected,
			..base
		},
		Status::Hovered => Style
		{
			text_color: styles.button.hover,
			..base
		},
		Status::Disabled => Style
		{
			background: Some(Background::Color(styles.button.backgroundDisabled)),
			text_color: styles.button.disabled,
			..base
		},
	}
}

fn generalBase(styles: &Styles) -> Style
{
	Style
	{
		background: Some(Background::Color(styles.button.background)),
		text_color: styles.button.normal,
		border: Border
		{
			color: styles.button.border,
			width: 1.0.into(),
			radius: 5.0.into(),
		},
		..Style::default()
	}
}
