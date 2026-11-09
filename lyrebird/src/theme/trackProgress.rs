// SPDX-License-Identifier: BSD-3-Clause

use iced::Border;

use crate::{theme::Theme, widgets::trackProgress::{Catalog, Style, StyleFn}};

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(footer)
	}

	fn style(&self, class: &Self::Class<'_>) -> Style
	{
		class(self)
	}
}

fn footer(theme: &Theme) -> Style
{
	// Extract the styling information for the theme
	let styles = theme.styles();

	Style
	{
		textColour: styles.footer.text.colour,
		background: Some(iced::Background::Color(styles.footer.background)),
		seperatorColour: styles.footer.seperator,
		border: Border
		{
			color: styles.footer.border,
			width: 1.0,
			radius: 0.0.into(),
		},
	}
}
