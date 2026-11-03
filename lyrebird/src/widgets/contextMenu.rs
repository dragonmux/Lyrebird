// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, Mutex};

use iced::{Background, Border, Color, Event, Length, Point, Rectangle, Size, Vector, mouse, touch};
use iced_core::renderer::Quad;
use iced_core::Overlay;
use iced_core::{Clipboard, Element, Layout, Shell, Widget, layout, overlay, renderer, widget::Operation};
use iced_core::widget::{Tree, tree};
use iced_widget::{rule, text};

pub struct ContextMenu<'a, Item, Context, Message, Theme, Renderer = iced::Renderer>
where
	Item: Copy + ToString,
	Context: Copy,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	content: Element<'a, Message, Theme, Renderer>,
	menuItems: Vec<MenuItem<Item>>,
	state: &'a ContextMenuState<Context>,
	onMenuClick: OnMenuClickFn<'a, Item, Context, Message>,
	onMenuClose: OnMenuCloseFn<'a, Message>,
	class: Theme::Class<'a>,
}

type OnMenuClickFn<'a, MenuItem, Context, Message> = Box<dyn Fn(MenuItem, Context) -> Message + 'a>;
type OnMenuCloseFn<'a, Message> = Box<dyn Fn() -> Message + 'a>;

pub enum MenuItem<Item>
where
	Item: Copy + ToString,
{
	Item(Item),
	Separator,
}

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

struct Menu<'a, Message, Theme, Renderer = iced::Renderer>
where
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	items: Vec<Element<'a, Message, Theme, Renderer>>,
	state: Arc<Mutex<State>>,
	onClick: OnClickFn<'a, Message>,
	class: Theme::Class<'a>,
}

type OnClickFn<'a, Message> = Box<dyn Fn(usize) -> Option<Message> + 'a>;

