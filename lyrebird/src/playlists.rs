// SPDX-License-Identifier: BSD-3-Clause
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Padding, StatefulWidget, Widget};
use serde::{Deserialize, Serialize};

use crate::{playback::Song, playlist::Playlist};

#[derive(Serialize, Deserialize)]
pub struct Playlists
{
	nowPlaying: Playlist,
	playlists: Vec<Playlist>,
	#[serde(skip)]
	nowPlayingState: ListState,
	#[serde(skip)]
	playlistsState: ListState,
}

impl Playlists
{
	pub fn new() -> Self
	{
		Self
		{
			nowPlaying: Playlist::new("Now Playing".into()),
			playlists: Vec::new(),
			nowPlayingState: ListState::default(),
			playlistsState: ListState::default(),
		}
	}

	pub fn handleKeyEvent(&mut self, key: KeyEvent) -> Option<Result<Song>>
	{
		if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat
		{
			match key.code
			{
				KeyCode::Left => {},
				KeyCode::Right => {},
				KeyCode::Up => {},
				KeyCode::Down => {},
				KeyCode::Enter => {},
				_ => {},
			}
		}
		None
	}

	pub fn nowPlaying<'a>(&'a mut self) -> &'a mut Playlist
	{
		&mut self.nowPlaying
	}
}

impl Widget for &mut Playlists
{
	fn render(self, area: Rect, buf: &mut Buffer)
		where Self: Sized
	{
		// Split the area up so we can display a listing of all the user's playlists, and what's currently queued
		let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
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
						.border_type(BorderType::Rounded)
						.padding(Padding::horizontal(1))
				),
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
						.border_type(BorderType::Rounded)
						.padding(Padding::horizontal(1))
				),
			layout[1],
			buf,
			&mut self.nowPlayingState
		);
	}
}
