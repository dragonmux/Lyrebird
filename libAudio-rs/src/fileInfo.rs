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

impl<'a> FileInfo<'a>
{
	pub fn new(fileInfo: *const bindings::FileInfo) -> Self
	{
		FileInfo
		{
			inner: fileInfo,
			phantom: PhantomData,
		}
	}

	pub fn totalTime(&self) -> u64
	{
		unsafe { audioFileTotalTime(self.inner) }
	}

	pub fn bitsPerSample(&self) -> u32
	{
		unsafe { audioFileBitsPerSample(self.inner) }
	}

	pub fn bitRate(&self) -> u32
	{
		unsafe { audioFileBitRate(self.inner) }
	}

	pub fn channels(&self) -> u8
	{
		unsafe { audioFileChannels(self.inner) }
	}

	pub fn title(&self) -> Result<Option<String>>
	{
		let title = unsafe { audioFileTitle(self.inner) };
		match title.is_null()
		{
			true => Ok(None),
			false =>
			{
				let title = unsafe { CStr::from_ptr(title) };
				Ok(Some(String::from_utf8(title.to_bytes().to_vec())?))
			}
		}
	}

	pub fn artist(&self) -> Result<Option<String>>
	{
		let artist = unsafe { audioFileArtist(self.inner) };
		match artist.is_null()
		{
			true => Ok(None),
			false =>
			{
				let artist = unsafe { CStr::from_ptr(artist) };
				Ok(Some(String::from_utf8(artist.to_bytes().to_vec())?))
			}
		}
	}

	pub fn album(&self) -> Result<Option<String>>
	{
		let album = unsafe { audioFileAlbum(self.inner) };
		match album.is_null()
		{
			true => Ok(None),
			false =>
			{
				let album = unsafe { CStr::from_ptr(album) };
				Ok(Some(String::from_utf8(album.to_bytes().to_vec())?))
			}
		}
	}

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
