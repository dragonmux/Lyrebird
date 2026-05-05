// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};
use std::time::Duration;

use color_eyre::Result;
use directories::ProjectDirs;
use tokio::sync::mpsc::{channel, Receiver};

use crate::options::OptionsPanel;
use crate::playback::{PlaybackState, Song};
use crate::playlists::Playlists;
use crate::{config::Config, libraryTree::LibraryTree};

/// Represents the main window of Lyrebird
pub struct MainWindow
{
	exit: bool,
	activeTab: Tab,

	libraryTree: LibraryTree,
	optionsPanel: OptionsPanel,
	playlists: Playlists,

	currentlyPlaying: Option<(Song, Receiver<PlaybackState>)>,
	errorState: Option<String>
}

#[derive(Clone, Copy)]
enum Tab
{
	LibraryTree = 0,
	Options = 3,
	Playlists = 4,
}

impl Tab
{
	const fn value(self) -> usize
	{
		self as usize
	}
}

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

impl MainWindow
{
	/// Set up a new main window, building the style pallet needed
	pub fn new(paths: &ProjectDirs, config: &mut Config) -> Result<Self>
	{
		Ok(Self
		{
			exit: false,
			activeTab: Tab::LibraryTree,

			libraryTree: LibraryTree::new
			(
				&paths.cache_dir().join("library.json"),
				&config.libraryPath,
			)?,
			optionsPanel: OptionsPanel::new(),
			playlists: Playlists::new(),

			currentlyPlaying: None,
			errorState: None,
		})
	}

	/// Run the program window until an exit-causing event occurs
	pub async fn run(&mut self) -> Result<()>
	{
		// Set up a redraw timer
		let mut frameTimer = tokio::time::interval(Duration::from_secs(1).div_f32(50.0));

		// Until the user's asked us to exit
		while !self.exit
		{
			// If we're not discovering the library tree any more, check if we don't need to join the background
			// thread for discovery
			if !self.libraryTree.isDiscovering()
			{
				self.libraryTree.maybeJoinDiscovery().await?;
			}
			// See if there's something to do from one of our event sources
			tokio::select!
			{
				// Redraw the terminal every 50th of a second while discovery runs
				_ = frameTimer.tick(), if self.libraryTree.isDiscovering() =>
					{ },
				// If there is a file playing, check to see if it's giving us any notifications
				Some(notification) = self.playbackNotification(), if self.currentlyPlaying.is_some() =>
					{ self.handlePlaybackNotification(&notification)? },
			}
		}
		Ok(())
	}

	fn quit(&mut self) -> Result<()>
	{
		self.exit = true;
		self.libraryTree.writeCache()
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

fn durationAsString(duration: Duration) -> String
{
	if duration.is_zero()
	{
		"--:--".to_string()
	}
	else
	{
		let seconds = duration.as_secs();
		let minutes = seconds / 60;
		let seconds = seconds % 60;
		format!("{minutes:2}:{seconds:02}")
	}
}
