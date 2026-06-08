// SPDX-License-Identifier: BSD-3-Clause

use iced::{Color, theme};

pub mod button;
pub mod container;
pub mod groupBox;
pub mod listView;
pub mod progressBar;
pub mod scrollable;
pub mod tabBar;
pub mod text;
pub mod textInput;
pub mod trackProgress;
pub mod treeView;

pub enum Theme
{
	Default(Styles),
	// TODO: Custom
}

pub struct Styles
{
	pub general: General,
	pub header: Header,
	pub footer: Footer,
	pub text: Text,
	pub textInput: TextInput,
	pub button: Button,
	pub scrollbar: Scrollbar,
	pub treeView: TreeView,
	pub listView: ListView,
}

pub struct General
{
	pub background: Color,
	pub border: Color,
}

pub struct Header
{
	pub background: Color,
	pub programName: TextStyle,
	pub tab: TabBar,
}

pub struct Footer
{
	pub text: TextStyle,
	pub background: Color,
	pub seperator: Color,
	pub border: Color,
}

pub struct TabBar
{
	pub button: TabButton,
	pub seperator: Color,
	pub border: Color,
}

pub struct TabButton
{
	pub normal: TextStyle,
	pub selected: TextStyle,
	pub hover: TextStyle,
	pub number: TextStyle,
}

pub struct Text
{
	pub general: TextStyle,
	pub title: TextStyle,
}

pub struct TextStyle
{
	pub colour: Color,
}

pub struct Button
{
	pub normal: Color,
	pub selected: Color,
	pub hover: Color,
	pub disabled: Color,
	pub background: Color,
	pub backgroundDisabled: Color,
	pub border: Color,
}

pub struct TextInput
{
	pub normal: Color,
	pub hover: Color,
	pub disabled: Color,
	pub placeholder: Color,
	pub selection: Color,
	pub border: Color,
}

pub struct Scrollbar
{
	pub background: Color,
	pub scroller: Scroller
}

pub struct Scroller
{
	pub border: Color,
	pub normal: Color,
	pub hover: Color,
	pub drag: Color,
}

pub struct TreeView
{
	pub normal: Color,
	pub hover: Color,
	pub selected: Color,
}

pub struct ListView
{
	pub normal: Color,
	pub hover: Color,
}

impl Theme
{
	fn styles(&self) -> &Styles
	{
		match self
		{
			Self::Default(styles) => styles,
		}
	}
}

impl theme::Base for Theme
{
	fn name(&self) -> &str
	{
		match self
		{
			Theme::Default(_) => "Lyrebird",
		}
	}

	fn mode(&self) -> theme::Mode
	{
		match self
		{
			Theme::Default(_) => theme::Mode::Dark,
		}
	}

	fn default(_preference: theme::Mode) -> Self
	{
		Self::Default(Styles::default())
	}

	fn base(&self) -> theme::Style
	{
		let styles = self.styles();
		theme::Style
		{
			background_color: styles.general.background,
			text_color: styles.text.general.colour
		}
	}

	fn palette(&self) -> Option<theme::Palette>
	{
		None
	}
}

impl Default for Theme
{
	fn default() -> Self
	{
		Self::Default(Styles::default())
	}
}

impl Default for Styles
{
	fn default() -> Self
	{
		Self
		{
			general: General
			{
				background: Color::from_rgb8(0x11, 0x11, 0x11),
				border: Color::from_rgb8(0x77, 0x77, 0x77),
			},
			text: Text
			{
				general: TextStyle { colour: Color::from_rgb8(0xd3, 0xd3, 0xd3), },
				title: TextStyle { colour: Color::from_rgb8(0x53, 0x82, 0xb9) },
			},
			textInput: TextInput
			{
				normal: Color::from_rgb8(0x53, 0x82, 0xb9),
				hover: Color::from_rgb8(0x72, 0x9f, 0xcf),
				disabled: Color::from_rgb8(0x1a, 0x32, 0x52),
				placeholder: Color::from_rgb8(0x55, 0x55, 0x55),
				selection: Color::from_rgb8(0x91, 0xbc, 0xe5),
				border: Color::from_rgb8(0x77, 0x77, 0x77),
			},
			button: Button
			{
				normal: Color::from_rgb8(0x53, 0x82, 0xb9),
				selected: Color::from_rgb8(0x72, 0x9f, 0xcf),
				hover: Color::from_rgb8(0x72, 0x9f, 0xcf),
				disabled: Color::from_rgb8(0x1a, 0x32, 0x52),
				background: Color::from_rgb8(0x33, 0x33, 0x33),
				backgroundDisabled: Color::from_rgb8(0x22, 0x22, 0x22),
				border: Color::from_rgb8(0x77, 0x77, 0x77),
			},
			header: Header
			{
				background: Color::from_rgb8(0x33, 0x33, 0x33),
				programName: TextStyle { colour: Color::from_rgb8(0x53, 0x82, 0xb9) },
				tab: TabBar
				{
					button: TabButton
					{
						normal: TextStyle { colour: Color::from_rgb8(0x34, 0x65, 0xa4) },
						selected: TextStyle { colour: Color::from_rgb8(0x72, 0x9f, 0xcf) },
						hover: TextStyle { colour: Color::from_rgb8(0x53, 0x82, 0xb9) },
						number: TextStyle { colour: Color::from_rgb8(0x72, 0x9f, 0xcf) },
					},
					seperator: Color::from_rgb8(0x34, 0x65, 0xa4),
					border: Color::from_rgb8(0x34, 0x65, 0xa4),
				}
			},
			footer: Footer
			{
				text: TextStyle { colour: Color::from_rgb8(0x53, 0x82, 0xb9) },
				background: Color::from_rgb8(0x33, 0x33, 0x33),
				seperator: Color::from_rgb8(0x34, 0x65, 0xa4),
				border: Color::from_rgb8(0x34, 0x65, 0xa4),
			},
			scrollbar: Scrollbar
			{
				background: Color::from_rgb8(0x22, 0x22, 0x22),
				scroller: Scroller
				{
					border: Color::from_rgb8(0x27, 0x4b, 0x7b),
					normal: Color::from_rgb8(0x34, 0x65, 0xa4),
					hover: Color::from_rgb8(0x53, 0x82, 0xb9),
					drag: Color::from_rgb8(0x72, 0x9f, 0xcf),
				},
			},
			treeView: TreeView
			{
				normal: Color::from_rgb8(0xa0, 0xa0, 0xa0),
				hover: Color::from_rgb8(0xd3, 0xd3, 0xd3),
				selected: Color::from_rgb8(0x72, 0x9f, 0xcf),
			},
			listView: ListView
			{
				normal: Color::from_rgb8(0xa0, 0xa0, 0xa0),
				hover: Color::from_rgb8(0xd3, 0xd3, 0xd3),
			}
		}
	}
}
