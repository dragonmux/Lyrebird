// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border};
use iced_widget::container::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(root)
	}

	fn style(&self, class: &Self::Class<'_>) -> Style
	{
		class(self)
	}
}

pub fn root(theme: &Theme) -> Style
{
	let styles = theme.styles();

	Style
	{
		background: Some(Background::Color(styles.general.background)),
		text_color: Some(styles.text.general.colour),
		..Default::default()
	}
}

pub fn transparent(_theme: &Theme) -> Style
{
	Style::default()
}

pub fn roundedBox(theme: &Theme) -> Style
{
	let styles = theme.styles();

	Style
	{
		border: Border
		{
			color: styles.general.border,
			width: 1.0,
			radius: 5.0.into(),
		},
		..Default::default()
	}
}
