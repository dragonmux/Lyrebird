use std::path::PathBuf;
use std::sync::Arc;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListState, Padding, StatefulWidget, Widget};
use tokio::sync::RwLock;

use crate::library::MusicLibrary;

// SPDX-License-Identifier: BSD-3-Clause
pub struct LibraryTree
{
	activeEntry: Style,
	activeSide: Side,
	dirListState: ListState,

	library: Arc<RwLock<MusicLibrary>>,
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
			dirListState: ListState::default().with_selected(Some(0)),

			library: MusicLibrary::new(&cacheFile, libraryPath)?,
		})
	}

	pub fn writeCache(&self) -> Result<()>
	{
		self.library.blocking_read().writeCache()
	}

	pub fn handleKeyEvent(&mut self, key: KeyEvent)
	{
		if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat
		{
			match key.code
			{
				KeyCode::Left => self.moveLeft(),
				KeyCode::Right => self.moveRight(),
				KeyCode::Up => self.moveUp(),
				KeyCode::Down => self.moveDown(),
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

	fn moveUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree => { self.dirListState.select_previous(); }
			Side::Files => {}
		}
	}

	fn moveDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree => { self.dirListState.select_next(); }
			Side::Files => {}
		}
	}
}

impl Widget for &mut LibraryTree
{
	fn render(self, area: Rect, buf: &mut Buffer)
		where Self: Sized
	{
		let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
			.split(area);

		StatefulWidget::render
		(
			List::new(self.library.blocking_read().directories())
				.block
				(
					Block::bordered()
						.title(" Directory Tree ")
						.title_alignment(Alignment::Left)
						.border_type(BorderType::Rounded)
						.padding(Padding::horizontal(1))
				)
				.highlight_style(self.activeEntry)
				.direction(ListDirection::TopToBottom),
			layout[0],
			buf,
			&mut self.dirListState
		);

		Block::bordered()
			.title(" Files ")
			.title_alignment(Alignment::Left)
			.border_type(BorderType::Rounded)
			.render(layout[1], buf);
	}
}