struct State
{
	menuTree: Tree,
	hoveredOption: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Style
{
	pub background: Background,
	pub normalColour: Color,
	pub hoverBackground: Background,
	pub hoverColour: Color,
	pub border: Border,
}

pub trait Catalog
where
	Self: Sized,
{
	type Class<'a>: From<StyleFn<'a, Self>>;
	fn default<'a>() -> Self::Class<'a>;
	fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<'a, Item, Context, Message, Theme, Renderer> ContextMenu<'a, Item, Context, Message, Theme, Renderer>
where
	Item: Copy + ToString + 'a,
	Context: Copy,
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	/// Creates a new [`ContextMenu`] for `content` with the given menu items and click handler
	pub fn new<const N: usize>
	(
		content: impl Into<Element<'a, Message, Theme, Renderer>>,
		menuItems: [impl Into<MenuItem<Item>>; N],
		state: &'a ContextMenuState<Context>,
		onMenuClick: impl Fn(Item, Context) -> Message + 'a,
		onMenuClose: impl Fn() -> Message + 'a,
	) -> Self
	{
		Self
		{
			content: content.into(),
			menuItems: menuItems.into_iter().map(Into::into).collect(),
			state,
			onMenuClick: Box::new(onMenuClick),
			onMenuClose: Box::new(onMenuClose),
			class: <Theme as Catalog>::default(),
		}
	}

	/// Changes the style of the [`ContextMenu`]
	#[must_use]
	pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
	where
		<Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
	{
		self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
		self
	}
}

impl<'a, Item, Context, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ContextMenu<'a, Item, Context, Message, Theme, Renderer>
where
	Item: Copy + ToString,
	Context: Copy,
	Theme: Catalog + text::Catalog + rule::Catalog,
	Renderer: iced_core::Renderer + iced_core::text::Renderer,
{
	fn tag(&self) -> tree::Tag
	{
		tree::Tag::of::<Arc<Mutex<State>>>()
	}

	fn state(&self) -> tree::State
	{
		tree::State::new(Arc::new(Mutex::new(State::new())))
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
		let state = tree.state.downcast_mut::<Arc<Mutex<State>>>();
		let mut state = state.lock().expect("Concurrency error, unable to lock context menu state");

		match event
		{
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerPressed { .. }) |
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) =>
			{
				if self.state.menuOpen
				{
					// Event wasn't processed by the overlay, so needs closing and the menu state resetting
					let onMenuClose = &self.onMenuClose;
					shell.publish(onMenuClose());
					state.close();
				}
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
		let state = tree.state.downcast_mut::<Arc<Mutex<State>>>();

		if self.state.menuOpen
		{
			let bounds = layout.bounds();
			let position = layout.position() + translation;

			// Turn all the menu items into iced display widgets
			let items = self.menuItems
				.iter()
				.map(|item| item.into())
				.collect();

			// Build the actual menu overlay and return it
			let menu = Menu::new
			(
				items,
				state.clone(),
				|index: usize|
				{
					let menuItem = &self.menuItems[index];
					match menuItem
					{
						MenuItem::Item(item) =>
						{
							let onMenuClick = &self.onMenuClick;
							Some(onMenuClick(*item, self.state.context.expect("A valid menu context has been established")))
						},
						MenuItem::Separator => None,
					}
				},
				|theme| Catalog::style(theme, &self.class),
			);
			Some(menu.into())
		}
		else
		{
			None
		}
	}
}

impl<'a, Item, Context, Message, Theme, Renderer> From<ContextMenu<'a, Item, Context, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Item: Copy + ToString + 'a,
	Context: Copy + 'a,
	Message: 'a,
	Theme: Catalog + text::Catalog + rule::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	fn from(contextMenu: ContextMenu<'a, Item, Context, Message, Theme, Renderer>) -> Self
	{
		Self::new(contextMenu)
	}
}

impl<Item> From<Item> for MenuItem<Item>
where
	Item: Copy + ToString,
{
	fn from(item: Item) -> Self
	{
		Self::Item(item)
	}
}

impl<Item> From<&Item> for MenuItem<Item>
where
	Item: Copy + ToString,
{
	fn from(item: &Item) -> Self
	{
		Self::Item(*item)
	}
}

impl<'a, Item, Message, Theme, Renderer> From<&MenuItem<Item>> for iced::Element<'a, Message, Theme, Renderer>
where
	Item: Copy + ToString + 'a,
	Message: 'a,
	Theme: Catalog + text::Catalog + rule::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	fn from(menuItem: &MenuItem<Item>) -> Self
	{
		match menuItem
		{
			MenuItem::Item(item) => text(item.to_string()).into(),
			MenuItem::Separator => rule::horizontal(1).into(),
		}
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

impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	pub fn new
	(
		items: Vec<Element<'a, Message, Theme, Renderer>>,
		state: Arc<Mutex<State>>,
		onClick: impl Fn(usize) -> Option<Message> + 'a,
		style: impl Fn(&Theme) -> Style + 'a,
	) -> Self
	where
		<Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
	{
		Self
		{
			items,
			state,
			onClick: Box::new(onClick),
			class: (Box::new(style) as StyleFn<'a, Theme>).into(),
		}
	}
}

impl<'a, Message, Theme, Renderer> Overlay<Message, Theme, Renderer> for Menu<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node
	{
		let state = self.state.clone();
		let mut state = state.lock().expect("Concurrency error, unable to lock context menu state");

		let limits = layout::Limits::new
		(
			Size::ZERO,
			bounds,
		)
			.width(Length::Shrink);

		let node = <Self as Widget<Message, Theme, Renderer>>::layout
		(
			self,
			&mut state.menuTree,
			renderer,
			&limits
		);
		node
	}

	fn draw
	(
		&self,
		renderer: &mut Renderer,
		theme: &Theme,
		defaults: &renderer::Style,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
	)
	{
		let state = self.state.lock().expect("Concurrency error, unable to lock context menu state");
		let bounds = layout.bounds();
		let style = Catalog::style(theme, &self.class);

		renderer.fill_quad
		(
			Quad
			{
				bounds,
				border: style.border,
				..Quad::default()
			},
			style.background,
		);

		<Self as Widget<Message, Theme, Renderer>>::draw
		(
			&self,
			&state.menuTree,
			renderer,
			theme,
			defaults,
			layout,
			cursor,
			&bounds,
		);
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Menu<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn size(&self) -> Size<Length>
	{
		Size
		{
			width: Length::Shrink,
			height: Length::Shrink,
		}
	}

	fn layout
	(
		&mut self,
		_tree: &mut Tree,
		_renderer: &Renderer,
		limits: &layout::Limits,
	) -> layout::Node
	{
		layout::Node::new(limits.min())
	}

	fn draw
	(
		&self,
		_tree: &Tree,
		_renderer: &mut Renderer,
		_theme: &Theme,
		_style: &renderer::Style,
		_layout: Layout<'_>,
		_cursor: iced_core::mouse::Cursor,
		_viewport: &Rectangle,
	)
	{
		//
	}
}

impl<'a, Message, Theme, Renderer> From<Menu<'a, Message, Theme, Renderer>>
	for overlay::Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	fn from(menu: Menu<'a, Message, Theme, Renderer>) -> Self
	{
		Self::new(Box::new(menu))
	}
}

impl State
{
	pub fn new() -> Self
	{
		Self
		{
			menuTree: Tree::empty(),
			hoveredOption: None,
		}
	}

	pub fn close(&mut self)
	{
		self.hoveredOption = None;
	}
}
