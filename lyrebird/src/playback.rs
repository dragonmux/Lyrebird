// SPDX-License-Identifier: BSD-3-Clause
use std::ops::Deref;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use color_eyre::eyre::{self, OptionExt, Result};
use libAudio::audioFile::AudioFile;

pub struct Song
{
	description: String,
	duration: Option<Duration>,
	played: Duration,
	playbackThread: Option<JoinHandle<()>>,
	state: Arc<ThreadState>
}

#[derive(Clone, PartialEq, Eq)]
pub enum PlaybackState
{
	NotStarted,
	Playing,
	Paused,
	Stopped,
	Unknown(String),
}

struct ThreadState
{
	audioFile: AudioFile,
	notification: Sender<PlaybackState>,
	state: Mutex<PlaybackState>,
}

impl Song
{
	/// Try to make a new Song from the path to a given file
	pub fn from(fileName: &Path, notificationChannel: Sender<PlaybackState>) -> Result<Self>
	{
		// Ask libAudio to open the file for read and playback, and grab how long the file's playback lasts
		let audioFile = AudioFile::readFile(fileName)
			.ok_or_eyre(format!("Failed to open file {}", fileName.to_string_lossy()))?;
		let totalTime = audioFile.fileInfo().totalTime();

		// Build a description of the song being played to display
		let fileInfo = audioFile.fileInfo();
		let title = fileInfo.title()?;
		let album = fileInfo.album()?;
		let artist = fileInfo.artist()?;

		Ok
		(
			Self
			{
				description: Self::buildDescriptionFrom(fileName, title, album, artist),
				duration: if totalTime != 0 { Some(Duration::from_secs(totalTime)) } else { None },
				played: Duration::default(),
				playbackThread: None,
				state: Arc::new(ThreadState::from(audioFile, notificationChannel)),
			}
		)
	}

	// Try to build a description of this track from parts
	fn buildDescriptionFrom(fileName: &Path, title: Option<String>, album: Option<String>, artist: Option<String>)
		-> String
	{
		// If the title, album and artist are all missing, then use the full path to the file as a description
		if title.is_none() && album.is_none() && artist.is_none()
		{
			return fileName.to_string_lossy().to_string();
		}

		// Otherwise, at least one of these is not None, so try to build up
		// the description chunks, starting with the title
		let mut description = match title
		{
			Some(title) => title.clone(),
			None => fileName.file_name().unwrap_or(fileName.as_os_str()).to_string_lossy().to_string(),
		};
		// Now add the album, if we have one
		if let Some(album) = album
		{
			description += format!(" - {album}").as_str();
		}
		// And finally the artist, if we have that
		if let Some(artist) = artist
		{
			description += format!(" - {artist}").as_str();
		}

		description
	}

	// Return a copy of the description of what this song is
	pub fn description(&self) -> String
	{
		self.description.clone()
	}

	// Extract how long the song runs for
	pub fn songDuration(&self) -> Option<Duration>
	{
		self.duration
	}

	// Extract how much we've played of this song
	pub fn playedDuration(&self) -> Duration
	{
		self.played
	}

	// Launch playback of the song on a seperate thread
	pub fn play(&mut self)
	{
		// If there is not already playback running
		if let None = self.playbackThread
		{
			let state = self.state.clone();
			let task = move || { state.play(); };
			self.playbackThread = Some(spawn(task));
		}
	}

	// Pause playback of the song
	pub fn pause(&mut self) -> Result<()>
	{
		// If we're in a playing state, pause playback
		let result = self.state.pause(self.playbackThread.take());
		self.playbackThread = None;
		result
	}

	// Stop playback of the song
	pub fn stop(&mut self) -> Result<()>
	{
		// If we're in a playing state, stop playback
		let result = self.state.stop(self.playbackThread.take());
		self.playbackThread = None;
		result
	}

	// Query the state playback is currently in for this song
	pub fn state(&self) -> PlaybackState
	{
		self.state.state.lock()
			.map(|lock| lock.clone())
			.unwrap_or_else(|error| PlaybackState::Unknown(error.to_string()))
	}
}

impl ThreadState
{
	pub fn from(audioFile: AudioFile, notification: Sender<PlaybackState>) -> Self
	{
		Self
		{
			audioFile,
			notification,
			state: Mutex::new(PlaybackState::NotStarted),
		}
	}

	fn play(&self)
	{
		if self.switchTo(PlaybackState::Playing)
		{
			self.audioFile.play();
		}
	}

	fn pause(&self, threadHandle: Option<JoinHandle<()>>) -> Result<()>
	{
		// See if we have any work to do
		if self.switchTo(PlaybackState::Paused)
		{
			// Now actually pause playback
			self.audioFile.pause();
			// Extract the join handle
			return threadHandle.map
			(
				|thread|
				{
					// Ask the thread to join, and map any error it produces to our error types
			 		let result = thread.join()
						.map_err(|error| eyre::eyre!("Error from playback thread: {:?}", error));
					return result;
				}
			)
			// Extract the resulting Result from that, making this an Ok if there was no thread to join
			.unwrap_or_else(|| Ok(()));
		}
		Ok(())
	}

	fn stop(&self, threadHandle: Option<JoinHandle<()>>) -> Result<()>
	{
		// See if we have any work to do
		if self.switchTo(PlaybackState::Stopped)
		{
			// Now actually stop playback
			self.audioFile.stop();
			// Extract the join handle
			return threadHandle.map
			(
				|thread|
				{
					// Ask the thread to join, and map any error it produces to our error types
			 		let result = thread.join()
						.map_err(|error| eyre::eyre!("Error from playback thread: {:?}", error));
					return result;
				}
			)
			// Extract the resulting Result from that, making this an Ok if there was no thread to join
			.unwrap_or_else(|| Ok(()));
		}
		Ok(())
	}

	/// This is essentially compare-exchange - if we are already in the state
	/// being requested, then this fails by returning false. Otherwise, the state
	/// is atomically updated and we return true
	fn switchTo(&self, newState: PlaybackState) -> bool
	{
		let mut state = self.state.lock()
			.expect("playback state mutex in invalid state");
		if *state != newState
		{
			*state = newState;
			return true;
		}
		false
	}
}

impl Drop for ThreadState
{
	fn drop(&mut self)
	{
		self.audioFile.stop();
	}
}
