// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border, Shadow, overlay::menu};
use iced_widget::overlay::menu::Style;

use crate::widgets::contextMenu::{Catalog, ContextMenuClass};

use super::Theme;

impl Catalog for Theme
{
	fn default<'a>() -> ContextMenuClass<'a, Self>
	{
		ContextMenuClass::new(default, contextMenu)
	}

	fn style(&self, class: &ContextMenuClass<'_, Self>) -> Style
	{
		class.widgetStyle(self)
	}
}

impl menu::Catalog for Theme
{
	type Class<'a> = menu::StyleFn<'a, Self>;

	fn default<'a>() -> menu::StyleFn<'a, Self>
	{
		Box::new(contextMenu)
	}

	fn style(&self, class: &menu::StyleFn<'_, Self>) -> Style
	{
		class(self)
	}
}

fn contextMenu(theme: &Theme) -> Style
{
	default(theme)
}

fn default(theme: &Theme) -> Style
{
	let styles = theme.styles();

	Style
	{
		background: Background::Color(styles.general.background),
		border: Border
		{
			color: styles.general.border,
			width: 1.0,
			radius: 5.0.into(),
		},
		text_color: styles.button.normal,
		selected_text_color: styles.button.selected,
		selected_background: Background::Color(styles.general.background),
		shadow: Shadow::default(),
	}
}
