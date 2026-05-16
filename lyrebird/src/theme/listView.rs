// SPDX-License-Identifier: BSD-3-Clause

use crate::widgets::listView::{Catalog, Style, StyleFn};

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
		normalColour: styles.listView.normal,
		hoverColour: styles.listView.hover,
	}
}
