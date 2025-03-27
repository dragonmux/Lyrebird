// SPDX-License-Identifier: BSD-3-Clause
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use color_eyre::eyre::{self, OptionExt, Result};
use libAudio::audioFile::AudioFile;

pub struct SongState
{
	title: String,
	duration: Option<Duration>,
	played: Duration,
	playbackThread: Option<JoinHandle<()>>,
	state: Arc<ThreadState>
}

#[derive(PartialEq, Eq)]
enum PlaybackState
{
	NotStarted,
	Playing,
	Paused,
	Stopped,
}

struct ThreadState
{
	audioFile: AudioFile,
	state: Mutex<PlaybackState>,
}

impl SongState
{
	pub fn from(fileName: &Path) -> Result<Self>
	{
		let audioFile = AudioFile::readFile(fileName)
			.ok_or_eyre(format!("Failed to open file {}", fileName.to_string_lossy()))?;
		let title = audioFile.fileInfo().title()?;
		let totalTime = audioFile.fileInfo().totalTime();

		Ok
		(
			Self
			{
				title,
				duration: if totalTime != 0 { Some(Duration::from_secs(totalTime)) } else { None },
				played: Duration::default(),
				playbackThread: None,
				state: Arc::new(ThreadState::from(audioFile)),
			}
		)
	}

	pub fn title(&self) -> String
	{
		self.title.clone()
	}

	pub fn songDuration(&self) -> Option<Duration>
	{
		self.duration
	}

	pub fn playedDuration(&self) -> Duration
	{
		self.played
	}

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

	pub fn pause(&mut self) -> Result<()>
	{
		// If we're in a playing state, pause playback
		let result = self.state.pause(self.playbackThread.take());
		self.playbackThread = None;
		result
	}

	pub fn stop(&mut self) -> Result<()>
	{
		// If we're in a playing state, stop playback
		let result = self.state.stop(self.playbackThread.take());
		self.playbackThread = None;
		result
	}
}

impl From<AudioFile> for ThreadState
{
	fn from(audioFile: AudioFile) -> Self
	{
		Self
		{
			audioFile,
			state: Mutex::new(PlaybackState::NotStarted),
		}
	}
}

impl ThreadState
{
	fn play(&self)
	{
		if !self.switchTo(PlaybackState::Playing)
		{
			self.audioFile.play();
		}
	}

	fn pause(&self, threadHandle: Option<JoinHandle<()>>) -> Result<()>
	{
		// See if we have any work to do
		if !self.switchTo(PlaybackState::Paused)
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
		if !self.switchTo(PlaybackState::Stopped)
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
		let mut lock = self.state.lock().expect("playback state mutex in invalid state");
		if lock.deref() != &newState
		{
			*lock = newState;
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
