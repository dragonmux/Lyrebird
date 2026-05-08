// SPDX-License-Identifier: BSD-3-Clause

use iced::Background;
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
