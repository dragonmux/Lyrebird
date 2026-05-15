// SPDX-License-Identifier: BSD-3-Clause

use std::{cell::RefCell, rc::Rc};

use iced::{Background, Border, Color, Event, Length, Rectangle, Size, mouse, touch};
use iced_core::{Clipboard, Layout, Shell, Widget, layout, renderer::{self, Quad}, widget::{Operation, Tree, tree}, window};
use iced_widget::{column, text};

pub struct TreeView<'a, Model, Message, Theme, Renderer = iced::Renderer>
where
	Model: TreeItem<Message>,
	Message: Clone,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	nodeID: Model::ItemID,
	node: iced_core::Element<'a, Message, Theme, Renderer>,
	subtree: iced_core::Element<'a, Message, Theme, Renderer>,
	setup: Rc<RefCell<TreeViewSetup<'a, Model::ItemID, Message, Theme>>>,
	state: State,
}

/// Structure to hold all the setup information for the [`TreeView`], to be held via Rc for all sub-trees
/// so the tree view gets a consistent setup all the way down, and things like setting the click handler
/// works correctly with less magic in [`TreeItem`]
struct TreeViewSetup<'a, ItemID, Message, Theme>
where
	Message: Clone,
	Theme: Catalog,
{
	width: Length,
	height: Length,
	class: Theme::Class<'a>,
	onSelect: Option<OnSelectFn<'a, ItemID, Message>>,
}

type OnSelectFn<'a, ItemID, Message> = Box<dyn Fn(ItemID) -> Message + 'a>;

/// The possible states of a [`TabButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// The [`TreeView`] node is not currently selected.
    Unselected,
	/// The [`TreeView`] node is being hovered over.
	Hovered,
    /// The [`TreeView`] node is currently selected.
    Selected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct TreeViewState
{
	pressed: bool
}

pub trait TreeItem<Message>: Sized
{
	type ItemID: Clone + Copy + PartialEq + Eq;

