// SPDX-License-Identifier: BSD-3-Clause

use iced::{Border, border::Radius};
use iced_widget::progress_bar::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(general)
	}

	fn style(&self, class: &Self::Class<'_>) -> Style
	{
		class(self)
	}
}

pub fn general(theme: &Theme) -> Style
{
	let styles = theme.styles();

	Style
	{
		background: iced::Background::Color(styles.general.background),
		bar: iced::Background::Color(styles.text.general.colour),
		border: Border
		{
			color: styles.text.general.colour,
			width: 1.0,
			radius: 5.0.into(),
		},
	}
}
