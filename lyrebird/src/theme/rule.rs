// SPDX-License-Identifier: BSD-3-Clause

use iced_widget::rule::{Catalog, FillMode, Style, StyleFn};

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
		color: styles.text.general.colour,
		radius: 5.into(),
		fill_mode: FillMode::Padded(5),
		snap: false,
	}
}
