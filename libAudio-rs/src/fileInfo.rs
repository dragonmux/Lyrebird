// SPDX-License-Identifier: BSD-3-Clause
use std::marker::PhantomData;

use crate::{audioFile::{self, AudioFile}, bindings};

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
}

impl<'a> From<AudioFile> for FileInfo<'a>
{
	fn from(audioFile: AudioFile) -> Self
	{
		audioFile.fileInfo()
	}
}
