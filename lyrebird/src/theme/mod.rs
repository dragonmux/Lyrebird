// SPDX-License-Identifier: BSD-3-Clause

use iced::{Color, theme};

pub mod container;
pub mod tabBar;
pub mod text;
pub mod trackProgress;

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
}

pub struct General
{
	pub background: Color,
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
}

pub struct TextStyle
{
	pub colour: Color,
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
			},
			text: Text
			{
				general: TextStyle { colour: Color::from_rgb8(0xd3, 0xd3, 0xd3), },
			},
			header: Header
			{
				background: Color::from_rgb8(0x44, 0x44, 0x44),
				programName: TextStyle { colour: Color::from_rgb8(0x53, 0x82, 0xb9) },
				tab: TabBar
				{
					button: TabButton
					{
						normal:  TextStyle { colour: Color::from_rgb8(0x34, 0x65, 0xa4) },
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
				background: Color::from_rgb8(0x44, 0x44, 0x44),
				seperator: Color::from_rgb8(0x34, 0x65, 0xa4),
				border: Color::from_rgb8(0x34, 0x65, 0xa4),
			}
		}
	}
}
