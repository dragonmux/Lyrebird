// SPDX-License-Identifier: BSD-3-Clause

use iced::{Length, Padding, Rectangle, Size, mouse};
use iced_core::{Layout, Widget, layout, renderer, widget::Tree};
use iced_widget::text;

pub struct ListView<'a, Message, Theme, Renderer = iced::Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	contents: Vec<iced_core::Element<'a, Message, Theme, Renderer>>,
	width: Length,
	height: Length,
	class: Theme::Class<'a>,
}

pub trait ListItem: Sized
{
	fn displayText(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style
{
}

pub trait Catalog
{
	type Class<'a>;
	fn default<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<'a, Message, Theme, Renderer> ListView<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	pub fn new<Items>(items: &[Items]) -> Self
	where
		Items: ListItem
	{
		Self
		{
			contents: items.iter().map(|item| text(item.displayText()).into()).collect(),
			width: Length::Shrink,
			height: Length::Shrink,
			class: <Theme as Catalog>::default(),
		}
	}

	/// Sets the width of the [`ListView`]
	#[must_use]
	pub fn width(mut self, width: impl Into<Length>) -> Self
	{
		self.width = width.into();
		self
	}

	/// Sets the height of the [`ListView`]
	#[must_use]
	pub fn height(mut self, height: impl Into<Length>) -> Self
	{
		self.height = height.into();
		self
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ListView<'a, Message, Theme, Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	fn children(&self) -> Vec<Tree>
	{
		self.contents.iter().map(Tree::new).collect()
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.diff_children(&self.contents);
	}

	fn size(&self) -> Size<Length>
	{
		Size
		{
			width: self.width,
			height: self.height,
		}
	}

	fn layout
	(
		&mut self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node
	{
		layout::flex::resolve
		(
			layout::flex::Axis::Vertical,
			renderer,
			limits,
			self.width,
			self.height,
			Padding::default(),
			0.0,
			iced::Alignment::Start,
			&mut self.contents,
			&mut tree.children
		)
	}

	fn draw
	(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		style: &renderer::Style,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		viewport: &Rectangle,
	)
	{
		let _listStyle = theme.style(&self.class);

		for ((node, tree), layout) in self.contents
			.iter()
			.zip(&tree.children)
			.zip(layout.children())
		{
			node
				.as_widget()
				.draw(tree, renderer, theme, style, layout, cursor, viewport);
		}
	}
}

impl<'a, Message, Theme, Renderer> From<ListView<'a, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(listView: ListView<'a, Message, Theme, Renderer>) -> Self
	{
		Self::new(listView)
	}
}
