// SPDX-License-Identifier: BSD-3-Clause

use iced::{Length, Padding, Rectangle, Size, mouse};
use iced_core::{Layout, Widget, layout, renderer, widget::Tree};
use iced_widget::text;

pub struct ListView<'a, Items, Message, Theme, Renderer = iced::Renderer>
where
	Items: ListItem,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	contents: Vec<iced_core::Element<'a, Message, Theme, Renderer>>,
	width: Length,
	height: Length,
	class: Theme::Class<'a>,
	onClick: Option<OnClickFn<'a, Items::ItemID, Message>>,
	onDoubeClick: Option<OnDoubeClickFn<'a, Items::ItemID, Message>>,
	onRightClick: Option<OnRightClickFn<'a, Items::ItemID, Message>>,
}

type OnClickFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;
type OnDoubeClickFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;
type OnRightClickFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;

pub trait ListItem: Sized
{
	type ItemID: Clone + Copy;

	fn nodeID(&self) -> Self::ItemID;
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

impl<'a, Items, Message, Theme, Renderer> ListView<'a, Items, Message, Theme, Renderer>
where
	Items: ListItem + 'a,
	Message: 'a,
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	pub fn new(items: &[Items]) -> Self
	{
		Self
		{
			contents: items.iter().map(|item| text(item.displayText()).into()).collect(),
			width: Length::Shrink,
			height: Length::Shrink,
			class: <Theme as Catalog>::default(),
			onClick: None,
			onDoubeClick: None,
			onRightClick: None,
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

	/// Sets the function to call to generate a [`Message`] for the [`ListView`] node when
	/// the node gets clicked if you wish to handle single click somehow
	#[must_use]
	pub fn onClick(mut self, click: impl Fn(Items::ItemID) -> Message + 'a) -> Self
	{
		self.onClick = Some(Box::new(click));
		self
	}

	/// Sets the function to call to generate a [`Message`] for the [`ListView`] node when
	/// the node gets clicked if you wish to handle double click somehow
	#[must_use]
	pub fn onDoubeClick(mut self, doubeClick: impl Fn(Items::ItemID) -> Message + 'a) -> Self
	{
		self.onDoubeClick = Some(Box::new(doubeClick));
		self
	}

	/// Sets the function to call to generate a [`Message`] for the [`ListView`] node when
	/// the node gets clicked if you wish to handle right click somehow
	#[must_use]
	pub fn onRightClick(mut self, rightClick: impl Fn(Items::ItemID) -> Message + 'a) -> Self
	{
		self.onRightClick = Some(Box::new(rightClick));
		self
	}
}

impl<'a, Items, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ListView<'a, Items, Message, Theme, Renderer>
where
	Items: ListItem,
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

impl<'a, Items, Message, Theme, Renderer> From<ListView<'a, Items, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Items: ListItem + 'a,
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(listView: ListView<'a, Items, Message, Theme, Renderer>) -> Self
	{
		Self::new(listView)
	}
}