	fn nodeID(&self) -> Self::ItemID;
	fn displayText(&self) -> String;
	fn children(&self) -> &[Self];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style
{
	pub normalColour: Color,
	pub hoverColour: Color,
	pub selectedColour: Color,
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

impl<'a, Model, Message, Theme, Renderer> TreeView<'a, Model, Message, Theme, Renderer>
where
	Model: TreeItem<Message> + 'a,
	Message: Clone + 'a,
	Theme: Catalog + text::Catalog + 'a,
	Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
{
	pub fn new(model: &Model, selectedNode: Option<Model::ItemID>) -> Self
	{
		let setup = Rc::new
		(
			RefCell::new
			(
				TreeViewSetup
				{
					width: Length::Shrink,
					height: Length::Shrink,
					class: <Theme as Catalog>::default(),
					onSelect: None,
				}
			)
		);
		Self::with_setup(model, selectedNode, setup)
	}

	fn with_setup
	(
		model: &Model,
		selectedNode: Option<Model::ItemID>,
		setup: Rc<RefCell<TreeViewSetup<'a, Model::ItemID, Message, Theme>>>
	) -> Self
	{
		// Turn all the child items into tree views of their own
		let subtree = model.children()
			.into_iter()
			.map(|item| Self::with_setup(item, selectedNode, setup.clone()).into());

		let isSelected = selectedNode == Some(model.nodeID());

		Self
		{
			nodeID: model.nodeID(),
			node: text(model.displayText()).into(),
			subtree: column(subtree).into(),
			setup,
			state: if isSelected { State::Selected } else { State::Unselected },
		}
	}

	/// Sets the width of the [`TreeView`]
	#[must_use]
	pub fn width(self, width: impl Into<Length>) -> Self
	{
		self.setup.borrow_mut().width = width.into();
		self
	}

	/// Sets the height of the [`TreeView`]
	#[must_use]
	pub fn height(self, height: impl Into<Length>) -> Self
	{
		self.setup.borrow_mut().height = height.into();
		self
	}

	/// Changes the style of the [`TreeView`]
	#[must_use]
	pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self
	where
		<Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
	{
		self.setup.borrow_mut().class = (Box::new(style) as StyleFn<'a, Theme>).into();
		self
	}

	/// Sets the function to call to generate a [`Message`] for the [`TreeView`] node when
	/// the node gets selected if you wish to handle selection somehow
	#[must_use]
	pub fn onSelect(self, select: impl Fn(Model::ItemID) -> Message + 'a) -> Self
	{
		self.setup.borrow_mut().onSelect = Some(Box::new(select));
		self
	}
}

impl<'a, Model, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TreeView<'a, Model, Message, Theme, Renderer>
where
	Model: TreeItem<Message>,
	Message: Clone,
	Theme: Catalog,
	Renderer: iced_core::Renderer,
{
	fn tag(&self) -> tree::Tag
	{
		tree::Tag::of::<TreeViewState>()
	}

	fn state(&self) -> tree::State
	{
		tree::State::new(TreeViewState::default())
	}

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
			width: self.setup.borrow().width,
			height: self.setup.borrow().height,
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
		let limits = limits
			.clone()
			.width(self.setup.borrow().width)
			.height(self.setup.borrow().height)
			.loose();

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
				self.setup.borrow().width,
				self.setup.borrow().height,
				Size
				{
					width: nodeSize.width.max(subtreeSize.width),
					height: nodeSize.height + subtreeSize.height,
				}
			),
			vec![nodeLayout, subtreeLayout]
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
			self.node
				.as_widget_mut()
				.operate(&mut tree.children[0], layout.child(0), renderer, operation);
		});
		operation.traverse(&mut |operation|
		{
			self.subtree
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
		cursor: mouse::Cursor,
		renderer: &Renderer,
		clipboard: &mut dyn Clipboard,
		shell: &mut Shell<'_, Message>,
		viewport: &Rectangle,
	)
	{
		// Start by running updates on all the subcomponents
		self.node
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
		self.subtree
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

		// Check to see if we should already stop
		if shell.is_event_captured()
		{
			return;
		}

		// Then extract the layout for the node display and process where the cursor is
		let bounds = layout.child(0).bounds();

		match event
		{
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerPressed { .. }) =>
			{
				if cursor.is_over(bounds)
				{
					let state = tree.state.downcast_mut::<TreeViewState>();
					state.pressed = true;
					shell.capture_event();
				}
			},
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) |
			Event::Touch(touch::Event::FingerLifted { .. }) =>
			{
				let state = tree.state.downcast_mut::<TreeViewState>();
				if state.pressed
				{
					state.pressed = false;
					if cursor.is_over(bounds)
					{
						let onSelect = &self.setup.borrow().onSelect;
						if let Some(onSelect) = onSelect
						{
							shell.publish(onSelect(self.nodeID));
						}
					}
					shell.capture_event();
				}
			},
			Event::Touch(touch::Event::FingerLost { .. }) =>
				tree.state.downcast_mut::<TreeViewState>().pressed = false,
			_ => {},
		}

		if self.state != State::Selected
		{
			let currentStatus = if cursor.is_over(bounds)
			{
				let state = tree.state.downcast_mut::<TreeViewState>();

				if state.pressed
				{
					State::Selected
				}
				else
				{
					State::Hovered
				}
			}
			else
			{
				State::Unselected
			};

			if let Event::Window(window::Event::RedrawRequested(_)) = event
			{
				self.state = currentStatus;
			}
			else if self.state != currentStatus
			{
				shell.request_redraw();
			}
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
		cursor: mouse::Cursor,
		viewport: &Rectangle,
	)
	{
		// Extract widget styling information
		let treeStyle = theme.style(&self.setup.borrow().class);
		let style = renderer::Style { text_color: treeStyle.normalColour };

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

		let nodeStyle = renderer::Style
		{
			text_color: match self.state
			{
				State::Unselected => treeStyle.normalColour,
				State::Hovered => treeStyle.hoverColour,
				State::Selected => treeStyle.selectedColour,
			}
		};
		// Draw our entry
		self.node
			.as_widget()
			.draw(&tree.children[0], renderer, theme, &nodeStyle, layout.child(0), cursor, viewport);
		// Draw the subtree under us
		self.subtree
			.as_widget()
			.draw(&tree.children[1], renderer, theme, &style, layout.child(1), cursor, viewport);
	}
}

impl<'a, Model, Message, Theme, Renderer> From<TreeView<'a, Model, Message, Theme, Renderer>>
	for iced::Element<'a, Message, Theme, Renderer>
where
	Model: TreeItem<Message> + 'a,
	Message: Clone + 'a,
	Theme: Catalog + 'a,
	Renderer: iced_core::Renderer + 'a,
{
	fn from(treeView: TreeView<'a, Model, Message, Theme, Renderer>) -> Self
	{
		Self::new(treeView)
	}
}
