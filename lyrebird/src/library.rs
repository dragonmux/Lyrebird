// SPDX-License-Identifier: BSD-3-Clause
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{create_dir_all, File};
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::iter;

use color_eyre::eyre::{self, OptionExt, Result};
use libAudio::audioFile::AudioFile;
use serde::{Deserialize, Serialize};
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::track::{Album, AlbumID, Artist, ArtistID, Track, TrackID};

#[derive(Serialize, Deserialize)]
pub struct MusicLibrary
{
	/// Root of this music library
	basePath: PathBuf,
	/// Path to where to cache the library
	#[serde(skip)]
	cacheFile: PathBuf,
	/// Paths to directories containing music relative to the root
	dirs: BTreeSet<PathBuf>,
	/// Map of directories to a list of files in that directory which are music
	files: BTreeMap<PathBuf, BTreeSet<PathBuf>>,

	#[serde(skip)]
	tracks: BTreeMap<TrackID, Track>,
	#[serde(skip)]
	artists: BTreeMap<ArtistID, Artist>,
	#[serde(skip)]
	albums: BTreeMap<AlbumID, Album>,

	#[serde(skip)]
	discoveryThread: Option<JoinHandle<Result<()>>>,
	#[serde(skip)]
	discoveryCancellation: CancellationToken,

	#[serde(skip)]
	nextTrackID: AtomicU64,
}

impl MusicLibrary
{
	pub fn new(cacheFile: &Path, basePath: &Path) -> Result<Arc<RwLock<Self>>>
	{
		if cacheFile.exists()
		{
			Self::fromCache(cacheFile)
				.or_else
				(
					|report|
					{
						error!("Reading library cache failed: {}", report);
						Self::fromPath(cacheFile, basePath)
					}
				)
		}
		else
		{
			Self::fromPath(cacheFile, basePath)
		}
	}

	/// Construct a library from a cache JSON
	pub fn fromCache(cacheFile: &Path) -> Result<Arc<RwLock<Self>>>
	{
		let cache = File::open(cacheFile)?;
		let mut library: Self = serde_json::from_reader(cache)?;
		library.cacheFile = cacheFile.to_path_buf();
		Ok(Arc::new(RwLock::new(library)))
	}

	/// Construct a library from a new base path
	pub fn fromPath(cacheFile: &Path, basePath: &Path) -> Result<Arc<RwLock<Self>>>
	{
		if !basePath.is_dir()
		{
			return Err(eyre::eyre!("Library path must be a valid directory"));
		}

		let library = Arc::new
		(
			RwLock::new
			(
				Self
				{
					basePath: basePath.to_path_buf(),
					cacheFile: cacheFile.to_path_buf(),
					dirs: BTreeSet::new(),
					files: BTreeMap::new(),

					tracks: BTreeMap::new(),
					artists: BTreeMap::new(),
					albums: BTreeMap::new(),

					discoveryThread: None,
					discoveryCancellation: CancellationToken::new(),

					nextTrackID: AtomicU64::default(),
				}
			)
		);

		// Self::backgroundDiscover(&library, library.clone(), basePath.to_path_buf())?;

		Ok(library)
	}

	pub fn writeCache(&self) -> Result<()>
	{
		// Ask our discovery task to stop if it didn't already
		self.discoveryCancellation.cancel();
		// Make sure all the leading path elements exist
		create_dir_all
		(
			self.cacheFile.parent()
				.ok_or_eyre("Failed to extract the path to the music library cache file")?
		)?;
		// Open the cache file for writing
		let cache = File::create(&self.cacheFile)?;
		// Ask serde to serialise out the library cache
		Ok(serde_json::to_writer(cache, self)?)
	}

	pub fn isDiscovering(&self) -> bool
	{
		match &self.discoveryThread
		{
			Some(thread) => !thread.is_finished(),
			None => false,
		}
	}

	pub async fn maybeJoinDiscoveryThread(library: &Arc<RwLock<Self>>) -> Result<()>
	{
		if Self::readLock(library)?.discoveryThread.is_some()
		{
			let thread = Self::writeLock(library)?.discoveryThread.take()
				.ok_or(eyre::eyre!("Inconsistency in discovery thread state"))?;
			return thread.await?;
		}
		Ok(())
	}

	fn backgroundDiscover(localLibrary: &Arc<RwLock<Self>>, library: Arc<RwLock<Self>>, currentDirectory: PathBuf) -> Result<()>
	{
		let task = async move
		{
			Self::discover(library.as_ref(), currentDirectory.as_path())
		};

		let mut library = Self::writeLock(localLibrary)?;
		library.discoveryThread = Some(spawn(task));
		Ok(())
	}

	fn writeLock(library: &RwLock<Self>) -> Result<RwLockWriteGuard<'_, Self>>
	{
		library.write()
			.map_err
			(
				|error| eyre::eyre!("While discovering library: {}", error)
			)
	}

	fn readLock(library: &RwLock<Self>) -> Result<RwLockReadGuard<'_, Self>>
	{
		library.read()
			.map_err
			(
				|error| eyre::eyre!("While discovering library: {}", error)
			)
	}

