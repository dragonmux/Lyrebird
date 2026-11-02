// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border};

use crate::widgets::contextMenu::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(default)
	}

	fn style(&self, class: &Self::Class<'_>) -> Style
	{
		class(self)
	}
}

fn default(theme: &Theme) -> Style
{
	let styles = theme.styles();

	Style
	{
		background: Background::Color(styles.general.background),
		normalColour: styles.button.normal,
		hoverBackground: Background::Color(styles.general.background),
		hoverColour: styles.button.hover,
		border: Border
		{
			color: styles.general.border,
			width: 1.0,
			radius: 5.0.into(),
		},
	}
}
