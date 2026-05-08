// SPDX-License-Identifier: BSD-3-Clause

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
		background: Some(iced::Background::Color(styles.footer.background))
	}
}
