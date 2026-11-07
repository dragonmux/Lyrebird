// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, RwLock};

use color_eyre::Result;
use color_eyre::eyre::eyre;
use directories::ProjectDirs;
use iced::alignment::Horizontal;
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::{Length, Program, Settings, Subscription, Task, keyboard, window};
use iced_futures::backend::default::Executor;
use iced_widget::Column;
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::albumList::AlbumList;
use crate::artistList::ArtistList;
use crate::library::MusicLibrary;
use crate::messages::{EntityID, Message, PlayPosition, Tab};
use crate::options::OptionsPanel;
use crate::playback::{PlaybackState, TrackState};
use crate::playlists::{PlaylistID, Playlists};
use crate::theme::Theme;
use crate::track::TrackID;
use crate::widgets::trackProgress::TrackProgress;
use crate::widgets::{Element, Renderer};
use crate::widgets::tabBar::TabBar;
use crate::{config::Config, libraryTree::LibraryTree};

/// Represents the state of the main window of Lyrebird
pub struct MainWindowState
{
	windowID: Option<window::Id>,

	tabBar: TabBar<Tab>,

	libraryTree: LibraryTree,
	artistList: ArtistList,
	albumList: AlbumList,
	optionsPanel: OptionsPanel,
	playlists: Playlists,

	musicLibrary: Arc<RwLock<MusicLibrary>>,
	currentlyPlaying: Option<TrackState>,

	/// Global cancellation token to shut down anything still running such as discovery
	/// when the user closes the main interface window
	cancellation: CancellationToken,
}

/// Represents the main window of Lyrebird itself
pub struct MainWindow
{
	musicLibrary: Arc<RwLock<MusicLibrary>>,
	settings: Arc<RwLock<Config>>,
}

impl MainWindowState
{
	/// Set up the main window state
	pub fn new(mainWindow: &MainWindow) -> Self
	{
		Self
		{
			windowID: None,

			tabBar: TabBar::new("Lyrebird"),

			libraryTree: LibraryTree::new(mainWindow.musicLibrary.clone()),
			artistList: ArtistList::new(mainWindow.musicLibrary.clone()),
			albumList: AlbumList::new(mainWindow.musicLibrary.clone()),
			optionsPanel: OptionsPanel::new(mainWindow.settings.clone()),
			playlists: Playlists::new(mainWindow.musicLibrary.clone()),

			musicLibrary: mainWindow.musicLibrary.clone(),
			currentlyPlaying: None,

			cancellation: CancellationToken::new(),
		}
	}

	pub fn update(&mut self, mainWindow: &MainWindow, message: Message) -> Task<Message>
	{
		match message
		{
			Message::SwitchTo(tab) => self.tabBar.switchTo(tab),
			Message::LibraryDiscover =>
				Task::future(MusicLibrary::discover(mainWindow.musicLibrary.clone(), self.cancellation.clone())),
			Message::LibraryDiscovered =>
				Task::future(MusicLibrary::writeCache(mainWindow.musicLibrary.clone())),
			Message::LibraryTree(message) => self.libraryTree.update(message),
			Message::ArtistList(message) => self.artistList.update(message),
			Message::AlbumList(message) => self.albumList.update(message),
			Message::Playlist(message) => self.playlists.update(message),
			Message::KeyEvent(event) =>
			{
				match event
				{
					keyboard::Event::KeyPressed { key: Key::Named(Named::Space), repeat: false, .. } =>
						Task::done(Message::TogglePlayback),
					_ => Task::none()
				}
			}
			Message::WindowOpened(id) => self.handleWindowOpen(id),
			Message::WindowClosed(id) => self.handleWindowClose(id),
			Message::AddToPlaylist(playlistID, entityID, playPosition) =>
				self.handleAddToPlaylist(playlistID, entityID, playPosition),
			Message::TogglePlayback => self.togglePlayback(),
			Message::PlaybackNotification(notification) =>
				self.handlePlaybackNotification(notification)
					.expect("Playback state change notification should not have caused an error"),
			_ => Task::none(),
		}
	}

	pub fn view(&self, _mainWindow: &MainWindow) -> Element<'_, Message>
	{
		// Build the window header (view selector) and footer (playback status bar)
		let header = self.tabBar.view();
		let footer = TrackProgress::new(self.currentlyPlaying.as_ref());

		// Figure out what to display in the main area
		let content = match self.tabBar.activeTab()
		{
			Tab::LibraryTree => self.libraryTree.view(),
			Tab::Artists => self.artistList.view(),
			Tab::Albums => self.albumList.view(),
			Tab::Options => self.optionsPanel.view(),
			Tab::Playlists => self.playlists.view(),
		};

		// Compose the whole lot into a layout
		let layout = Column::with_children
		([
			header,
			content.into(),
			footer.into(),
		]);

		// And set that layout up to display things full window size nicely
		layout
			.width(Length::Fill)
			.height(Length::Fill)
			.align_x(Horizontal::Center)
			.into()
	}

