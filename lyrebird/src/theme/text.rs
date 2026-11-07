// SPDX-License-Identifier: BSD-3-Clause

use iced_widget::text::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(|_theme| Style::default())
	}

	fn style(&self, class: &Self::Class<'_>) -> Style
	{
		class(self)
	}
}
