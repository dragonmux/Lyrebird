// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use color_eyre::eyre::{self, Result};

use crate::library::MusicLibrary;
use crate::window::Operation;

pub struct LibraryTree
{
	activeSide: Side,
	library: Arc<RwLock<MusicLibrary>>,
}

#[derive(Clone, Copy)]
enum Side
{
	DirectoryTree,
	Files,
}

impl LibraryTree
{
	pub fn new(cacheFile: &Path, libraryPath: &Path) -> Result<Self>
	{
		Ok(Self
		{
			activeSide: Side::DirectoryTree,
			library: MusicLibrary::new(cacheFile, libraryPath)?,
		})
	}

	pub fn writeCache(&self) -> Result<()>
	{
		self.library.read()
			.map_err
			(
				|error|
					eyre::eyre!("While writing library cache: {}", error.to_string())
			)?
			.writeCache()
	}

	pub fn isDiscovering(&self) -> bool
	{
		self.library.read().expect("Library lock in bad state").isDiscovering()
	}

	pub async fn maybeJoinDiscovery(&self) -> Result<()>
	{
		MusicLibrary::maybeJoinDiscoveryThread(&self.library).await
	}

	const fn moveLeft(&mut self)
		{ self.activeSide = Side::DirectoryTree; }

	const fn moveRight(&mut self)
		{ self.activeSide = Side::Files; }

	fn moveUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	fn moveDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	fn movePageUp(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	fn movePageDown(&mut self)
	{
		match self.activeSide
		{
			Side::DirectoryTree =>
			{
			}
			Side::Files =>
			{
			}
		}
	}

	/// If the currently sellected side is the directory listing, switch to that directory's file listing
	/// otherwise, if it's the file listing, figure out which one and make a `SongState` for it
	fn makeSelection(&mut self) -> Option<PathBuf>
	{
		match self.activeSide
		{
			Side::DirectoryTree => self.activeSide = Side::Files,
			Side::Files =>
			{
			}
		}
		None
	}

	fn playSelection(&mut self) -> Operation
	{
		let selection = self.makeSelection();
		match selection
		{
			Some(selection) => Operation::Play(selection),
			None => Operation::None,
		}
	}
}
