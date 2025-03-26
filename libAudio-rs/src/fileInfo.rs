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

	pub fn title(&self) -> Result<String>
	{
		let title = unsafe { CStr::from_ptr(audioFileTitle(self.inner)) };
		Ok(String::from_utf8(title.to_bytes().to_vec())?)
	}

	pub fn artist(&self) -> Result<String>
	{
		let artist = unsafe { CStr::from_ptr(audioFileArtist(self.inner)) };
		Ok(String::from_utf8(artist.to_bytes().to_vec())?)
	}

	pub fn album(&self) -> Result<String>
	{
		let album = unsafe { CStr::from_ptr(audioFileAlbum(self.inner)) };
		Ok(String::from_utf8(album.to_bytes().to_vec())?)
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

impl<'a> From<AudioFile> for FileInfo<'a>
{
	fn from(audioFile: AudioFile) -> Self
	{
		audioFile.fileInfo()
	}
}
