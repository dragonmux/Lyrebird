// SPDX-License-Identifier: BSD-3-Clause
use std::collections::BTreeMap;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use color_eyre::eyre::{self, Result, eyre};
use libAudio::audioFile::AudioFile;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::cache;
use crate::messages::Message;
use crate::track::{Album, AlbumID, Artist, ArtistID, Track, TrackID};
use crate::widgets::treeView::TreeItem;

/// Represents a music library
pub struct MusicLibrary
{
	/// Root of this music library
	basePath: PathBuf,
	/// Path to where to cache the library
	cacheFile: PathBuf,

	/// Map of directory IDs to directories in the library
	dirs: BTreeMap<DirectoryID, Directory>,
	/// Map of track IDs to tracks in the library
	tracks: BTreeMap<TrackID, Track>,
	/// Map of artist IDs to artists in the library
	artists: BTreeMap<ArtistID, Artist>,
	/// Map of album IDs to albums in the library
	albums: BTreeMap<AlbumID, Album>,

	discoveryCancellation: CancellationToken,

	nextTrackID: AtomicU64,
}

/// Represents a directory in a music library
pub struct Directory
{
	/// Unique 64-bit identifier for the directory
	id: u64,
	/// Filesystem path for this directory
	path: PathBuf,
	/// List of directories this directory immediately contains
	subdirectories: Vec<DirectoryID>,
	/// List of tracks this directory immediately contains
	tracks: Vec<TrackID>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DirectoryID(u64);

trait Lockable<T>
{
	fn readLock(&self) -> Result<RwLockReadGuard<'_, T>>;
	fn writeLock(&self) -> Result<RwLockWriteGuard<'_, T>>;
}

impl MusicLibrary
{
	pub fn new(cacheFile: &Path, basePath: &Path) -> Self
	{
		Self
		{
			basePath: basePath.to_path_buf(),
			cacheFile: cacheFile.to_path_buf(),

			dirs: BTreeMap::new(),
			tracks: BTreeMap::new(),
			artists: BTreeMap::new(),
			albums: BTreeMap::new(),

			discoveryCancellation: CancellationToken::new(),

			nextTrackID: AtomicU64::default(),
		}
	}

	pub async fn load(library: Arc<RwLock<Self>>) -> Message
	{
		match library.write()
		{
			Ok(mut library) => match library.fromCache()
			{
				Ok(_) => Message::LibraryLoaded,
				Err(error) =>
				{
					error!("Failed to load library from cache");
					error!("{}", error);
					info!("Trying to re-discover library");
					Message::LibraryDiscover
				}
			},
			Err(error) =>
			{
				error!("Failed to lock library to read cache into");
				error!("{}", error);
				Message::ConcurrencyError
			},
		}
	}

	// Try to load the library from a library cache file
	fn fromCache(&mut self) -> Result<()>
	{
		// If the cache file doesn't exist, bail
		if !self.cacheFile.try_exists()?
		{
			return Err(eyre!("Cache file {} does not exist", self.cacheFile.display()));
		}
		// Try load the cache and convert it into a set of library maps
		let library = cache::loadLibrary(&self.cacheFile)?;
		let maps = library.to_maps()?;

		// Move the maps into this library
		self.dirs = maps.dirs;
		self.tracks = maps.tracks;
		self.artists = maps.artists;
		self.albums = maps.albums;

		// Update the next track ID value to the one for the loaded library
		self.nextTrackID.store(maps.nextTrackID.unwrap_or_default(), Ordering::Release);

		Ok(())
	}

