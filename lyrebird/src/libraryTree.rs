// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};
use std::sync::Arc;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListState, Padding, StatefulWidget, Widget};
use tokio::sync::RwLock;

use crate::library::MusicLibrary;
use crate::playback::Song;
use crate::window::Operation;

pub struct LibraryTree
{
	activeEntry: Style,
	activeSide: Side,
	dirListState: ListState,
	filesListState: ListState,

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
	pub fn new(activeEntry: Style, cacheFile: &Path, libraryPath: &Path) -> Result<Self>
	{
		Ok(Self
		{
			activeEntry,
			activeSide: Side::DirectoryTree,
			dirListState: ListState::default().with_selected(Some(0)),
			filesListState: ListState::default(),

			library: MusicLibrary::new(cacheFile, libraryPath)?,
		})
	}

	pub fn writeCache(&self) -> Result<()>
	{
		self.library.blocking_read().writeCache()
	}

	pub fn handleKeyEvent(&mut self, key: KeyEvent) -> Operation
	{
		if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat
		{
			match key.code
			{
				KeyCode::Left => self.moveLeft(),
				KeyCode::Right => self.moveRight(),
				KeyCode::Up => self.moveUp(),
				KeyCode::Down => self.moveDown(),
				KeyCode::Enter =>
				{
					let selection = self.makeSelection();
					return match selection
					{
						Some(selection) => Operation::Play(Song::from(selection.as_path())),
						None => Operation::None,
					};
				},
				KeyCode::Char('+') => { return Operation::playlist(self.makeSelection()); },
				_ => {},
			}
		}
		Operation::None
	}

	const fn moveLeft(&mut self)
	{
		self.activeSide = Side::DirectoryTree;
	}

	const fn moveRight(&mut self)
	{
		self.activeSide = Side::Files;
	}

	fn moveUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
				self.dirListState.select_previous();
				self.filesListState = ListState::default();
			}
			Side::Files =>
			{
				self.filesListState.select_previous();
			}
		}
	}

	fn moveDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
				self.dirListState.select_next();
				self.filesListState = ListState::default();
			}
			Side::Files =>
			{
				self.filesListState.select_next();
			}
		}
	}

	/// If the currently sellected side is the directory listing, switch to that directory's file listing
	/// otherwise, if it's the file listing, figure out which one and make a SongState for it
	fn makeSelection(&mut self) -> Option<PathBuf>
	{
		match self.activeSide
		{
			Side::DirectoryTree => { self.activeSide = Side::Files },
			Side::Files =>
			{
				// Lock open access to the library
				let library = self.library.blocking_read();
				// Extract the current directory selection
				let dir = library.directoryAt(self.dirListState.selected()?)?;
				// Extract the current file selection
				let file = library.fileIn(dir, self.filesListState.selected()?)?;
				// Now make a new SongState object for that file if possible
				return Some(dir.join(file));
			}
		}
		None
	}
}

impl Widget for &mut LibraryTree
{
	fn render(self, area: Rect, buf: &mut Buffer)
		where Self: Sized
	{
		// Split the display area up to display the user's library tree on the left, and the files in a given
		// directory on the right
		let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
			.split(area);

		// Get a lock on the library so we get a consistent view of it for rendering
		let libraryLock = self.library.blocking_read();

		// Render the directory list using the internal state object
		StatefulWidget::render
		(
			// Build a list of directories currently in the library
			List::new(libraryLock.directories())
				// Put it in a bordered block for presentation
				.block
				(
					Block::bordered()
						.title(" Directory Tree ")
						.title_alignment(Alignment::Left)
						.title_style
						(
							match self.activeSide
							{
								Side::DirectoryTree => self.activeEntry,
								Side::Files => Style::default(),
							}
						)
						.border_type(BorderType::Rounded)
						// Make sure the contents are padded one space on the sides for presentation
						.padding(Padding::horizontal(1))
				)
				.highlight_style(self.activeEntry)
				.direction(ListDirection::TopToBottom),
			layout[0],
			buf,
			&mut self.dirListState
		);

		// Build a list of files in the current directory being displayed
		let filesList = libraryLock.filesFor(self.dirListState.selected())
			.map(List::new)
			.unwrap_or_default()
			// Put it in a bordered block for presentation
			.block
			(
				Block::bordered()
					.title(" Files ")
					.title_alignment(Alignment::Left)
					.border_type(BorderType::Rounded)
					.title_style
					(
						match self.activeSide
						{
							Side::Files => self.activeEntry,
							Side::DirectoryTree => Style::default(),
						}
					)
					// Make sure the contents are padded one space on the sides for presentation
					.padding(Padding::horizontal(1))
			)
			.highlight_style(self.activeEntry)
			.direction(ListDirection::TopToBottom);

		StatefulWidget::render(filesList, layout[1], buf, &mut self.filesListState);
	}
}
