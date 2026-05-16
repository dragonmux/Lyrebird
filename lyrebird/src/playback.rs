// SPDX-License-Identifier: BSD-3-Clause

use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use color_eyre::eyre::{self, OptionExt, Result, eyre};
use libAudio::audioFile::AudioFile;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::library::MusicLibrary;
use crate::track::Track;

pub struct TrackState
{
	description: String,
	duration: Option<Duration>,
	played: Duration,
	notification: Receiver<PlaybackState>,
	playbackThread: Option<JoinHandle<()>>,
	state: Arc<ThreadState>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum PlaybackState
{
	NotStarted,
	Playing,
	Paused,
	Stopped,
	Complete,
	Unknown(String),
}

struct ThreadState
{
	audioFile: AudioFile,
	notification: Sender<PlaybackState>,
	state: Mutex<PlaybackState>,
}

impl TrackState
{
	pub fn new(track: &Track, library: &MusicLibrary) -> Result<Self>
	{
		let audioFile = track.audioFile()
			.ok_or_eyre(eyre!("Failed to open file {}", track.fileName()))?;
		let (sender, receiver) = channel(1);
		let totalTime = track.totalLength();
		let album = track
			.albumID()
			.map(|albumID| library.albumFor(albumID))
			.map(|album| album.name());
		let artist = track
			.artistID()
			.map(|artistID| library.artistFor(artistID))
			.map(|artist| artist.name());

		Ok(Self
		{
			description: Self::buildDescriptionFrom(track.title(), album, artist),
			duration: if totalTime != 0 { Some(Duration::from_secs(totalTime)) } else { None },
			played: Duration::default(),
			notification: receiver,
			playbackThread: None,
			state: Arc::new(ThreadState::from(audioFile, sender))
		})
	}

	// Try to build a description of this track from parts
	fn buildDescriptionFrom(title: &str, album: Option<&str>, artist: Option<&str>)
		-> String
	{
		// Start out with the track title, turning it into an owned String
		let mut description = title.to_string();
		// Now add the album, if we have one
		if let Some(album) = album
		{
			description += &format!(" - {album}");
		}
		// And finally the artist, if we have that
		if let Some(artist) = artist
		{
			description += &format!(" - {artist}");
		}

		description
	}

	/// Return a reference to a description of what the track is
	pub fn description(&self) -> &str
	{
		&self.description
	}

	/// Extract how long the track runs for
	pub fn trackDuration(&self) -> Option<Duration>
	{
		self.duration
	}

	/// Extract how much we've played of this track
	pub fn playedDuration(&self) -> Duration
	{
		self.played
	}

	/// Launch playback of the track on a seperate thread
	pub fn play(&mut self)
	{
		// If there is not already playback running
		if self.playbackThread.is_none()
		{
			let state = self.state.clone();
			let task = move || { state.play(); };
			self.playbackThread = Some(spawn(task));
		}
	}

	/// Pause playback of the track
	pub fn pause(&mut self) -> Result<()>
	{
		// If we're in a playing state, pause playback
		let result = self.state.pause(self.playbackThread.take());
		self.playbackThread = None;
		result
	}

	/// Stop playback of the track
	pub fn stop(&mut self) -> Result<()>
	{
		// If we're in a playing state, stop playback
		let result = self.state.stop(self.playbackThread.take());
		self.playbackThread = None;
		result
	}

	/// Query the state playback is currently in for this track
	pub fn state(&self) -> PlaybackState
	{
		self.state.state.lock()
			.map_or_else
			(
				|error| PlaybackState::Unknown(error.to_string()),
				|lock| lock.clone()
			)
	}

	pub fn notification(&mut self) -> &mut Receiver<PlaybackState>
	{
		&mut self.notification
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
		// Switch into playing state if we're not already
		if self.switchTo(PlaybackState::Playing)
		{
			// We weren't already, so have libAudio actually do playback (this is blocking!)
			self.audioFile.play();
			// Now, check what playback state we're in.. if we're in Playing still, the file ended
			// and we should notify the main window of this fact via a channel
			let mut state = self.state.lock()
				.expect("playback state mutex in invalid state");
			if *state == PlaybackState::Playing
			{
				*state = PlaybackState::Complete;
			}
			let state = state.clone();
			self.notification.blocking_send(state).expect("Notification sender has bad state");
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
			return threadHandle.map_or_else
			(
				||
				{
					// If there's no thread to join, then just return Ok.
					Ok(())
				},
				|thread|
				{
					// Ask the thread to join, and map any error it produces to our error types
			 		thread.join()
						.map_err(|error| eyre::eyre!("Error from playback thread: {:?}", error))
				}
			);
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
			return threadHandle.map_or_else
			(
				|| {
					// If there's no thread to join, then just return Ok.
					Ok(())
				},
				|thread|
				{
					// Ask the thread to join, and map any error it produces to our error types
					thread.join()
						.map_err(|error| eyre::eyre!("Error from playback thread: {:?}", error))
				}
			)
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
