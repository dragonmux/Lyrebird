// SPDX-License-Identifier: BSD-3-Clause
use std::{ffi::CStr, marker::PhantomData, ops::Range};

use color_eyre::eyre::Result;

use crate::{audioFile::{self, AudioFile}, bindings::{self, audioFileAlbum, audioFileArtist, audioFileBitRate, audioFileBitsPerSample, audioFileChannels, audioFileOtherComment, audioFileOtherCommentsCount, audioFileTitle}};
use crate::bindings::audioFileTotalTime;

pub struct FileInfo<'a>
{
	inner: *const bindings::FileInfo,
	phantom: PhantomData<&'a audioFile::AudioFile>,
}

impl FileInfo<'_>
{
	#[must_use]
	pub fn new(fileInfo: *const bindings::FileInfo) -> Self
	{
		FileInfo
		{
			inner: fileInfo,
			phantom: PhantomData,
		}
	}

	#[must_use]
	pub fn totalTime(&self) -> u64
	{
		unsafe { audioFileTotalTime(self.inner) }
	}

	#[must_use]
	pub fn bitsPerSample(&self) -> u32
	{
		unsafe { audioFileBitsPerSample(self.inner) }
	}

	#[must_use]
	pub fn bitRate(&self) -> u32
	{
		unsafe { audioFileBitRate(self.inner) }
	}

	#[must_use]
	pub fn channels(&self) -> u8
	{
		unsafe { audioFileChannels(self.inner) }
	}

	/// # Errors
	/// Fails if the track title is not valid UTF-8.
	pub fn title(&self) -> Result<Option<String>>
	{
		let title = unsafe { audioFileTitle(self.inner) };
		if title.is_null()
		{
			return Ok(None);
		}
		let title = unsafe { CStr::from_ptr(title) };
		Ok(Some(String::from_utf8(title.to_bytes().to_vec())?))
	}

	/// # Errors
	/// Fails if the track artist is not valid UTF-8.
	pub fn artist(&self) -> Result<Option<String>>
	{
		let artist = unsafe { audioFileArtist(self.inner) };
		if artist.is_null()
		{
			return Ok(None);
		}
		let artist = unsafe { CStr::from_ptr(artist) };
		Ok(Some(String::from_utf8(artist.to_bytes().to_vec())?))
	}

	/// # Errors
	/// Fails if the album title is not valid UTF-8.
	pub fn album(&self) -> Result<Option<String>>
	{
		let album = unsafe { audioFileAlbum(self.inner) };
		if album.is_null()
		{
			return Ok(None);
		}
		let album = unsafe { CStr::from_ptr(album) };
		Ok(Some(String::from_utf8(album.to_bytes().to_vec())?))
	}

	/// # Errors
	/// Fails if other comments are not valid UTF-8.
	pub fn otherComments(&self) -> Result<Vec<String>>
	{
		let count = unsafe { audioFileOtherCommentsCount(self.inner) };
		let mut comments = Vec::with_capacity(count);
		let indexes = Range{ start: 0, end: count };
		for idx in indexes
		{
			let comment = unsafe { CStr::from_ptr(audioFileOtherComment(self.inner, idx)) };
			comments.push(String::from_utf8(comment.to_bytes().to_vec())?);
		}
		Ok(comments)
	}
}

impl<'a> From<&'a AudioFile> for FileInfo<'a>
{
	fn from(audioFile: &'a AudioFile) -> Self
	{
		audioFile.fileInfo()
	}
}
