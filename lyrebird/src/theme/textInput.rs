// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border, Color};
use iced_widget::text_input::{Catalog, Status, Style, StyleFn};

use crate::theme::Styles;

use super::Theme;

impl Catalog for Theme
{
	type Class<'a> = StyleFn<'a, Self>;

	fn default<'a>() -> Self::Class<'a>
	{
		Box::new(default)
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style
	{
		class(self, status)
	}
}

pub fn default(theme: &Theme, status: Status) -> Style
{
	// Extract the styling information and construct a base style
	let styles = theme.styles();
	let base = defaultBase(styles);

	match status
	{
		Status::Active => base,
		Status::Hovered => Style
		{
			value: styles.textInput.hover,
			..base
		},
		Status::Focused { .. } => Style
		{
			selection: styles.textInput.selection,
			..base
		},
		Status::Disabled => Style
		{
			placeholder: styles.textInput.disabled,
			value: styles.textInput.disabled,
			selection: styles.textInput.disabled,
			..base
		}
	}
}

fn defaultBase(styles: &Styles) -> Style
{
	Style
	{
		background: Background::Color(Color::TRANSPARENT),
		border: Border
		{
			color: styles.textInput.border,
			width: 1.0,
			radius: 5.0.into(),
		},
		icon: styles.textInput.selection,
		placeholder: styles.textInput.placeholder,
		value: styles.textInput.normal,
		selection: styles.textInput.normal,
	}
}
