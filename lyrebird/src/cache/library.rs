// SPDX-License-Identifier: BSD-3-Clause

use std::{collections::BTreeMap, path::PathBuf};

use color_eyre::eyre::{OptionExt, Report, Result, eyre};
use serde::{Deserialize, Serialize};

use crate::{library::{self, DirectoryID}, track::{self, AlbumID, ArtistID, TrackID}};

#[derive(Serialize, Deserialize)]
pub struct MusicLibrary
{
	// List of all the directories found in the music library
	directories: Vec<Directory>,
	// List of all the tracks found in the music library
	tracks: Vec<Track>,
	// List of all the artists of the tracks in the library
	artists: Vec<Artist>,
	// List of all the albums of the tracks in the library
	albums: Vec<Album>,
}

#[derive(Serialize, Deserialize)]
pub struct Directory
{
	/// Unique 64-bit identifier for the directory
	pub(self) id: u64,
	/// ID of the parent directory (0 if this is the root, which also makes `id` 0)
	pub(self) parentID: u64,
	/// Name of this specific directory to add to the end of the parent directory's path
	pub(self) name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Track
{
	/// Unique 64-bit identifier for the track
	id: u64,
	/// Directory this track is found in
	directoryID: u64,
	/// File name for this track to glue to the end of the directory path
	fileName: String,
	/// Total length of the track in seconds (from its metadata)
	totalLength: u64,
	/// Title of the track (from its metadata)
	title: String,
	/// ID of the artist for the track (if there is one)
	artist: Option<u64>,
	/// ID of the album for the track (if there is one)
	album: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Artist
{
	/// Unique 64-bit identifier for the artist
	id: u64,
	/// Name of the artist
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Album
{
	/// Unique 64-bit identifier for the album
	id: u64,
	/// Name of the album
	name: String,
}

pub struct LibraryMaps
{
	pub dirs: BTreeMap<DirectoryID, library::Directory>,
	pub tracks: BTreeMap<TrackID, track::Track>,
	pub artists: BTreeMap<ArtistID, track::Artist>,
	pub albums: BTreeMap<AlbumID, track::Album>,
	pub nextTrackID: Option<u64>,
}

impl MusicLibrary
{
	pub fn to_maps(&self) -> Result<LibraryMaps>
	{
		self.try_into()
	}
}

impl TryFrom<&MusicLibrary> for LibraryMaps
{
	type Error = Report;

	fn try_from(library: &MusicLibrary) -> Result<Self>
	{
		// Set up maps to deserialise the library into
		let mut dirs: BTreeMap<DirectoryID, library::Directory> = BTreeMap::new();
		let mut tracks: BTreeMap<TrackID, track::Track> = BTreeMap::new();
		let mut artists: BTreeMap<ArtistID, track::Artist> = BTreeMap::new();
		let mut albums: BTreeMap<AlbumID, track::Album> = BTreeMap::new();
		let mut lastTrackID = None;

		// Run through all the directories first, so we can have their paths ready to go
		for dir in &library.directories
		{
			// Make sure the directory's ID doesn't already exist
			if dirs.contains_key(&DirectoryID::new(dir.id))
			{
				return Err(eyre!("Library cache contains duplicated directory entry"));
			}

			// If the directory is the root directory, make sure it has the right parent value and is a valid path
			let path = if dir.id == 0
			{
				let path = PathBuf::from(&dir.name);
				if dir.parentID != 0 || !path.exists()
				{
					return Err(eyre!("Library cache contains invalid root directory"));
				}
				path
			}
			else
			{
				// Reconstitute the path for this directory
				let parent = dirs.get(&DirectoryID::new(dir.parentID))
					.ok_or_eyre("Library cache contains an invalid directory entry")?;
				parent.path().join(&dir.name)
			};

			// Transmute the directory into a library one
			dirs.insert(DirectoryID::new(dir.id), library::Directory::from_path(&path));
		}

		// Finally run through all the tracks, looking their directory, artist, and album up and mapping them
		for track in &library.tracks
		{
			// Look up the artist if this track has one
			let artistID = track.artist.map(ArtistID::new);
			let artist = artistID.and_then(|id| artists.get_mut(&id));
			if artistID.is_some() && artist.is_none()
			{
				return Err(eyre!("Library cache contains track with invalid artist ID"));
			}
			// Look up the album if this track has one
			let albumID = track.album.map(AlbumID::new);
			let album = albumID.and_then(|id| albums.get_mut(&id));
			if albumID.is_some() && album.is_none()
			{
				return Err(eyre!("Library cache contains track with invalid album ID"));
			}
			// Look up the containing directory for the track
			let dir = dirs.get(&DirectoryID::new(track.directoryID))
				.ok_or_eyre("Library cache contains track with invalid directory ID")?;
			let fileName = dir.path().join(&track.fileName);
			if !fileName.exists()
			{
				return Err(eyre!("Library cache contains non-existent track"));
			}

			// Note if the ID is the highest seen yet
			lastTrackID = Some(track.id.max(lastTrackID.unwrap_or_default()));
			// Put it all together into a track
			let track = track::Track::fromCache
			(
				track.id,
				fileName,
				track.totalLength,
				track.title.clone(),
				artistID,
				albumID
			);
			// Add the track to the requisite artist, and album and insert it into the track map
			artist.map(|artist| artist.addTrack(track.id()));
			album.map(|album| album.addTrack(track.id()));
			tracks.insert(track.id(), track);
		}

		Ok(LibraryMaps
		{
			dirs,
			tracks,
			artists,
			albums,
			nextTrackID: lastTrackID.map(|id| id + 1),
		})
	}
}