	fn discover(library: &RwLock<Self>, currentDirectory: &Path) -> Result<()>
	{
		// Explore the current directory's contents
		let contents = currentDirectory.read_dir()?;
		// For each entry in it
		for entry in contents
		{
			// Get the path to that entry
			let path = entry?.path();
			// If it's a directory, add it to the set discovered and recurse
			if path.is_dir()
			{
				let relativePath = path.strip_prefix(&Self::readLock(library)?.basePath)?.to_path_buf();
				Self::writeLock(library)?.dirs.insert(relativePath.clone());
				Self::discover(library, &path)?;
				// Well, only add it to the directories set if there were any audio files for us or one
				// of the subdirectories within (which would mean that subdirectory is in the dirs set)
				if !Self::readLock(library)?.files.contains_key(&path) &&
					!Self::readLock(library)?.dirs.iter().any
					(
						|dir| dir.starts_with(&relativePath) && dir != &relativePath
					)
				{
					// In the case that we actually don't have anything for this directory, remove it again
					Self::writeLock(library)?.dirs.remove(&relativePath);
				}
			}
			// Else if it's a file, see if it's audio
			else if let Some(file) = AudioFile::readFile(&path) {
				// Grab a lock on the library for the next few ops
				let mut library = Self::writeLock(library)?;

				// See if this file's directory is already in the map
				let filePath = path.parent()
					.ok_or_eyre("File does not have a valid path parent")?;
				if !library.files.contains_key(filePath)
				{
					library.files.insert(filePath.to_path_buf(), BTreeSet::new());
				}
				// Now we definitely have a vec to use, look the path up and add the file
				library.files.get_mut(filePath)
					.ok_or_eyre("Failed to look file's path up in file map")?
					.insert(path);

				// Convert the file into a track and insert it into the available track list
				let trackID = library.nextTrackID
					.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
				let track = Track::new(file, trackID, library.deref_mut())?;
				library.tracks.insert(track.id(), track);
			}
			// If we're being asked to stop, stop
			if Self::readLock(library)?.discoveryCancellation.is_cancelled()
			{
				break
			}
		}

		// We done? good!
		Ok(())
	}

	pub fn directoryCount(&self) -> usize
		{ self.dirs.len() + 1 }

	pub fn filesCount(&self, dirIndex: Option<usize>) -> usize
	{
		dirIndex
			.and_then(|index| iter::once(&self.basePath).chain(self.dirs.iter()).nth(index))
			.and_then(|dir| self.filesIn(dir))
			.map(BTreeSet::len)
			.unwrap_or_default()
	}

	pub fn directoryAt(&self, index: usize) -> Option<&PathBuf>
	{
		iter::once(&self.basePath)
			.chain(self.dirs.iter())
			.nth(index)
	}

	pub fn fileIn(&self, dir: &PathBuf, index: usize) -> Option<&PathBuf>
	{
		let files = self.filesIn(dir)?;
		files.iter().nth(index)
	}

	fn filesIn(&self, dir: &PathBuf) -> Option<&BTreeSet<PathBuf>>
	{
		if dir.is_relative()
		{
			let path = self.basePath.join(dir);
			self.files.get(&path)
		}
		else
		{
			self.files.get(dir)
		}
	}

	/// Look up an artist by name to get an ArtistID
	pub fn lookupArtist(&mut self, artistName: &str) -> ArtistID
	{
		// See if we can locate a given artist in the list
		for (id, artist) in &self.artists
		{
			if artist.name() == artistName
			{
				return *id
			}
		}
		// If not, then construct a new one
		let artistID = self.artists
			.last_key_value()
			.map_or_else(|| ArtistID::new(0), |(id, _)| id.next());
		self.artists.insert(artistID, Artist::new(artistName));
		artistID
	}

	/// Look up an album by name to get an AlbumID
	pub fn lookupAlbum(&mut self, albumName: &str) -> AlbumID
	{
		// See if we can locate a given artist in the list
		for (id, artist) in &self.albums
		{
			if artist.name() == albumName
			{
				return *id
			}
		}
		// If not, then construct a new one
		let albumID = self.albums
			.last_key_value()
			.map_or_else(|| AlbumID::new(0), |(id, _)| id.next());
		self.albums.insert(albumID, Album::new(albumName));
		albumID
	}

	/// Find the Artist object associated with a particular ArtistID and return it by reference
	pub fn artistFor(&self, artistID: ArtistID) -> &Artist
	{
		&self.artists[&artistID]
	}

	/// Find the Artist object associated with a particular ArtistID and return a mutable reference to it
	pub fn mutArtistFor(&mut self, artistID: ArtistID) -> &mut Artist
	{
		// This is safe because it's impossible to get an ArtistID that's not valid
		unsafe { self.artists.get_mut(&artistID).unwrap_unchecked() }
	}

	/// Find the Album object associated with a particular AlbumID and return it by reference
	pub fn albumFor(&self, albumID: AlbumID) -> &Album
	{
		&self.albums[&albumID]
	}

	/// Find the Album object associated with a particular AlbumID and return a mutable reference to it
	pub fn mutAlbumFor(&mut self, albumID: AlbumID) -> &mut Album
	{
		// This is safe because it's impossible to get an AlbumID that's not valid
		unsafe { self.albums.get_mut(&albumID).unwrap_unchecked() }
	}
}
