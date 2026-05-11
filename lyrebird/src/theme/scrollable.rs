// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border, Shadow};
use iced_widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn};

use crate::theme::{Styles, container};

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
	// Extract the styling information for the theme
	let styles = theme.styles();
	// Get a base style for the scrollers widget
	let base = scrollerBase(styles);

	// Figure out how to style the scrollers for this set of bars
	let (verticalBar, horizontalBar) = match status
	{
		Status::Active { .. } => (base, base),
		Status::Hovered
		{
			is_horizontal_scrollbar_hovered: horizontalHovered,
			is_vertical_scrollbar_hovered: verticalHovered,
			..
		} =>
		{
			let horizontalBar = if horizontalHovered
			{
				Scroller
				{
					background: Background::Color(styles.scrollbar.scroller.hover),
					..base
				}
			}
			else
			{
				base
			};

			let verticalBar = if verticalHovered
			{
				Scroller
				{
					background: Background::Color(styles.scrollbar.scroller.hover),
					..base
				}
			}
			else
			{
				base
			};

			(horizontalBar, verticalBar)
		},
		Status::Dragged
		{
			is_horizontal_scrollbar_dragged: horizontalDragged,
			is_vertical_scrollbar_dragged: verticalDragged,
			..
		} =>
		{
			let horizontalBar = if horizontalDragged
			{
				Scroller
				{
					background: Background::Color(styles.scrollbar.scroller.drag),
					..base
				}
			}
			else
			{
				base
			};

			let verticalBar = if verticalDragged
			{
				Scroller
				{
					background: Background::Color(styles.scrollbar.scroller.drag),
					..base
				}
			}
			else
			{
				base
			};

			(horizontalBar, verticalBar)
		},
	};

	// Build up the final styling object to make everything look right
	Style
	{
		container: container::transparent(theme),
		vertical_rail: Rail
		{
			background: Some(Background::Color(styles.scrollbar.background)),
			border: Border::default(),
			scroller: verticalBar,
		},
		horizontal_rail: Rail
		{
			background: Some(Background::Color(styles.scrollbar.background)),
			border: Border::default(),
			scroller: horizontalBar,
		},
		gap: Some(Background::Color(styles.scrollbar.background)),
		auto_scroll: AutoScroll
		{
			background: Background::Color(styles.scrollbar.scroller.normal.scale_alpha(0.75)),
			border: Border
			{
				color: styles.scrollbar.scroller.border.scale_alpha(0.8),
				width: 1.0,
				radius: 5.0.into()
			},
			shadow: Shadow::default(),
			icon: styles.scrollbar.scroller.drag
		}
	}
}

fn scrollerBase(styles: &Styles) -> Scroller
{
	Scroller
	{
		background: Background::Color(styles.scrollbar.scroller.normal),
		border: Border
		{
			color: styles.scrollbar.scroller.border,
			width: 1.0,
			radius: 5.0.into()
		}
	}
}
