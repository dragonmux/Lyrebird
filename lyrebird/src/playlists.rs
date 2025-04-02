// SPDX-License-Identifier: BSD-3-Clause
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListItem, ListState, Padding, StatefulWidget, Widget};
use serde::{Deserialize, Serialize};

use crate::window::Operation;
use crate::playlist::Playlist;

#[derive(Serialize, Deserialize)]
pub struct Playlists
{
	nowPlaying: Playlist,
	playlists: Vec<Playlist>,
	#[serde(skip)]
	activeEntry: Style,
	#[serde(skip)]
	activeSide: Side,
	#[serde(skip)]
	currentPlaylistState: ListState,
	#[serde(skip)]
	playlistsState: ListState,
}

#[derive(Clone, Copy)]
enum Side
{
	Playlists,
	PlaylistContents,
}

impl Default for Side
{
	fn default() -> Self
		{ Side::Playlists }
}

impl Playlists
{
	pub fn new(activeEntry: Style) -> Self
	{
		Self
		{
			nowPlaying: Playlist::new("Now Playing".into()),
			playlists: Vec::new(),
			activeEntry,
			activeSide: Side::Playlists,
			currentPlaylistState: ListState::default(),
			playlistsState: ListState::default(),
		}
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
				KeyCode::Enter => { return self.makeSelection(); },
				_ => {},
			}
		}
		Operation::None
	}

	pub fn nowPlaying<'a>(&'a mut self) -> &'a mut Playlist
		{ &mut self.nowPlaying }

	const fn moveLeft(&mut self)
		{ self.activeSide = Side::Playlists; }

	const fn moveRight(&mut self)
		{ self.activeSide = Side::PlaylistContents; }

	fn moveUp(&mut self)
	{
		match self.activeSide
		{
			Side::Playlists =>
			{
				self.playlistsState.select_previous();
				self.currentPlaylistState = ListState::default();
			}
			Side::PlaylistContents =>
			{
				self.currentPlaylistState.select_previous();
			}
		}
	}

	fn moveDown(&mut self)
	{
		match self.activeSide
		{
			Side::Playlists =>
			{
				self.playlistsState.select_next();
				self.currentPlaylistState = ListState::default();
			}
			Side::PlaylistContents =>
			{
				self.currentPlaylistState.select_next();
			}
		}
	}

	fn makeSelection(&mut self) -> Operation
	{
		match self.activeSide
		{
			Side::Playlists => Operation::None,
			Side::PlaylistContents =>
			{
				// Figure out which file this is from the list, starting by looking up
				// which entry is currently selected (if any)
				match self.currentPlaylistState.selected()
				{
					// If we have a valid selection
					Some(index) =>
					{
						// Look that up in the now playing list
						let fileName = self.nowPlaying.entry(index).to_path_buf();
						// Set it as the next thing to play, and ask the file to be switched to
						self.nowPlaying.nextEntry(index);
						Operation::PlayNext(fileName)
					},
					None => Operation::None,
				}
			}
		}
	}
}

impl Widget for &mut Playlists
{
	fn render(self, area: Rect, buf: &mut Buffer)
	where
		Self: Sized
	{
		// Split the area up so we can display a listing of all the user's playlists, and what's currently queued
		let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
			.split(area);

		// Render the playlist listing using the internal state object
		StatefulWidget::render
		(
			// Build a list of playlists currently available to the user
			List::new
			(
				self.playlists
					.iter()
					.map(|playlist| ListItem::new(playlist.name()))
			)
				// Put it in a bordered block for presentation
				.block
				(
					Block::bordered()
						.title(" Playlists ")
						.title_alignment(Alignment::Left)
						.title_style
						(
							match self.activeSide
							{
								Side::Playlists => self.activeEntry,
								Side::PlaylistContents => Style::default(),
							}
						)
						.border_type(BorderType::Rounded)
						.padding(Padding::horizontal(1))
				)
				.highlight_style(self.activeEntry)
				.direction(ListDirection::TopToBottom),
			layout[0],
			buf,
			&mut self.playlistsState
		);

		// Render the now playing playlist using the internal state object
		StatefulWidget::render
		(
			// Build a list of all the files in the Now Playing playlist
			List::new(self.nowPlaying.contents())
				// Put it in a bordered block for presentation
				.block
				(
					Block::bordered()
						.title(" Now Playing ")
						.title_alignment(Alignment::Left)
						.title_style
						(
							match self.activeSide
							{
								Side::PlaylistContents => self.activeEntry,
								Side::Playlists => Style::default(),
							}
						)
						.border_type(BorderType::Rounded)
						.padding(Padding::horizontal(1))
				)
				.highlight_style(self.activeEntry)
				.direction(ListDirection::TopToBottom),
			layout[1],
			buf,
			&mut self.currentPlaylistState
		);
	}
}
