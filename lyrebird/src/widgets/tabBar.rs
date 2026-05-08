// SPDX-License-Identifier: BSD-3-Clause

use iced::mouse::{self, Cursor};
use iced::
{
	Alignment, Background, Border, Color, Event, Length, Padding, Rectangle, Shadow, Size, overlay, touch, window
};
use iced::widget::{Row, button::Status, container, text};
use iced_core::widget::{Operation, Tree, tree};
use iced_core::{Clipboard, Layout, Shell, Widget, layout, renderer};

use crate::messages::Message;
use crate::theme::{self, Theme};
use crate::widgets::Element;

/// A widget that draws a set of tabs providing equidistant space by default
pub struct TabBar<TabEnum>
where
	TabEnum: Default + TabBarEnum + Clone + Copy,
{
	/// The title for this tab bar, displayed on the left
	title: &'static str,
	/// The currently active tab on the bar
	activeTab: TabEnum,
}

struct TabBarWidget<'a, Theme, Renderer = iced::Renderer>
where
	Theme: Catalog + text::Catalog,
	Renderer: iced_core::text::Renderer,
{
	tabs: Vec<TabButton<'a, Message, Theme, Renderer>>,
	children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
	width: Length,
	height: Length,
	class: <Theme as Catalog>::Class<'a>,
}

struct TabPlaceholder
{
	height: f32,
}

