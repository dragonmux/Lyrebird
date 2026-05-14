// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border, Color, Length, Rectangle, Size, mouse};
use iced_core::{Layout, Widget, layout, renderer::{self, Quad}, widget::Tree};
use iced_widget::{column, text};

pub struct TreeView<'a, Message, Theme, Renderer = iced::Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	node: iced_core::Element<'a, Message, Theme, Renderer>,
	subtree: iced_core::Element<'a, Message, Theme, Renderer>,
	width: Length,
	height: Length,
	selectMessage: Message,
	class: Theme::Class<'a>,
}

pub trait TreeItem<Message>: Sized
{
	fn displayText(&self) -> String;
	fn children(&self) -> &[Self];
	fn selectMessage(&self) -> Message;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style
{
	pub textColour: Color,
	pub treeColour: Color,
	pub backgroundColour: Color,
}

pub trait Catalog
{
	type Class<'a>;
	fn default<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<'a, Message, Theme, Renderer> TreeView<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	pub fn new<Model>(model: &Model) -> Self
	where
		Model: TreeItem<Message>
	{
		// Turn all the child items into tree views of their own
		let subtree = model.children()
			.into_iter()
			.map(|item| Self::new(item).into());

		Self
		{
			node: text(model.displayText()).into(),
			subtree: column(subtree).into(),
			width: Length::Shrink,
			height: Length::Shrink,
			selectMessage: model.selectMessage(),
			class: <Theme as Catalog>::default(),
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

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TreeView<'a, Message, Theme, Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	fn children(&self) -> Vec<Tree>
	{
		vec!
		[
			Tree::new(&self.node),
			Tree::new(&self.subtree),
		]
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.diff_children(&[&self.node, &self.subtree]);
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
		// Compute new limits that take into account the configured width and height
		let limits = limits.clone().width(self.width).height(self.height).loose();

		// Figure out the layout for our entry in the tree
		let nodeLayout = self.node
			.as_widget_mut()
			.layout(&mut tree.children[0], renderer, &limits);
		let nodeSize = nodeLayout.size();

		// Now constrain in the layout to position the subtree, if present, and calculate
		// the layout for that
		let subtreeLayout = self.subtree
			.as_widget_mut()
			.layout
			(
				&mut tree.children[1],
				renderer,
				&limits
					.shrink(Size
					{
						width: 16.0,
						height: nodeSize.height,
					})
			)
			.translate([
				16.0,
				nodeSize.height,
			]);
		let subtreeSize = subtreeLayout.size();

		layout::Node::with_children
		(
			limits.resolve
			(
				self.width,
				self.height,
				Size
				{
					width: nodeSize.width.max(subtreeSize.width),
					height: nodeSize.height + subtreeSize.height,
				}
			),
			vec![nodeLayout, subtreeLayout]
		)
	}

	fn draw
	(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		_style: &renderer::Style,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		viewport: &Rectangle,
	)
	{
		// Extract widget styling information
		let treeStyle = theme.style(&self.class);
		let style = renderer::Style { text_color: treeStyle.textColour };

		// Extract the bounds information for the sublayouts
		let nodeBounds = layout.child(0).bounds();
		let subtreeBounds = layout.child(1).bounds();

		// Calculate how tall the guide bar should be, and if it's non-zero, draw it
		let guideHeight = (subtreeBounds.height - (nodeBounds.height / 2.0) + 5.0).max(0.0);
		if guideHeight > 0.0
		{
			// Draw in the guide bar for the entries under us
			renderer.fill_quad(
				Quad
				{
					bounds: Rectangle
					{
						x: nodeBounds.x + 4.0,
						y: subtreeBounds.y - 5.0,
						width: 10.0,
						height: guideHeight,
					},
					border: Border
					{
						color: treeStyle.treeColour,
						width: 1.0,
						radius: 5.0.into(),
					},
					..Default::default()
				},
				Background::Color(Color::TRANSPARENT)
			);
			// Blot out the regions that drew that shouldn't have been drawn but had to be to make it work
			renderer.fill_quad
			(
				Quad
				{
					bounds: Rectangle
					{
						x: nodeBounds.x + 4.0,
						y: subtreeBounds.y - 5.0,
						width: 10.0,
						height: 5.0,
					},
					..Default::default()
				},
				Background::Color(treeStyle.backgroundColour)
			);
			renderer.fill_quad
			(
				Quad
				{
					bounds: Rectangle
					{
						x: nodeBounds.x + 9.0,
						y: subtreeBounds.y - 5.0,
						width: 5.0,
						height: guideHeight,
					},
					..Default::default()
				},
				Background::Color(treeStyle.backgroundColour)
			);
		}

		// Draw our entry
		self.node
			.as_widget()
			.draw(&tree.children[0], renderer, theme, &style, layout.child(0), cursor, viewport);
		// Draw the subtree under us
		self.subtree
			.as_widget()
			.draw(&tree.children[1], renderer, theme, &style, layout.child(1), cursor, viewport);
	}
}

impl<'a, Message, Theme, Renderer> From<TreeView<'a, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(treeView: TreeView<'a, Message, Theme, Renderer>) -> Self
	{
		Self::new(treeView)
	}
}
