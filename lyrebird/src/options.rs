// SPDX-License-Identifier: BSD-3-Clause

use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Padding, Widget};

use crate::window::Operation;

pub struct OptionsPanel
{
}

impl OptionsPanel
{
	pub fn new() -> Self
	{
		Self
		{
		}
	}

	pub fn handleKeyEvent(&mut self, _key: KeyEvent) -> Operation
	{
		Operation::None
	}
}

impl Default for OptionsPanel
{
    fn default() -> Self
	{
        Self::new()
    }
}

impl Widget for &mut OptionsPanel
{
	fn render(self, area: Rect, buf: &mut Buffer)
	where
		Self: Sized
	{
		Block::bordered()
			.title(" Options ")
			.title_alignment(Alignment::Left)
			.border_type(BorderType::Rounded)
			.padding(Padding::horizontal(1))
			.render(area, buf);
	}
}