	pub async fn discover(library: Arc<RwLock<Self>>) -> Message
	{
		// Prepare the library for (re-)discovery
		let directory = match library.write()
		{
			Ok(mut library) =>
			{
				info!("Preparing library for discovery");
				// Create brand new maps to discover the library into
				library.tracks = BTreeMap::new();
				library.dirs = BTreeMap::new();
				library.artists = BTreeMap::new();
				library.albums = BTreeMap::new();
				// Reset the next track ID back to 0
				library.nextTrackID.store(0, Ordering::Release);
				info!("Running discovery in {}", library.basePath.display());
				library.basePath.clone()
			},
			Err(error) =>
			{
				error!("Failed to lock library to run discovery into");
				error!("{}", error);
				return Message::ConcurrencyError;
			},
		};
		// Now actually run the discovery process
		match MusicLibrary::recursiveDiscover(&library, &directory)
		{
			Ok(_) => Message::LibraryDiscovered,
			Err(error) =>
			{
				error!("Failed to discover library from {}", directory.display());
				error!("{}", error);
				Message::LibraryError
			}
		}
	}

	fn recursiveDiscover(library: &RwLock<Self>, directory: &Path) -> Result<()>
	{
		// If the base path is not valid, abort
		if !directory.try_exists()?
		{
			return Err(eyre::eyre!("Library root path must be a valid directory"));
		}

		// Turn this base directory into a root directory entry and add it to the library as the base directory
		let directory = Directory::from_path(0, directory);
		let directoryID = directory.id();
		library.writeLock()?.dirs.insert(directoryID, directory);

		Self::discoverDirectory(library, directoryID)
	}

	fn discoverDirectory(library: &RwLock<Self>, currentDirectory: DirectoryID) -> Result<()>
	{
		// Explore the current directory's contents
		let contents = library.readLock()?.directoryFor(currentDirectory).path().read_dir()?;
		// For each entry in the directory
		for entry in contents
		{
			// Extract the path for it
			let path = entry?.path();
			// If the entry is a directory
			if path.is_dir()
			{
				//
			}
			// Else if it's a file, see if it's audio
			else if let Some(file) = AudioFile::readFile(&path)
			{
				// Grab a lock on the library for the next few ops
				let mut library = library.writeLock()?;

				// Convert the file into a track and insert it into the available track list
				let trackID = library.nextTrackID
					.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
				let track = Track::new(file, trackID, &mut library)?;
				// Add the track to its containing directory while we still can
				let directory = library.mutDirectoryFor(currentDirectory);
				directory.addTrack(track.id());
				library.tracks.insert(track.id(), track);
			}
		}

		Ok(())
	}

	// pub fn writeCache(&self) -> Result<()>
	// {
	// 	// Ask our discovery task to stop if it didn't already
	// 	self.discoveryCancellation.cancel();
	// 	// Make sure all the leading path elements exist
	// 	create_dir_all
	// 	(
	// 		self.cacheFile.parent()
	// 			.ok_or_eyre("Failed to extract the path to the music library cache file")?
	// 	)?;
	// 	// Open the cache file for writing
	// 	let cache = File::create(&self.cacheFile)?;
	// 	// Ask serde to serialise out the library cache
	// 	Ok(serde_json::to_writer(cache, self)?)
	// }

	// fn discover(library: &RwLock<Self>, currentDirectory: &Path) -> Result<()>
	// {
	// 	// Explore the current directory's contents
	// 	let contents = currentDirectory.read_dir()?;
	// 	// For each entry in it
	// 	for entry in contents
	// 	{
	// 		// Get the path to that entry
	// 		let path = entry?.path();
	// 		// If it's a directory, add it to the set discovered and recurse
	// 		if path.is_dir()
	// 		{
	// 			let relativePath = path.strip_prefix(&Self::readLock(library)?.basePath)?.to_path_buf();
	// 			Self::writeLock(library)?.dirs.insert(relativePath.clone());
	// 			Self::discover(library, &path)?;
	// 			// Well, only add it to the directories set if there were any audio files for us or one
	// 			// of the subdirectories within (which would mean that subdirectory is in the dirs set)
	// 			if !Self::readLock(library)?.files.contains_key(&path) &&
	// 				!Self::readLock(library)?.dirs.iter().any
	// 				(
	// 					|dir| dir.starts_with(&relativePath) && dir != &relativePath
	// 				)
	// 			{
	// 				// In the case that we actually don't have anything for this directory, remove it again
	// 				Self::writeLock(library)?.dirs.remove(&relativePath);
	// 			}
	// 		}
	// 		// If we're being asked to stop, stop
	// 		if Self::readLock(library)?.discoveryCancellation.is_cancelled()
	// 		{
	// 			break
	// 		}
	// 	}

