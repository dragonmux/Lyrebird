// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};

use color_eyre::Result;
use directories::ProjectDirs;
use iced::alignment::Horizontal;
use iced::{Length, Program, Settings, Task, window};
use iced_futures::backend::default::Executor;
use iced_widget::{Column, text};
use tokio::sync::mpsc::{channel, Receiver};

use crate::messages::{Message, Tab};
use crate::options::OptionsPanel;
use crate::playback::{PlaybackState, Song};
use crate::playlists::Playlists;
use crate::theme::{self, Theme};
use crate::widgets::trackProgress::TrackProgress;
use crate::widgets::{Element, Renderer};
use crate::widgets::tabBar::{TabBar, TabBarEnum};
use crate::{config::Config, libraryTree::LibraryTree};

/// Represents the state of the main window of Lyrebird
pub struct MainWindowState
{
	exit: bool,

	tabBar: TabBar<Tab>,

	optionsPanel: OptionsPanel,
	playlists: Playlists,

	currentlyPlaying: Option<(Song, Receiver<PlaybackState>)>,
	errorState: Option<String>
}

/// Represents the main window of Lyrebird itself
pub struct MainWindow;

pub enum Operation
{
	/// Processing event determined there's nothing needs to be done
	None,
	/// Play a file, replacing the Now Playing playlist
	Play(PathBuf),
	/// Play a file already in the Now Playing playlist as if the current reached `PlaybackState::Complete`
	PlayNext(PathBuf),
	/// Add a file to the Now Playing playlist
	Playlist(PathBuf),
}

impl Operation
{
	pub fn playlist(song: Option<PathBuf>) -> Self
	{
		match song
		{
			Some(song) => Operation::Playlist(song),
			None => Operation::None,
		}
	}
}

impl MainWindowState
{
	/// Set up the main window state
	pub fn new(_mainWindow: &MainWindow) -> Self
	{
		Self
		{
			exit: false,

			tabBar: TabBar::new("Lyrebird"),

			optionsPanel: OptionsPanel::new(),
			playlists: Playlists::new(),

			currentlyPlaying: None,
			errorState: None,
		}
	}

	pub fn update(&mut self, _mainWindow: &MainWindow, message: Message) -> Task<Message>
	{
		match message
		{
			Message::SwitchTo(tab) => self.tabBar.switchTo(tab),
			_ => {},
		}
		Task::none()
	}

	pub fn view(&self, _mainWindow: &MainWindow) -> Element<'_, Message>
	{
		let header = self.tabBar.view();
		let footer = TrackProgress::new(self.currentlyPlaying.as_ref().map(|(track, _)| track));
		let content = text!("{} content", self.tabBar.activeTab().name())
			.style(theme::text::general)
			.center()
			.height(Length::Fill);

		let layout = Column::with_children
		([
			header,
			content.into(),
			footer.into(),
		]);

		layout
			.width(Length::Fill)
			.height(Length::Fill)
			.align_x(Horizontal::Center)
			.into()
	}

	fn playSong(&mut self, fileName: &Path) -> Result<()>
	{
		// Make a new channel for the new playback thread to communicate back to us with
		let (sender, receiver) = channel(1);
		let mut song = Song::from(fileName, sender)?;
		let currentlyPlaying = self.currentlyPlaying.take();
		// If we already have a song playing, stop it
		if let Some((mut currentSong, _)) = currentlyPlaying
		{
			currentSong.stop()?;
		}
		// Now replace the current playing state with the new one having asked this new one to start
		song.play();
		self.currentlyPlaying = Some((song, receiver));
		Ok(())
	}

	fn playlistSong(&mut self, fileName: &Path) -> Result<()>
	{
		let nowPlaying = self.playlists.nowPlaying();
		nowPlaying.add(fileName);
		match &self.currentlyPlaying
		{
			Some(_) => Ok(()),
			None => self.playSong(fileName),
		}
	}

	fn togglePlayback(&mut self)
	{
		if let Some((song, _)) = &mut self.currentlyPlaying
		{
			match song.state()
			{
				PlaybackState::Playing =>
				{
					let result = song.pause();
					if let Err(error) = result
					{
						self.errorState = Some(error.to_string());
					}
				},
				PlaybackState::Paused |
				PlaybackState::Stopped |
				PlaybackState::NotStarted =>
					{ song.play(); }
				PlaybackState::Complete => {}
				PlaybackState::Unknown(error) =>
					{ self.errorState = Some(error); }
			}
		}
	}

	// Wait for a playback notification from the currently playing song - note, it is an
	// error to call this function if self.currentlyPlaying is None!
	async fn playbackNotification(&mut self) -> Option<PlaybackState>
	{
		#[expect(clippy::unwrap_used, reason = "impossible in context")]
		let (_, channel) = self.currentlyPlaying.as_mut().unwrap();
		channel.recv().await
	}

	fn handlePlaybackNotification(&mut self, notification: &PlaybackState) -> Result<()>
	{
		match notification
		{
			// Playback completed, so.. go find out if there's something more
			// to play in the now playing playlist, and set it going if there is
			PlaybackState::Complete =>
			{
				let nowPlaying = self.playlists.nowPlaying();
				let nextEntry = nowPlaying.next();
				match nextEntry
				{
					Some(fileName) => self.playSong(fileName.as_path())?,
					None => self.currentlyPlaying = None,
				}
			},
			_ => {},
		}
		Ok(())
	}
}

impl MainWindow
{
	// Set up a new main window, loading the music library and config settings
	pub fn new(_paths: &ProjectDirs, _config: &mut Config) -> Result<Self>
	{
		Ok(Self)
	}
}

impl Program for MainWindow
{
	type State = MainWindowState;
	type Message = Message;
	type Theme = Theme;
	type Renderer = Renderer;
	type Executor = Executor;

	fn name() -> &'static str
	{
		"Lyrebird"
	}

	fn title(&self, _state: &MainWindowState, _window: window::Id) -> String
	{
		Self::name().to_string()
	}

	fn theme(&self, _state: &MainWindowState, _window: window::Id) -> Option<Theme>
	{
		Some(<Theme as Default>::default())
	}

	fn boot(&self) -> (MainWindowState, Task<Message>)
	{
		(MainWindowState::new(self), Task::none())
	}

	fn update(&self, state: &mut MainWindowState, message: Message) -> Task<Message>
	{
		state.update(self, message)
	}

	fn view<'a>(&self, state: &'a MainWindowState, _windowID: window::Id) -> Element<'a, Message>
	{
		state.view(self)
	}

	fn settings(&self) -> Settings
	{
		Settings::default()
	}

	fn window(&self) -> Option<window::Settings>
	{
		Some(window::Settings::default())
	}
}
