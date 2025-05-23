// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use color_eyre::eyre::{self, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect, Size};
use ratatui::style::Style;
use ratatui::symbols::scrollbar;
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListState, Padding, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget};

use crate::library::MusicLibrary;
use crate::window::Operation;

pub struct LibraryTree
{
	activeEntry: Style,
	activeSide: Side,
	dirListState: ListState,
	dirListScrollbar: ScrollbarState,
	filesListState: ListState,
	filesListScrollbar: ScrollbarState,
	viewportSize: Size,

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
	pub fn new(activeEntry: Style, cacheFile: &Path, libraryPath: &Path, viewportSize: Size) -> Result<Self>
	{
		Ok(Self
		{
			activeEntry,
			activeSide: Side::DirectoryTree,
			dirListState: ListState::default().with_selected(Some(0)),
			dirListScrollbar: ScrollbarState::default(),
			filesListState: ListState::default(),
			filesListScrollbar: ScrollbarState::default(),
			viewportSize,

			library: MusicLibrary::new(cacheFile, libraryPath)?,
		})
	}

	pub fn writeCache(&self) -> Result<()>
	{
		self.library.read()
			.map_err
			(
				|error|
					eyre::eyre!("While writing library cache: {}", error.to_string())
			)?
			.writeCache()
	}

	pub fn isDiscovering(&self) -> bool
	{
		self.library.read().expect("Library lock in bad state").isDiscovering()
	}

	pub async fn maybeJoinDiscovery(&self) -> Result<()>
	{
		MusicLibrary::maybeJoinDiscoveryThread(&self.library).await
	}

	pub fn handleKeyEvent(&mut self, key: &KeyEvent) -> Operation
	{
		if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat
		{
			match key.code
			{
				KeyCode::Left => self.moveLeft(),
				KeyCode::Right => self.moveRight(),
				KeyCode::Up => self.moveUp(),
				KeyCode::Down => self.moveDown(),
				KeyCode::PageUp => self.movePageUp(),
				KeyCode::PageDown => self.movePageDown(),
				KeyCode::Enter => { return self.playSelection(); },
				KeyCode::Char('+') => { return Operation::playlist(self.makeSelection()); },
				_ => {},
			}
		}
		Operation::None
	}

	pub fn handleResize(&mut self, newSize: Size)
		{ self.viewportSize = newSize; }

	const fn moveLeft(&mut self)
		{ self.activeSide = Side::DirectoryTree; }

	const fn moveRight(&mut self)
		{ self.activeSide = Side::Files; }

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

	fn movePageUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
				self.dirListState.scroll_up_by(self.viewportSize.height);
				self.filesListState = ListState::default();
			}
			Side::Files =>
			{
				self.filesListState.scroll_up_by(self.viewportSize.height);
			}
		}
	}

	fn movePageDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
				self.dirListState.scroll_down_by(self.viewportSize.height);
				self.filesListState = ListState::default();
			}
			Side::Files =>
			{
				self.filesListState.scroll_down_by(self.viewportSize.height);
			}
		}
	}

	/// If the currently sellected side is the directory listing, switch to that directory's file listing
	/// otherwise, if it's the file listing, figure out which one and make a `SongState` for it
	fn makeSelection(&mut self) -> Option<PathBuf>
	{
		match self.activeSide
		{
			Side::DirectoryTree => self.activeSide = Side::Files,
			Side::Files =>
			{
				// Lock open access to the library
				let library = self.library.read().ok()?;
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

	fn playSelection(&mut self) -> Operation
	{
		let selection = self.makeSelection();
		match selection
		{
			Some(selection) => Operation::Play(selection),
			None => Operation::None,
		}
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
		let libraryLock = self.library.read().expect("Library lock in bad state");

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

		// Rebuild the directory scroll bar to take into account any library changes that
		// occured since last redraw, and figure out where the user is currently scrolled to
		self.dirListScrollbar = self.dirListScrollbar
			.content_length(libraryLock.directoryCount().saturating_sub(self.viewportSize.height.into()))
			.position(self.dirListState.selected().unwrap_or_default().saturating_sub(self.viewportSize.height.into()));
		// Render the scroll location of the directory list
		StatefulWidget::render
		(
			Scrollbar::new(ScrollbarOrientation::VerticalRight)
				.symbols(scrollbar::VERTICAL)
				.begin_symbol(None)
				.end_symbol(None),
			layout[0].inner(Margin::new(0, 1)),
			buf,
			&mut self.dirListScrollbar,
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

		// Rebuild the files scroll bar to take into account any library changes that
		// occured since last redraw, and figure out where the user is currently scrolled to
		self.filesListScrollbar = self.filesListScrollbar
			.content_length
			(
				libraryLock
					.filesCount(self.dirListState.selected())
					.saturating_sub(self.viewportSize.height.into())
			)
			.position
			(
				self.filesListState
					.selected()
					.unwrap_or_default()
					.saturating_sub(self.viewportSize.height.into())
			);
		// Render the scroll location of the files list
		StatefulWidget::render
		(
			Scrollbar::new(ScrollbarOrientation::VerticalRight)
				.symbols(scrollbar::VERTICAL)
				.begin_symbol(None)
				.end_symbol(None),
			layout[1].inner(Margin::new(0, 1)),
			buf,
			&mut self.filesListScrollbar,
		);
	}
}