	fn handleWindowOpen(&mut self, id: window::Id) -> Task<Message>
	{
		if self.windowID.is_none()
		{
			self.windowID = Some(id);
		}
		Task::none()
	}

	fn handleWindowClose(&mut self, id: window::Id) -> Task<Message>
	{
		// If this is our main window
		if self.windowID == Some(id)
		{
			// Cancel any ongoing discovery if there is any
			self.cancellation.cancel();
		}
		Task::none()
	}

	fn handleAddToPlaylist(&mut self, playlistID: PlaylistID, entityID: EntityID, playPosition: PlayPosition) -> Task<Message>
	{
		// Grab the playlist this track is to be added to
		let playlist = self.playlists.playlist(playlistID);
		// Add it to the playlist if we could get it back, report an error otherwise
		if let Some(playlist) = playlist {
			match entityID
			{
				EntityID::Track(trackID) => playlist.add(trackID, playPosition),
				EntityID::Directory(directoryID) =>
				{
					let library = self.musicLibrary.read().expect("Concurrency error, could not lock music library for read");
					playlist.addFrom(&library, directoryID, playPosition);
				},
			};
			// If the play position was now and the playlist is Now Playing, trigger playback
			if playlistID == PlaylistID::nowPlaying() && playPosition == PlayPosition::Now
			{
				// Handle playing this track via the playback completion mechanism
				return Task::done(Message::PlaybackNotification(PlaybackState::Complete))
			}
			// If it's now playing and nothing is currently playing, then set things in motion again
			else if playlistID == PlaylistID::nowPlaying() && self.currentlyPlaying.is_none()
			{
				return Task::done(Message::PlaybackNotification(PlaybackState::Complete))
			}
		} else {
			error!("Failed to get playlist {playlistID}, not adding track");
		}
		Task::none()
	}

	fn togglePlayback(&mut self) -> Task<Message>
	{
		if let Some(track) = &mut self.currentlyPlaying
		{
			match track.state()
			{
				PlaybackState::Playing =>
				{
					let result = track.pause();
					if let Err(error) = result
					{
						error!("{}", error);
					}
				},
				PlaybackState::Paused |
				PlaybackState::Stopped |
				PlaybackState::NotStarted =>
					{ track.play(); }
				PlaybackState::Complete => {}
				PlaybackState::Unknown(_error) =>
					{  }
			}
		}

		Task::none()
	}

	fn playTrack(&mut self, trackID: TrackID) -> Result<Task<Message>>
	{
		// Try and lock access to the library to get the track data for this track
		let library= self.libraryTree.library();
		let library = library
			.read()
			.map_err(|error| eyre!("Library should be lockable for read but was not: {}", error))?;
		// Grab the track and turn it into state info, dropping our library lock
		let (mut track, stateStream) = TrackState::new(library.trackFor(trackID), &library)?;
		drop(library);
		// See if another is currently playing, and make it stop if it is
		let currentlyPlaying = self.currentlyPlaying.take();
		if let Some(mut currentTrack) = currentlyPlaying
		{
			currentTrack.stop()?;
		}
		// Start the new track playing and push it into the state tracker
		track.play();
		self.currentlyPlaying = Some(track);
		Ok(Task::run(stateStream, |stateChange| Message::PlaybackNotification(stateChange)))
	}

	fn handlePlaybackNotification(&mut self, notification: PlaybackState) -> Result<Task<Message>>
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
					Some(trackID) => self.playTrack(trackID),
					None =>
					{
						self.currentlyPlaying = None;
						Ok(Task::none())
					},
				}
			},
			_ => Ok(Task::none()),
		}
	}
}

impl MainWindow
{
	// Set up a new main window, loading the music library and config settings
	pub fn new(paths: &ProjectDirs) -> Result<Self>
	{
		let config = Arc::new(RwLock::new(Config::read(paths)?));
		Ok(Self
		{
			musicLibrary: Arc::new
			(
				RwLock::new
				(
					MusicLibrary::new
					(
						&paths.cache_dir().join("library.json"),
						&config.read().map_err(|error| eyre!("Failed to read lock program config: {}", error))?.libraryPath,
					)
				)
			),
			settings: config.clone(),
		})
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
		(
			MainWindowState::new(self),
			Task::future(MusicLibrary::load(self.musicLibrary.clone()))
		)
	}

	fn update(&self, state: &mut MainWindowState, message: Message) -> Task<Message>
	{
		state.update(self, message)
	}

	fn view<'a>(&self, state: &'a MainWindowState, _windowID: window::Id) -> Element<'a, Message>
	{
		state.view(self)
	}

	fn subscription(&self, _state: &MainWindowState) -> Subscription<Message>
	{
		// Return a slew of things we want events for
		Subscription::batch
		([
			// Listen to keyboard events that haven't been captured by anything else
			keyboard::listen().map(Message::KeyEvent),
			window::open_events().map(Message::WindowOpened),
			window::close_events().map(Message::WindowClosed),
		])
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
