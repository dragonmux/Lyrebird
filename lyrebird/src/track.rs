// SPDX-License-Identifier: BSD-3-Clause

use std::{borrow::Cow, path::PathBuf};

use color_eyre::eyre::Result;
use libAudio::audioFile::AudioFile;

use crate::library::MusicLibrary;

/// Represents a track in a music library
pub struct Track
{
	/// Unique 64-bit identifier for the track
	id: u64,
	/// Path to the file holding this track
	fileName: PathBuf,
	/// Total length of the track in seconds
	totalLength: u64,
	/// Title of the track
	title: String,
	/// ID of the artist for the track (if there is one)
	artist: Option<ArtistID>,
	/// ID of the album for the track (if there is one)
	album: Option<AlbumID>
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Unique strongly typed identifier for a track
pub struct TrackID(u64);

/// Represents the artist for some tracks in a music library
pub struct Artist
{
	name: String,
	tracks: Vec<TrackID>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Unique strongly typed identifier for an artist
pub struct ArtistID(u64);

/// Represents an album of some tracks in a music library
pub struct Album
{
	name: String,
	tracks: Vec<TrackID>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Unique strongly typed identifier for an album
pub struct AlbumID(u64);

impl Track
{
	/// Create a new Track from an AudioFile with a given ID
	pub fn new(file: AudioFile, trackID: u64, library: &mut MusicLibrary) -> Result<Self>
	{
		// Extract the file's metadata
		let fileInfo = file.fileInfo();
		// Try and construct a title for the track
		let title = fileInfo
			.title()?
			// If it doesn't have a title, try to use the file name
			.unwrap_or_else(||
				{
					let fileName = file
						.path()
						.file_name()
						// If there isn't a file name (??) then use the full path
						.unwrap_or_else(|| file.path().as_os_str());
					fileName.to_string_lossy().to_string()
				}
			);

		// Look up the track's artist
		let artist = fileInfo
			.artist()?
			.map(|artistName| library.lookupArtist(&artistName));

		// Look up the track's album
		let album = fileInfo
			.album()?
			.map(|albumName| library.lookupAlbum(&albumName));

		// Create the complete Track object and return
		let track = Self
		{
			id: trackID,
			fileName: file.path().to_path_buf(),
			totalLength: fileInfo.totalTime(),
			title,
			artist,
			album,
		};

		// Add the track to the artist it's made by
		artist.map(|artistID| library.mutArtistFor(artistID).addTrack(track.id()));
		// Add the track to the album it's from
		album.map(|albumID| library.mutAlbumFor(albumID).addTrack(track.id()));

		Ok(track)
	}

	pub fn fromCache
	(
		trackID: u64,
		fileName: PathBuf,
		totalLength: u64,
		title: String,
		artist: Option<ArtistID>,
		album: Option<AlbumID>
	) -> Self
	{
		Self
		{
			id: trackID,
			fileName,
			totalLength,
			title,
			artist,
			album,
		}
	}

	pub fn id(&self) -> TrackID
	{
		TrackID(self.id)
	}

	pub fn fileName(&self) -> Cow<'_, str>
	{
		self.fileName.file_name().unwrap_or_else(|| self.fileName.as_os_str()).to_string_lossy()
	}

	pub fn totalLength(&self) -> u64
	{
		self.totalLength
	}

	pub fn title(&self) -> &str
	{
		&self.title
	}

	pub fn artistID(&self) -> Option<ArtistID>
	{
		self.artist
	}

	pub fn albumID(&self) -> Option<AlbumID>
	{
		self.album
	}

	pub fn audioFile(&self) -> Option<AudioFile>
	{
		AudioFile::readFile(&self.fileName)
	}
}

impl From<TrackID> for u64
{
	fn from(id: TrackID) -> Self
	{
		id.0
	}
}

impl Artist
{
	pub fn new(artistName: &str) -> Self
	{
		Self
		{
			name: artistName.to_string(),
			tracks: Vec::new(),
		}
	}

	pub fn addTrack(&mut self, track: TrackID)
	{
		self.tracks.push(track);
	}

	pub fn name(&self) -> &str
	{
		&self.name
	}
}

impl ArtistID
{
	/// Construct a new ArtistID with a specific ID value
	pub fn new(id: u64) -> Self
	{
		Self(id)
	}

	/// Construct a new ArtistID with the next value to this one
	pub fn next(&self) -> Self
	{
		Self(self.0 + 1)
	}
}

impl From<ArtistID> for u64
{
	fn from(id: ArtistID) -> Self
	{
		id.0
	}
}

impl From<&ArtistID> for u64
{
	fn from(id: &ArtistID) -> Self
	{
		id.0
	}
}

impl Album
{
	pub fn new(albumName: &str) -> Self
	{
		Self
		{
			name: albumName.to_string(),
			tracks: Vec::new(),
		}
	}

	pub fn addTrack(&mut self, track: TrackID)
	{
		self.tracks.push(track);
	}

	pub fn name(&self) -> &str
	{
		&self.name
	}
}

impl AlbumID
{
	/// Construct a new AlbumID with a specific ID value
	pub fn new(id: u64) -> Self
	{
		Self(id)
	}

	/// Construct a new AlbumID with the next value to this one
	pub fn next(&self) -> Self
	{
		Self(self.0 + 1)
	}
}

impl From<AlbumID> for u64
{
	fn from(id: AlbumID) -> Self
	{
		id.0
	}
}

impl From<&AlbumID> for u64
{
	fn from(id: &AlbumID) -> Self
	{
		id.0
	}
}
