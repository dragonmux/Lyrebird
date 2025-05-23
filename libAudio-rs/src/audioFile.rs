// SPDX-License-Identifier: BSD-3-Clause
use std::{ffi::CString, os::{raw::c_void, unix::ffi::OsStrExt}, path::Path, ptr::NonNull};

use crate::{fileInfo::FileInfo, AudioType};
use crate::bindings::
{
	audioCloseFile, audioGetFileInfo, audioOpenR, audioOpenW, audioPause, audioPlay, audioStop, isAudio
};

pub struct AudioFile
{
	inner: NonNull<c_void>,
}

impl AudioFile
{
	/// Try to open the given file as an audio file
	#[must_use]
	pub fn readFile(path: &Path) -> Option<AudioFile>
	{
		let fileName = path.to_path_buf();
		let fileName = fileName.as_os_str().as_bytes();
		let fileName = CString::new(fileName).ok()?;

		let file = unsafe { audioOpenR(fileName.as_ptr()) };
		Some(AudioFile { inner: NonNull::new(file)? })
	}

	/// Try to open the given file as an audio file
	#[must_use]
	pub fn writeFile(path: &Path, format: AudioType) -> Option<AudioFile>
	{
		let fileName = path.to_path_buf();
		let fileName = fileName.as_os_str().as_bytes();
		let fileName = CString::new(fileName).ok()?;

		let file = unsafe { audioOpenW(fileName.as_ptr(), format) };
		Some(AudioFile { inner: NonNull::new(file)? })
	}

	/// Check if the target file is a valid audio file
	#[must_use]
	pub fn isAudio(path: &Path) -> bool
	{
		let fileName = path.to_path_buf();
		let fileName = fileName.as_os_str().as_bytes();
		let fileName = CString::new(fileName);
		match fileName
		{
			Ok(fileName) => unsafe { isAudio(fileName.as_ptr()) }
			Err(_) => false
		}
	}

	/// Get the metadata for this audio file
	#[must_use]
	pub fn fileInfo(&self) -> FileInfo<'_>
	{
		FileInfo::new
		(
			unsafe { audioGetFileInfo(self.inner.as_ptr()) }
		)
	}

	/// Play the file back (resumes playback if previously played and returned from)
	pub fn play(&self)
	{
		unsafe { audioPlay(self.inner.as_ptr()) };
	}

	/// Pause the file playback (causes play to return)
	pub fn pause(&self)
	{
		unsafe { audioPause(self.inner.as_ptr()) };
	}

	/// Stop the file playback (causes play to return)
	pub fn stop(&self)
	{
		unsafe { audioStop(self.inner.as_ptr()) };
	}
}

impl Drop for AudioFile
{
	/// Clean up properly by disposing of the audio file object correctly
	fn drop(&mut self)
	{
		unsafe
		{
			audioCloseFile(self.inner.as_ptr());
		}
	}
}

unsafe impl Sync for AudioFile {}
unsafe impl Send for AudioFile {}
