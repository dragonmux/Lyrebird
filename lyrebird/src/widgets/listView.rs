// SPDX-License-Identifier: BSD-3-Clause

use std::cell::RefCell;
use std::rc::Rc;

use iced::{Event, Length, Padding, Rectangle, Size, mouse, touch};
use iced_core::{Clipboard, Layout, Shell, Widget, layout, mouse::{Click, click}, renderer, widget::{Operation, Tree, tree}};
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
	setup: Rc<RefCell<ListViewSetup<'a, Theme>>>,
	onClick: Option<OnClickFn<'a, Items::ItemID, Message>>,
	onDoubeClick: Option<OnDoubeClickFn<'a, Items::ItemID, Message>>,
	onRightClick: Option<OnRightClickFn<'a, Items::ItemID, Message>>,
}

type OnClickFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;
type OnDoubeClickFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;
type OnRightClickFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;

/// Structure to hold the styling setup information for the [`ListView`], to be held via Rc,
/// so that the [`ListEntry`] items can get their styling information
struct ListViewSetup<'a, Theme>
where
	Theme: Catalog,
{
	class: Theme::Class<'a>,
}

#[derive(Debug, Clone, Copy)]
struct ListViewState<ItemID>
where
	ItemID: Copy,
{
	nodeID: Option<ItemID>,
	previousClick: Option<Click>,
}

pub trait ListItem: Sized
{
	type ItemID: Copy + 'static;

	fn nodeID(&self) -> Self::ItemID;
	fn displayText(&self) -> String;
}

struct ListEntry<'a, ItemID, Message, Theme, Renderer = iced::Renderer>
where
	ItemID: Copy,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	nodeID: ItemID,
	content: iced::Element<'a, Message, Theme, Renderer>,
	setup: Rc<RefCell<ListViewSetup<'a, Theme>>>,
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
		let setup = Rc::new
		(
			RefCell::new
			(
				ListViewSetup
				{
					class: <Theme as Catalog>::default(),
				}
			)
		);

		Self
		{
			contents: items
				.iter()
				.map(|item|
				{
					ListEntry::new(item.nodeID(), item.displayText(), setup.clone()).into()
				})
				.collect(),
			width: Length::Shrink,
			height: Length::Shrink,
			setup,
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

	/// Changes the style of the [`ListView`]
	#[must_use]
	pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self
	where
		<Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
	{
		self.setup.borrow_mut().class = (Box::new(style) as StyleFn<'a, Theme>).into();
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
	fn tag(&self) -> tree::Tag
	{
		tree::Tag::of::<ListViewState<Items::ItemID>>()
	}

	fn state(&self) -> tree::State
	{
		tree::State::new(ListViewState::<Items::ItemID>::default())
	}

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
		for ((content, tree), layout) in self.contents
			.iter_mut()
			.zip(&mut tree.children)
			.zip(layout.children())
		{
			operation.traverse(&mut |operation|
			{
				content
					.as_widget_mut()
					.operate(tree, layout, renderer, operation);
			});
		}
	}

	fn update
	(
		&mut self,
		tree: &mut Tree,
		event: &Event,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		renderer: &Renderer,
		clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		viewport: &Rectangle,
	)
	{
		// Start by running updates on all the contents of the list view
		for ((content, tree), layout) in self.contents
			.iter_mut()
			.zip(&mut tree.children)
			.zip(layout.children())
		{
			content
				.as_widget_mut()
				.update(tree, event, layout, cursor, renderer, clipboard, shell, viewport);
		}

		// Check to see if we should already stop
		if shell.is_event_captured()
		{
			return;
		}

		// Loop through all the items in the list
		for (item, layout) in tree.children.iter().zip(layout.children())
		{
			// Extract the item's bounds and check if the mouse is over this one
			let bounds = layout.bounds();
			if cursor.is_over(bounds)
			{
				// It is, then this is the item we're interested in
				let state = tree.state.downcast_mut::<ListViewState<Items::ItemID>>();
				state.nodeID = Some(*item.state.downcast_ref::<Items::ItemID>());
				break;
			}
		}

		// Figure out how to handle the event that lead us here
		match event
		{
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerPressed { .. }) =>
			{
				if let Some(cursorPosition) = cursor.position()
				{
					let state = tree.state.downcast_mut::<ListViewState<Items::ItemID>>();
					let click = Click::new(cursorPosition, mouse::Button::Left, state.previousClick);
					state.previousClick = Some(click);
					shell.capture_event();
				}
			},
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerLifted { .. }) =>
			{
				let state = tree.state.downcast_ref::<ListViewState<Items::ItemID>>();
				if let Some(click) = &state.previousClick && let Some(nodeID) = state.nodeID
				{
					if let Some(onDoubeClick) = &self.onDoubeClick && click.kind() == click::Kind::Double
					{
						shell.publish(onDoubeClick(nodeID));
						shell.capture_event();
					}
					else if let Some(onSingleClick) = &self.onClick && click.kind() == click::Kind::Single
					{
						shell.publish(onSingleClick(nodeID));
						shell.capture_event();
					}
				}
			},
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) =>
			{
				let state = tree.state.downcast_ref::<ListViewState<Items::ItemID>>();
				// If the user has a right-click handler registered, handle the event with it
				if let Some(onRightClick) = &self.onRightClick && let Some(nodeID) = state.nodeID
				{
					shell.publish(onRightClick(nodeID));
					shell.capture_event();
				}
			},
			_ => {},
		}
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

impl<ItemID> Default for ListViewState<ItemID>
where
	ItemID: Copy,
{
	fn default() -> Self
	{
		Self
		{
			nodeID: None,
			previousClick: None
		}
	}
}

impl<'a, ItemID, Message, Theme, Renderer> ListEntry<'a, ItemID, Message, Theme, Renderer>
where
	ItemID: Copy,
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	pub fn new(nodeID: ItemID, displayText: String, setup:Rc<RefCell<ListViewSetup<'a, Theme>>>) -> Self
	{
		Self
		{
			nodeID,
			content: text(displayText).into(),
			setup,
		}
	}
}

impl<'a, ItemID, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ListEntry<'a, ItemID, Message, Theme, Renderer>
where
	ItemID: Copy + 'static,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	fn tag(&self) -> tree::Tag
	{
		tree::Tag::of::<ItemID>()
	}

	fn state(&self) -> tree::State
	{
		tree::State::new(self.nodeID)
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

	fn draw
	(
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
		let _listStyle = theme.style(&self.setup.borrow().class);

		self.content.as_widget().draw(&tree.children[0], renderer, theme, style, layout, cursor, viewport);
	}
}

impl<'a, ItemID, Message, Theme, Renderer> From<ListEntry<'a, ItemID, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	ItemID: Copy + 'static,
	Message: 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(listEntry: ListEntry<'a, ItemID, Message, Theme, Renderer>) -> Self
	{
		Self::new(listEntry)
	}
}
