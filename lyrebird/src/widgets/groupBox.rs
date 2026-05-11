// SPDX-License-Identifier: BSD-3-Clause

use iced::{Background, Border, Color, Event, Length, Padding, Pixels, Rectangle, Size, mouse::Cursor};
use iced_core::{Clipboard, Layout, Shell, Widget, layout, renderer::{self, Quad}, widget::{Operation, Tree}};
use iced_widget::text;

pub struct GroupBox<'a, Message, Theme, Renderer = iced::Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	title: iced_core::Element<'a, Message, Theme, Renderer>,
	content: iced_core::Element<'a, Message, Theme, Renderer>,
	width: Length,
	height: Length,
	titleMargin: f32,
	titlePadding: f32,
	padding: Padding,
	class: Theme::Class<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style
{
	pub textColour: Color,
	pub textBackground: Background,
	pub border: Border,
}

pub trait Catalog
{
	type Class<'a>;
	fn default<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<'a, Message, Theme, Renderer> GroupBox<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	/// Creates a new [`GroupBox`] with a given title
	pub fn new
	(
		title: impl text::IntoFragment<'a>,
		content: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>
	) -> Self
	{
		Self
		{
			title: text(title).into(),
			content: content.into(),
			width: Length::Shrink,
			height: Length::Shrink,
			titleMargin: 5.0,
			titlePadding: 5.0,
			padding: 5.0.into(),
			class: <Theme as Catalog>::default(),
		}
	}

	/// Sets the width of the [`GroupBox`]
	#[must_use]
	pub fn width(mut self, width: impl Into<Length>) -> Self
	{
		self.width = width.into();
		self
	}

	/// Sets the height of the [`GroupBox`]
	#[must_use]
	pub fn height(mut self, height: impl Into<Length>) -> Self
	{
		self.height = height.into();
		self
	}

	/// Sets the margins on the sides of the title of of the [`GroupBox`]
	#[must_use]
	pub fn titleMargin(mut self, margin: impl Into<Pixels>) -> Self
	{
		self.titleMargin = margin.into().0;
		self
	}

	/// Sets the padding on the sides of the title of of the [`GroupBox`]
	#[must_use]
	pub fn titlePadding(mut self, padding: impl Into<Pixels>) -> Self
	{
		self.titlePadding = padding.into().0;
		self
	}

	/// Sets the padding on the inside of the [`GroupBox`]
	#[must_use]
	pub fn padding(mut self, padding: impl Into<Padding>) -> Self
	{
		self.padding = padding.into();
		self
	}

