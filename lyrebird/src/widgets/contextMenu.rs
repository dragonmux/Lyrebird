// SPDX-License-Identifier: BSD-3-Clause

use iced::{Event, Length, Point, Rectangle, Size, Vector, mouse, touch};
use iced::overlay::menu::{self, Menu, Style};
use iced_core::{Clipboard, Element, Layout, Shell, Widget, layout, overlay, renderer, widget::Operation};
use iced_core::widget::{Tree, tree};

pub struct ContextMenu<'a, MenuItem, Context, Message, Theme, Renderer = iced::Renderer>
where
	MenuItem: Clone + ToString,
	Context: Copy,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	content: Element<'a, Message, Theme, Renderer>,
	menuItems: &'a [MenuItem],
	state: &'a ContextMenuState<Context>,
	onMenuClick: OnMenuClickFn<'a, MenuItem, Context, Message>,
	onMenuClose: OnMenuCloseFn<'a, Message>,
	class: ContextMenuClass<'a, Theme>,
}

type OnMenuClickFn<'a, MenuItem, Context, Message> = Box<dyn Fn(MenuItem, Context) -> Message + 'a>;
type OnMenuCloseFn<'a, Message> = Box<dyn Fn() -> Message + 'a>;

#[derive(Clone)]
pub struct ContextMenuState<Context>
where
	Context: Copy,
{
	menuOpen: bool,
	context: Option<Context>,
	position: Point,
	entryHeight: f32,
}

struct State
{
	menuState: menu::State,
	hoveredOption: Option<usize>,
}

pub trait Catalog
where
	Self: Sized,
{
	fn default<'a>() -> ContextMenuClass<'a, Self>;
	fn style(&self, class: &ContextMenuClass<'_, Self>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

pub struct ContextMenuClass<'a, Theme>
{
	widgetStyle: StyleFn<'a, Theme>,
	menuStyle: menu::StyleFn<'a, Theme>,
}

impl<'a, MenuItem, Context, Message, Theme, Renderer> ContextMenu<'a, MenuItem, Context, Message, Theme, Renderer>
where
	MenuItem: Clone + ToString + 'a,
	Context: Copy,
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	/// Creates a new [`ContextMenu`] for `content` with the given menu items and click handler
	pub fn new
	(
		content: impl Into<Element<'a, Message, Theme, Renderer>>,
		menuItems: &'a [MenuItem],
		state: &'a ContextMenuState<Context>,
		onMenuClick: impl Fn(MenuItem, Context) -> Message + 'a,
		onMenuClose: impl Fn() -> Message + 'a,
	) -> Self
	{
		Self
		{
			content: content.into(),
			menuItems,
			state,
			onMenuClick: Box::new(onMenuClick),
			onMenuClose: Box::new(onMenuClose),
			class: <Theme as Catalog>::default(),
		}
	}

	/// Changes the style of the [`ContextMenu`]
	#[must_use]
	pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
	{
		self.class.menuStyle = Box::new(style);
		self
	}
}

impl<'a, MenuItem, Context, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ContextMenu<'a, MenuItem, Context, Message, Theme, Renderer>
where
	MenuItem: Clone + ToString,
	Context: Copy,
	Theme: Catalog + menu::Catalog<Class<'a> = menu::StyleFn<'a, Theme>>,
	Renderer: iced_core::Renderer + iced_core::text::Renderer,
{
	fn tag(&self) -> tree::Tag
	{
		tree::Tag::of::<State>()
	}

	fn state(&self) -> tree::State
	{
		tree::State::new(State::new())
	}

	fn children(&self) -> Vec<Tree>
	{
		[Tree::new(&self.content)].into()
	}

	fn diff(&self, tree: &mut Tree)
	{
		tree.children.clear();
		tree.children.extend([Tree::new(&self.content)]);
	}

	fn size(&self) -> Size<Length>
	{
		self.content.as_widget().size()
	}

	fn layout
	(
		&mut self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node
	{
		self.content.as_widget_mut().layout(&mut tree.children[0], renderer, limits)
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
		self.content.as_widget_mut().operate(&mut tree.children[0], layout, renderer, operation);
	}

	fn update(
		&mut self,
		tree: &mut Tree,
		event: &iced::Event,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		renderer: &Renderer,
		clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		viewport: &Rectangle,
	)
	{
		let state = tree.state.downcast_mut::<State>();

		match event
		{
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerPressed { .. }) |
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) =>
			{
				// Event wasn't processed by the overlay, so needs closing and the menu state resetting
				let onMenuClose = &self.onMenuClose;
				shell.publish(onMenuClose());
				state.close();
			},
			_ => {},
		};

		self.content
			.as_widget_mut()
			.update(&mut tree.children[0], event, layout, cursor, renderer, clipboard, shell, viewport);
	}

	fn draw(
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
		self.content.as_widget().draw(&tree.children[0], renderer, theme, style, layout, cursor, viewport);
	}

	fn overlay<'b>(
		&'b mut self,
		tree: &'b mut Tree,
		layout: Layout<'b>,
		_renderer: &Renderer,
		viewport: &Rectangle,
		translation: Vector,
	) -> Option<overlay::Element<'b, Message, Theme, Renderer>>
	{
		let state = tree.state.downcast_mut::<State>();

		if self.state.menuOpen
		{
			let bounds = layout.bounds();
			let position = layout.position() + translation;
			let menu = Menu::new
			(
				&mut state.menuState,
				self.menuItems,
				&mut state.hoveredOption,
				|item|
					(self.onMenuClick)(item, self.state.context.expect("A valid menu context has been established")),
				None,
				&self.class.menuStyle
			)
				.width(bounds.width);

			Some(menu.overlay(position, *viewport, self.state.entryHeight, Length::Shrink))
		}
		else
		{
			None
		}
	}
}

