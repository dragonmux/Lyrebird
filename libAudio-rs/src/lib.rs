// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]

use std::{ffi::CString, os::{raw::c_void, unix::ffi::OsStrExt}, path::Path, ptr::NonNull};

use bindings::{audioCloseFile, audioOpenR, isAudio};

mod bindings;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AudioFile
{
	inner: NonNull<c_void>,
}

impl AudioFile
{
	/// Try to open the given file as an audio file
	pub fn forPathRead(path: &Path) -> Option<AudioFile>
	{
		let fileName = path.to_path_buf();
		let fileName = fileName.as_os_str().as_bytes();
		let fileName = CString::new(fileName).ok()?;

		let file = unsafe { audioOpenR(fileName.as_ptr()) };
		Some(AudioFile { inner: NonNull::new(file)? })
	}

	/// Check if the target file is a valid audio file
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
