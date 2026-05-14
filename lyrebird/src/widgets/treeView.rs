// SPDX-License-Identifier: BSD-3-Clause

use iced::{Alignment, Length, Padding, Rectangle, Size, mouse};
use iced_core::{Layout, Widget, layout, renderer, widget::Tree};

pub struct TreeView<'a, Model, Theme>
where
	Model: TreeItem,
	Theme: Catalog,
{
	model: Model,
	width: Length,
	height: Length,
	class: Theme::Class<'a>,
}

pub trait TreeItem: Sized
{
	fn displayName(&self) -> &str;
	fn children(&self) -> &[Self];
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

impl<'a, Model, Theme> TreeView<'a, Model, Theme>
where
	Model: TreeItem,
	Theme: Catalog + 'a,
{
	pub fn new(model: Model) -> Self
	{
		Self
		{
			model,
			width: Length::Shrink,
			height: Length::Shrink,
			class: Theme::default(),
		}
	}

	/// Sets the width of the [`TreeView`]
	#[must_use]
	pub fn width(mut self, width: impl Into<Length>) -> Self
	{
		self.width = width.into();
		self
	}

	/// Sets the height of the [`TreeView`]
	#[must_use]
	pub fn height(mut self, height: impl Into<Length>) -> Self
	{
		self.height = height.into();
		self
	}
}

impl<'a, Model, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TreeView<'a, Model, Theme>
where
	Model: TreeItem,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	fn children(&self) -> Vec<Tree>
	{
		vec![]
	}

	fn diff(&self, _tree: &mut Tree)
	{
		//
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
		layout::flex::resolve::<Message, Theme, Renderer>
		(
			layout::flex::Axis::Vertical,
			renderer,
			limits,
			self.width,
			self.height,
			Padding::ZERO,
			0.0,
			Alignment::Center,
			&mut [],
			&mut tree.children
		)
	}

	fn draw
	(
		&self,
		_tree: &Tree,
		_renderer: &mut Renderer,
		_theme: &Theme,
		_style: &renderer::Style,
		_layout: Layout<'_>,
		_cursor: mouse::Cursor,
		_viewport: &Rectangle,
	)
	{
	}
}

impl<'a, Model, Message, Theme, Renderer> From<TreeView<'a, Model, Theme>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Model: TreeItem + 'a,
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(treeView: TreeView<'a, Model, Theme>) -> Self
	{
		Self::new(treeView)
	}
}
