// SPDX-License-Identifier: BSD-3-Clause

use iced_widget::text::{Catalog, Style, StyleFn};

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
	Style { color: Some(theme.styles().text.general.colour) }
}