	/// Changes the style of the [`GroupBox`]
	#[must_use]
	pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
	where
		<Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
	{
		self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
		self
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for GroupBox<'a, Message, Theme, Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	fn children(&self) -> Vec<Tree>
	{
		vec!
		[
			Tree::new(&self.title),
			Tree::new(&self.content),
		]
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.diff_children
		(
			&[&self.title, &self.content]
		);
	}

	fn size(&self) -> Size<Length>
	{
		Size
		{
			width: self.width,
			height: self.height
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
		// Extract the border width and compute the inner content's padding
		let mut innerPadding = self.padding.clone();

		// Figure out the layout for the title having made enough room for the border, padding, etc
		let titleLayout = self.title
			.as_widget_mut()
			.layout
			(
				&mut tree.children[0],
				renderer,
				&limits
					.shrink(Size
					{
						width: innerPadding.x() + (self.titleMargin + self.titlePadding) * 2.0,
						height: 0.0,
					})
			)
			.translate
			([
				innerPadding.left + self.titleMargin + self.titlePadding,
				0.0
			]);
		let titleSize = titleLayout.size();
		let titleHeight = titleLayout.bounds().height;

		// Adjust the top padding for the inner content if the title height determines we should
		if titleHeight > innerPadding.top
		{
			innerPadding.top = titleHeight;
		}

		// And then the layout for the content block
		let contentLayout = self.content
			.as_widget_mut()
			.layout
			(
				&mut tree.children[1],
				renderer,
				&limits.shrink(innerPadding)
			)
			.translate([
				innerPadding.left,
				innerPadding.top.max(titleHeight)
			]);
		let contentSize = contentLayout.size();

		// Build the final layout for the group box
		layout::Node::with_children
		(
			limits.resolve
			(
				self.width,
				self.height,
				Size
				{
					width: (contentSize.width + innerPadding.left + innerPadding.right)
						.max(titleSize.width + innerPadding.left + innerPadding.right + self.titlePadding * 2.0),
					height: titleSize.height + contentSize.height + innerPadding.bottom,
				}
			),
			vec![titleLayout, contentLayout],
		)
	}

	fn operate
	(
		&mut self,
		tree: &mut Tree,
		layout: Layout<'_>,
		renderer: &Renderer,
		operation: &mut dyn Operation,
	)
	{
		operation.container(None, layout.bounds());
		operation.traverse(&mut |operation|
		{
			self.title
				.as_widget_mut()
				.operate(&mut tree.children[0], layout.child(0), renderer, operation);
		});
		operation.traverse(&mut |operation|
		{
			self.content
				.as_widget_mut()
				.operate(&mut tree.children[1], layout.child(1), renderer, operation);
		});
	}

	fn update
	(
		&mut self,
		tree: &mut Tree,
		event: &Event,
		layout: Layout<'_>,
		cursor: Cursor,
		renderer: &Renderer,
		clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		viewport: &Rectangle,
	)
	{
		self.title
			.as_widget_mut()
			.update
			(
				&mut tree.children[0],
				event,
				layout.child(0),
				cursor,
				renderer,
				clipboard,
				shell,
				viewport
			);

		self.content
			.as_widget_mut()
			.update
			(
				&mut tree.children[1],
				event,
				layout.child(1),
				cursor,
				renderer,
				clipboard,
				shell,
				viewport
			);
	}

	fn draw
	(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		style: &renderer::Style,
		layout: Layout<'_>,
		cursor: Cursor,
		viewport: &Rectangle,
	)
	{
		// Extract widget bounds and styling information
		let bounds = layout.bounds();
		let boxStyle = theme.style(&self.class);

		// Calculate the center line of the title text
		let titleLayout = layout.child(0);
		let titleHeight = titleLayout.bounds().height;
		let textCenter = titleLayout.position().y + (titleHeight / 2.0);

		// Draw in the rounded border rectangle
		renderer.fill_quad
		(
			Quad
			{
				bounds: Rectangle
				{
					x: layout.position().x,
					y: textCenter,
					width: bounds.width,
					height: bounds.height - (titleHeight / 2.0),
				},
				border: boxStyle.border,
				..Quad::default()
			},
			Background::Color(Color::TRANSPARENT)
		);

		// Draw in the background for the title text
		renderer.fill_quad
		(
			Quad
			{
				bounds: titleLayout
					.bounds()
					.expand(Padding
					{
						top: 0.0,
						right: self.titlePadding,
						bottom: 0.0,
						left: self.titlePadding,
					}),
				..Quad::default()
			},
			boxStyle.textBackground,
		);

		// Draw all our subwidgets having completed drawing the borders for them
		self.title
			.as_widget()
			.draw
			(
				&tree.children[0],
				renderer,
				theme,
				&renderer::Style { text_color: boxStyle.textColour },
				layout.child(0),
				cursor,
				viewport
			);
		self.content
			.as_widget()
			.draw(&tree.children[1], renderer, theme, style, layout.child(1), cursor, viewport);
	}
}

impl<'a, Message, Theme, Renderer> From<GroupBox<'a, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(groupBox: GroupBox<'a, Message, Theme, Renderer>) -> Self
	{
		Self::new(groupBox)
	}
}