	// 	// We done? good!
	// 	Ok(())
	// }

	// pub fn filesCount(&self, dirIndex: Option<usize>) -> usize
	// {
	// 	dirIndex
	// 		.and_then(|index| iter::once(&self.basePath).chain(self.dirs.iter()).nth(index))
	// 		.and_then(|dir| self.filesIn(dir))
	// 		.map(BTreeSet::len)
	// 		.unwrap_or_default()
	// }

	// pub fn directoryAt(&self, index: usize) -> Option<&PathBuf>
	// {
	// 	iter::once(&self.basePath)
	// 		.chain(self.dirs.iter())
	// 		.nth(index)
	// }

	// pub fn fileIn(&self, dir: &PathBuf, index: usize) -> Option<&PathBuf>
	// {
	// 	let files = self.filesIn(dir)?;
	// 	files.iter().nth(index)
	// }

	// fn filesIn(&self, dir: &PathBuf) -> Option<&BTreeSet<PathBuf>>
	// {
	// 	if dir.is_relative()
	// 	{
	// 		let path = self.basePath.join(dir);
	// 		self.files.get(&path)
	// 	}
	// 	else
	// 	{
	// 		self.files.get(dir)
	// 	}
	// }

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

	/// Find the Directory object associated with a particular DirectoryID and return it by reference
	pub fn directoryFor(&self, directoryID: DirectoryID) -> &Directory
	{
		&self.dirs[&directoryID]
	}

	/// Find the Directory object associated with a particular DirectoryID and return a mutable reference to it
	pub fn mutDirectoryFor(&mut self, directoryID: DirectoryID) -> &mut Directory
	{
		// This is safe because it's impossible to get an DirectoryID that's not valid
		unsafe { self.dirs.get_mut(&directoryID).unwrap_unchecked() }
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

impl Lockable<MusicLibrary> for RwLock<MusicLibrary>
{
	fn readLock(&self) -> Result<RwLockReadGuard<'_, MusicLibrary>>
	{
		self.read()
			.map_err
			(
				|error| eyre::eyre!("Failed to read lock library: {}", error)
			)
	}

	fn writeLock(&self) -> Result<RwLockWriteGuard<'_, MusicLibrary>>
	{
		self.write()
			.map_err
			(
				|error| eyre::eyre!("Failed to write lock library: {}", error)
			)
	}
}

impl Directory
{
	pub fn from_path(id: u64, path: &Path) -> Self
	{
		Self
		{
			id,
			path: path.into(),
			subdirectories: Vec::new(),
			tracks: Vec::new()
		}
	}

	pub fn id(&self) -> DirectoryID
	{
		DirectoryID(self.id)
	}

	pub fn addSubdir(&mut self, directoryID: DirectoryID)
	{
		self.subdirectories.push(directoryID);
	}

	pub fn addTrack(&mut self, trackID: TrackID)
	{
		self.tracks.push(trackID);
	}

	pub fn path(&self) -> &Path
	{
		&self.path
	}

	pub fn contents(&self) -> &[TrackID]
	{
		&self.tracks
	}
}

impl TreeItem for Directory
{
	fn displayName(&self) -> String
	{
		if self.id == 0
		{
			self.path.as_os_str()
		}
		else
		{
			self.path.file_name().unwrap_or_else(|| self.path.as_os_str())
		}
			.to_string_lossy()
			.to_string()
	}

	fn children(&self) -> &[&Self]
	{
		&[]
	}
}

impl DirectoryID
{
	/// Construct a new DirectoryID with a specific ID value
	pub fn new(id: u64) -> Self
	{
		Self(id)
	}
}