struct TabButton<'a, Message, Theme = theme::Theme, Renderer = iced::Renderer>
where
	Theme: Catalog,
{
	content: iced::Element<'a, Message, Theme, Renderer>,
	onPress: Message,
	width: Length,
	height: Length,
	padding: Padding,
	class: Theme::Class<'a>,
	status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct TabButtonState
{
	pressed: bool
}

pub trait TabBarEnum
where Self:
	Sized
{
	fn tabs<'a>() -> &'a[Self];
	fn name(&self) -> &'static str;
	fn value(&self) -> usize;
	fn message_for(&self) -> Message;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style
{
    pub background: Option<Background>,
    pub titleColor: Color,
    pub tabTextColor: Color,
    pub tabNumberColor: Color,
    pub seperatorColor: Color,
    pub border: Border,
}

pub trait Catalog
{
	type Class<'a>;
	fn default<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl<TabEnum> TabBar<TabEnum>
where
	TabEnum: Default + TabBarEnum + Clone + Copy,
{
	pub fn new(title: &'static str) -> Self
	{
		Self
		{
			title,
			activeTab: TabEnum::default(),
		}
	}

	pub fn view<'a>(&'a self) -> Element<'a, Message>
	{
		TabBarWidget::new(self.title, TabEnum::tabs()).into()
	}

	pub fn switchTo(&mut self, tab: TabEnum)
	{
		self.activeTab = tab;
	}

	pub fn activeTab(&self) -> TabEnum
	{
		self.activeTab
	}
}

impl<'a, Theme, Renderer> TabBarWidget<'a, Theme, Renderer>
where
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::text::Renderer + 'a,
{
	pub fn new<TabEnum>(title: &'a str, tabs: &[TabEnum]) -> Self
	where
		TabEnum: Default + TabBarEnum + Clone + Copy,
	{
		let mut children = Vec::with_capacity(tabs.len() + 1);
		children.push(text(title).into());
		for _ in 1..children.capacity()
		{
			children.push(iced::Element::new(TabPlaceholder { height: 0.0 }));
		}

		Self
		{
			tabs: tabs
				.iter()
				.map
				(
					|tab: &TabEnum|
					{
						TabButton::new
						(
							format!("{} {}", tab.value(), tab.name()),
							tab.message_for()
						)
					}
				)
				.collect(),
			children,
			width: Length::Fill,
			height: Length::Shrink,
			class: <Theme as Catalog>::default(),
		}
	}
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for TabBarWidget<'a, Theme, Renderer>
where
	Theme: Catalog + text::Catalog,
	Renderer: iced_core::Renderer + iced_core::text::Renderer,
{
	fn children(&self) -> Vec<Tree>
	{
		self.children.iter().map(Tree::new).collect()
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.diff_children(&self.children);
	}

	fn size(&self) -> Size<Length>
	{
		Size
		{
			width: self.width,
			height: self.height
		}
	}

	fn layout(
		&mut self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node
	{
		layout::flex::resolve
		(
			layout::flex::Axis::Horizontal,
			renderer,
			limits,
			self.width,
			self.height,
			Padding::ZERO,
			2.0,
			Alignment::Center,
			&mut self.children,
			&mut tree.children
		)
	}

	fn draw(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		style: &renderer::Style,
		layout: Layout<'_>,
		cursor: iced_core::mouse::Cursor,
		viewport: &Rectangle,
	)
	{
		let bounds = layout.bounds();
		let style = Catalog::style(theme, &self.class, Status::Active);

		renderer.fill_quad
		(
			renderer::Quad
			{
				bounds: bounds,
				border: Border::default(),
				shadow: Shadow::default(),
				snap: false
			},
			style.background.unwrap_or_else(|| Background::Color(Color::TRANSPARENT)),
		);
		// let tabs = TabEnum::tabs();
		// let mut layout = Row::with_capacity(tabs.len() + 1);
		// let title = container(self.title)
		// 	.style(tabBarTitleStyle)
		// 	.width(Length::FillPortion(1))
		// 	.align_x(Alignment::Start)
		// 	.align_y(Alignment::Center)
		// 	.padding(Padding {
		// 		top: 5.0,
		// 		bottom: 5.0,
		// 		right: 10.0,
		// 		left: 25.0,
		// 	});

		// layout = layout.push(title);
		// for tab in tabs
		// {
		// 	layout = layout.push
		// 	(
		// 		TabButton::new
		// 		(
		// 			format!("{} {}", tab.value(), tab.name()),
		// 			tab.message_for()
		// 		)
		// 	);
		// }
		// layout.width(Length::Fill).into()
	}
}

impl<'a, Theme, Renderer> From<TabBarWidget<'a, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	fn from(tabBarWidget: TabBarWidget<'a, Theme, Renderer>) -> Self
	{
		Self::new(tabBarWidget)
	}
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TabPlaceholder
where
	Renderer: iced_core::Renderer,
{
	fn size(&self) -> Size<Length> {
		Size {
			width: Length::Shrink,
			height: Length::Shrink,
		}
	}

	fn layout(
		&mut self,
		_tree: &mut Tree,
		_renderer: &Renderer,
		_limits: &layout::Limits,
	) -> layout::Node {
		layout::Node::new(Size { width: 0.0, height: 0.0 })
	}

	fn draw(
		&self,
		_tree: &Tree,
		_renderer: &mut Renderer,
		_theme: &Theme,
		_style: &renderer::Style,
		_layout: Layout<'_>,
		_cursor: mouse::Cursor,
		_viewport: &Rectangle,
	) {
	}
}

impl<'a, Message, Theme, Renderer> TabButton<'a, Message, Theme, Renderer>
where
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::text::Renderer + 'a,
{
	pub fn new(value: String, onPress: Message) -> Self
	{
		Self
		{
			content: text(value).align_x(Alignment::Start).into(),
			onPress,
			width: Length::FillPortion(1),
			height: Length::Shrink,
			padding: Padding {
				top: 5.0,
				bottom: 5.0,
				right: 10.0,
				left: 10.0,
			},
			class: <Theme as Catalog>::default(),
			status: Status::Active,
		}
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TabButton<'a, Message, Theme, Renderer>
where
	Message: Clone,
	Renderer: iced_core::text::Renderer,
	Theme: Catalog,
{
	fn tag(&self) -> tree::Tag
	{
		tree::Tag::of::<TabButtonState>()
	}

	fn state(&self) -> tree::State
	{
		tree::State::new(TabButtonState::default())
	}

	fn children(&self) -> Vec<Tree>
	{
		vec![Tree::new(&self.content)]
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.diff_children(&[&self.content]);
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
		layout::padded
		(
			limits,
			self.width,
			self.height,
			self.padding,
			|limits|
			{
				self.content
					.as_widget_mut()
					.layout(&mut tree.children[0], renderer, limits)
			},
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
			self.content
				.as_widget_mut()
				.operate(&mut tree.children[0], layout.children().next().unwrap(), renderer, operation);
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
		self.content
			.as_widget_mut()
			.update
			(
				&mut tree.children[0],
				event,
				layout.children().next().unwrap(),
				cursor,
				renderer,
				clipboard,
				shell,
				viewport
			);

		if shell.is_event_captured()
		{
			return;
		}

		let bounds = layout.bounds();

		match event
		{
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerPressed { .. }) =>
			{
				if cursor.is_over(bounds)
				{
					let state = tree.state.downcast_mut::<TabButtonState>();
					state.pressed = true;
					shell.capture_event();
				}
			},
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerLifted { .. }) =>
			{
				let state = tree.state.downcast_mut::<TabButtonState>();
				if state.pressed
				{
					state.pressed = false;
					if cursor.is_over(bounds)
					{
						shell.publish(self.onPress.clone());
					}

					shell.capture_event();
				}
			},
			Event::Touch(touch::Event::FingerLost { .. }) =>
				tree.state.downcast_mut::<TabButtonState>().pressed = false,
			_ => {},
		}

		let currentStatus = if cursor.is_over(bounds)
		{
			let state = tree.state.downcast_ref::<TabButtonState>();

			if state.pressed
			{
				Status::Pressed
			}
			else
			{
				Status::Hovered
			}
		}
		else
		{
			Status::Active
		};

		if let Event::Window(window::Event::RedrawRequested(_)) = event
		{
			self.status = currentStatus;
		}
		else if self.status != currentStatus
		{
			shell.request_redraw();
		}
	}

	fn draw
	(
		&self,
		tree: &Tree,
		renderer: &mut Renderer,
		theme: &Theme,
		_style: &renderer::Style,
		layout: Layout<'_>,
		cursor: Cursor,
		viewport: &Rectangle,
	)
	{
		let bounds = layout.bounds();
		let contentLayout = layout.children().next().unwrap();
		let style = theme.style(&self.class, self.status);

		if let Some(background) = style.background
		{
			renderer.fill_quad
			(
				renderer::Quad
				{
					bounds: bounds.shrink(Padding::default().left(4.0)),
					border: style.border,
					shadow: Shadow::default(),
					snap: false
				},
				background,
			);
		}

		renderer.fill_quad
		(
			renderer::Quad
			{
				bounds: Rectangle::new(
					bounds.anchor
					(
						Size { width: 4.0, height: bounds.height },
						Alignment::Start,
						Alignment::Center
					),
					Size { width: 4.0, height: bounds.height },
				),
				border: Border::default(),
				shadow: Shadow::default(),
				snap: false
			},
			style.seperatorColor,
		);

		self.content
			.as_widget()
			.draw
			(
				&tree.children[0],
				renderer,
				theme,
				&renderer::Style { text_color: style.tabTextColor },
				contentLayout,
				cursor,
				viewport,
			);
	}

	fn mouse_interaction
	(
		&self,
		_tree: &Tree,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		_viewport: &Rectangle,
		_renderer: &Renderer,
	) -> mouse::Interaction
	{
		if cursor.is_over(layout.bounds())
		{
			mouse::Interaction::Pointer
		}
		else
		{
			mouse::Interaction::default()
		}
	}

	fn overlay<'b>
	(
		&'b mut self,
		tree: &'b mut Tree,
		layout: Layout<'b>,
		renderer: &Renderer,
		viewport: &Rectangle,
		translation: iced::Vector,
	) -> Option<overlay::Element<'b, Message, Theme, Renderer>>
	{
		self.content.as_widget_mut().overlay
		(
			&mut tree.children[0],
			layout.children().next().unwrap(),
			renderer,
			viewport,
			translation
		)
	}
}

impl<'a, Message, Theme, Renderer> From<TabButton<'a, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Message: Clone + 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	fn from(tabButton: TabButton<'a, Message, Theme, Renderer>) -> Self
	{
		Self::new(tabButton)
	}
}

impl Default for Style
{
	fn default() -> Self
	{
		Self
		{
			background: None,
			titleColor: Color::BLACK,
			tabTextColor: Color::BLACK,
			tabNumberColor: Color::BLACK,
			seperatorColor: Color::BLACK,
			border: Border::default(),
		}
	}
}

fn tabBarTitleStyle(theme: &Theme) -> container::Style
{
	let class = <Theme as Catalog>::default();
	let style = Catalog::style(theme, &class, Status::Active);

	container::Style
	{
		text_color: Some(style.titleColor),
		background: style.background,
		border: style.border,
		shadow: Shadow::default(),
		snap: false
	}
}