impl<'a, MenuItem, Context, Message, Theme, Renderer> From<ContextMenu<'a, MenuItem, Context, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	MenuItem: Clone + ToString + 'a,
	Context: Copy + 'a,
	Message: 'a,
	Theme: Catalog + menu::Catalog<Class<'a> = menu::StyleFn<'a, Theme>> + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	fn from(contextMenu: ContextMenu<'a, MenuItem, Context, Message, Theme, Renderer>) -> Self
	{
		Self::new(contextMenu)
	}
}

impl<Context> ContextMenuState<Context>
where
	Context: Copy,
{
	pub fn new() -> Self
	{
		Self
		{
			menuOpen: false,
			context: None,
			position: Point::default(),
			entryHeight: 0.0,
		}
	}

	pub fn open(&mut self, context: Context, position: Point, entryHeight: f32)
	{
		self.menuOpen = true;
		self.context = Some(context);
		self.position = position;
		self.entryHeight = entryHeight;
	}

	pub fn close(&mut self)
	{
		self.menuOpen = false;
		self.context = None;
	}
}

impl State
{
	pub fn new() -> Self
	{
		Self
		{
			menuState: menu::State::new(),
			hoveredOption: None,
		}
	}

	pub fn close(&mut self)
	{
		self.menuState = menu::State::new();
		self.hoveredOption = None;
	}
}

impl<'a, Theme> ContextMenuClass<'a, Theme>
{
	pub fn new(widgetStyle: impl Fn(&Theme) -> Style + 'a, menuStyle: impl Fn(&Theme) -> menu::Style + 'a) -> Self
	{
		Self
		{
			widgetStyle: Box::new(widgetStyle),
			menuStyle: Box::new(menuStyle),
		}
	}

	pub fn widgetStyle(&self, theme: &Theme) -> Style
	{
		let style = &self.widgetStyle;
		style(theme)
	}

	pub fn menuStyle(&self, theme: &Theme) -> menu::Style
	{
		let style = &self.menuStyle;
		style(theme)
	}
}
