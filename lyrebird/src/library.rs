// SPDX-License-Identifier: BSD-3-Clause
use std::collections::BTreeMap;
// use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use color_eyre::eyre::{self, Result, eyre};
use libAudio::audioFile::AudioFile;
use tracing::{error, info};

use crate::cache;
use crate::messages::Message;
use crate::track::{Album, AlbumID, Artist, ArtistID, Track, TrackID};

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

	nextDirectoryID: AtomicU64,
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

			nextDirectoryID: AtomicU64::default(),
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
				// Reset the next directory and track IDs back to 0
				library.nextDirectoryID.store(0, Ordering::Release);
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
			Ok(_) =>
			{
				match library.readLock()
				{
					Ok(library) =>
					{
						info!
						(
							"Discovered {} directories and {} tracks total",
							library.dirs.len(),
							library.tracks.len()
						);
						Message::LibraryDiscovered
					}
					Err(error) =>
					{
						error!("Failed to lock library to show discovery results from");
						error!("{}", error);
						Message::ConcurrencyError
					}
				}
			},
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
		let directoryID = Self::addDirectory(library, None, directory)?;

		// Run discovery now we have a directory object in the dirs map to work on
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
				// Turn this directory into an entry in the dirs map and run discovery for it
				let directoryID = Self::addDirectory(library, Some(currentDirectory), &path)?;
				Self::discoverDirectory(library, directoryID)?;
				// Having done discovery on this directory, now check to see if it contains any music
				// if it doesn't, then prune it back out the tree
				let mut library = library.writeLock()?;
				let directory = library.directoryFor(directoryID);
				if directory.contents().len() == 0 && directory.subdirs().len() == 0
				{
					library.dirs.remove(&directoryID);
					library.mutDirectoryFor(currentDirectory).removeSubdir(directoryID);
				}
			}
			// Else if it's a file, see if it's audio
			else if let Some(file) = AudioFile::readFile(&path)
			{
				// Grab a lock on the library for the next few ops
				let mut library = library.writeLock()?;

				// Convert the file into a track and insert it into the available track list
				let trackID = library.nextTrackID.fetch_add(1, Ordering::AcqRel);
				let track = Track::new(file, trackID, &mut library)?;
				// Add the track to its containing directory while we still can
				let directory = library.mutDirectoryFor(currentDirectory);
				directory.addTrack(track.id());
				library.tracks.insert(track.id(), track);
			}
		}

		Ok(())
	}

	// Add a directory to a specific parent directory
	fn addDirectory(library: &RwLock<Self>, parentID: Option<DirectoryID>, path: &Path) -> Result<DirectoryID>
	{
		// Grab a lock on the library to add the directory entry with
		let mut library = library.writeLock()?;
		// Extract the ID this new directory will have
		let directoryID = library.nextDirectoryID.fetch_add(1, Ordering::AcqRel);
		// Make the directory object and insert it
		let directory = Directory::from_path(directoryID, path);
		let directoryID = directory.id();
		library.dirs.insert(directoryID, directory);
		// Grab back the parent directory, if there is one
		if let Some(parentID) = parentID
		{
			let parent = library.mutDirectoryFor(parentID);
			// Add this subdirectory to the parent dir
			parent.addSubdir(directoryID);
		}
		// We're all done now, so return the new ID
		Ok(directoryID)
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

	pub fn directories(&self) -> &BTreeMap<DirectoryID, Directory>
	{
		&self.dirs
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

	pub fn removeSubdir(&mut self, directoryID: DirectoryID)
	{
		if let Some(index) = self.subdirectories.iter().position(|&entryID| entryID == directoryID)
		{
			self.subdirectories.remove(index);
		}
	}

	pub fn path(&self) -> &Path
	{
		&self.path
	}

	pub fn contents(&self) -> &[TrackID]
	{
		&self.tracks
	}

	pub fn subdirs(&self) -> &[DirectoryID]
	{
		&self.subdirectories
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
