use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

// SPDX-License-Identifier: BSD-3-Clause
pub struct LibraryTree
{
}

impl LibraryTree
{
	pub fn new() -> Self
	{
		LibraryTree
		{
			//
		}
	}
}

impl Widget for &LibraryTree
{
	fn render(self, area: Rect, buf: &mut Buffer)
		where Self: Sized
	{
	}
}
