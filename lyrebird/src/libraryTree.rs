use std::path::PathBuf;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Widget};

use crate::library::{self, MusicLibrary};

// SPDX-License-Identifier: BSD-3-Clause
pub struct LibraryTree
{
	activeEntry: Style,
	activeSide: Side,

	library: MusicLibrary,
}

#[derive(Clone, Copy)]
enum Side
{
	DirectoryTree,
	Files,
}

impl LibraryTree
{
	pub fn new(activeEntry: Style, cacheFile: PathBuf, libraryPath: &PathBuf) -> Result<Self>
	{
		Ok(LibraryTree
		{
			activeEntry: activeEntry,
			activeSide: Side::DirectoryTree,

			library: MusicLibrary::new(&cacheFile, libraryPath)?,
		})
	}

	pub fn writeCache(&self) -> Result<()>
	{
		self.library.writeCache()
	}

	pub fn handleKeyEvent(&mut self, key: KeyEvent)
	{
		if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat
		{
			match key.code
			{
				KeyCode::Left => self.moveLeft(),
				KeyCode::Right => self.moveRight(),
				_ => {},
			}
		}
	}

	fn moveLeft(&mut self)
	{
		self.activeSide = Side::DirectoryTree
	}

	fn moveRight(&mut self)
	{
		self.activeSide = Side::Files
	}
}

impl Widget for &LibraryTree
{
	fn render(self, area: Rect, buf: &mut Buffer)
		where Self: Sized
	{
		let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
			.split(area);

		Block::bordered()
			.title(" Directory Tree ")
			.title_alignment(Alignment::Left)
			.border_type(BorderType::Rounded)
			.render(layout[0], buf);

		Block::bordered()
			.title(" Files ")
			.title_alignment(Alignment::Left)
			.border_type(BorderType::Rounded)
			.render(layout[1], buf);
	}
}
