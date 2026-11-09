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
	pub(self) id: u64,
	/// Name of the artist
	pub(self) name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Album
{
	/// Unique 64-bit identifier for the album
	pub(self) id: u64,
	/// Name of the album
	pub(self) name: String,
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
	pub fn new
	(
		dirs: &BTreeMap<DirectoryID, library::Directory>,
		tracks: &BTreeMap<TrackID, track::Track>,
		artists: &BTreeMap<ArtistID, track::Artist>,
		albums: &BTreeMap<AlbumID, track::Album>,
	) -> Self
	{
		let mut trackList = Vec::new();
		for track in tracks.values()
		{
			// Look up the track in the library's directories map and find which one it belongs in
			let directoryID = dirs
				.iter()
				.find(|(_, directory)| directory.contents().contains(&track.id()))
				.map(|(&directoryID, _)| directoryID)
				.expect("Track somehow has no holding directory even though it must");
			// Map the track back to the cache version of Track and add it to the list
			trackList.push(Track
			{
				id: track.id().into(),
				directoryID: directoryID.into(),
				fileName: track.fileName().into(),
				totalLength: track.totalLength(),
				title: track.title().into(),
				artist: track.artistID().map(Into::into),
				album: track.albumID().map(Into::into),
			});
		}

		Self
		{
			directories: Directory::from_map(dirs),
			tracks: trackList,
			artists: Artist::from_map(artists),
			albums: Album::from_map(albums),
		}
	}

	pub fn to_maps(self) -> Result<LibraryMaps>
	{
		self.try_into()
	}
}

impl Directory
{
	pub fn from_map(map: &BTreeMap<DirectoryID, library::Directory>) -> Vec<Self>
	{
		let mut directories = Vec::new();
		// Loop through all the directories in the map
		for directory in map.values()
		{
			// See what the parent of the directory is, if there is one
			// (if there isn't it's the root directory, so synth a fake ID for it)
			let parentID = map
				.iter()
				.find(|(_, dir)| dir.subdirs().contains(&directory.id()))
				.map(|(&directoryID, _)| directoryID)
				.unwrap_or_else(|| DirectoryID::new(0));
			// Figure out what the name of this directory is
			let name = if directory.id() == DirectoryID::new(0)
			{
				directory.path().as_os_str()
			}
			else
			{
				directory
					.path()
					.file_name()
					.expect("Directory somehow has no parent even though it's below another")
			}
				.to_string_lossy();
			// Turn the result into a cache Directory object and add it to the list
			directories.push(Directory
			{
				id: directory.id().into(),
				parentID: parentID.into(),
				name: name.to_string(),
			});
		}
		directories
	}
}

impl Artist
{
	pub fn from_map(map: &BTreeMap<ArtistID, track::Artist>) -> Vec<Self>
	{
		let mut artists = Vec::new();
		// Loop through all the artists in the map
		for (artistID, artist) in map
		{
			// Map the artist back to an ID and a name and add the resulting Artist object to the list
			artists.push(Artist
			{
				id: artistID.into(),
				name: artist.name().into(),
			});
		}
		artists
	}
}

impl Album
{
	pub fn from_map(map: &BTreeMap<AlbumID, track::Album>) -> Vec<Self>
	{
		let mut albums = Vec::new();
		// Loop through all the albums in the map
		for (albumID, album) in map
		{
			// Map the album back to an ID and a name and add the resulting Album object to the list
			albums.push(Album
			{
				id: albumID.into(),
				name: album.name().into(),
			});
		}
		albums
	}
}

fn map_directories(dirs: Vec<Directory>) -> Result<BTreeMap<DirectoryID, library::Directory>>
{
	let mut result: BTreeMap<DirectoryID, library::Directory> = BTreeMap::new();

	for dir in dirs
	{
		// Make sure the directory's ID doesn't already exist
		if result.contains_key(&DirectoryID::new(dir.id))
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
			let parent = result.get(&DirectoryID::new(dir.parentID))
				.ok_or_eyre("Library cache contains an invalid directory entry")?;
			parent.path().join(&dir.name)
		};

		// Transmute the directory into a library one
		let directory = library::Directory::from_path(dir.id, &path);
		result.insert(directory.id(), directory);
		// Add the new directory into the parent one if it's not the root directory
		if dir.id != 0
		{
			result.get_mut(&DirectoryID::new(dir.parentID))
				.expect("Directory entry lookup failure")
				.addSubdir(DirectoryID::new(dir.id));
		}
	}

	Ok(result)
}

fn map_artists(artists: Vec<Artist>) -> Result<BTreeMap<ArtistID, track::Artist>>
{
	let mut result: BTreeMap<ArtistID, track::Artist> = BTreeMap::new();

	for artist in artists
	{
		// Make sure the artist's ID doesn't already exist
		if result.contains_key(&ArtistID::new(artist.id))
		{
			return Err(eyre!("Library cache contains duplicated artist entry"));
		}

		// Transmute the artist into a library oen
		result.insert(ArtistID::new(artist.id), track::Artist::new(&artist.name));
	}

	Ok(result)
}

fn map_albums(albums: Vec<Album>) -> Result<BTreeMap<AlbumID, track::Album>>
{
	let mut result: BTreeMap<AlbumID, track::Album> = BTreeMap::new();

	for album in albums
	{
		// Make sure the artist's ID doesn't already exist
		if result.contains_key(&AlbumID::new(album.id))
		{
			return Err(eyre!("Library cache contains duplicated album entry"));
		}

		// Transmute the album into a library oen
		result.insert(AlbumID::new(album.id), track::Album::new(&album.name));
	}

	Ok(result)
}

impl TryFrom<MusicLibrary> for LibraryMaps
{
	type Error = Report;

	fn try_from(library: MusicLibrary) -> Result<Self>
	{
		// Run through all the directories first, so we can have their paths ready to go
		let mut dirs = map_directories(library.directories)?;
		// Then through all the artists and albums so we have their IDs to look up
		let mut artists = map_artists(library.artists)?;
		let mut albums = map_albums(library.albums)?;
		// Set up maps to deserialise the library into
		let mut tracks: BTreeMap<TrackID, track::Track> = BTreeMap::new();
		let mut lastTrackID = None;

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
			let dir = dirs.get_mut(&DirectoryID::new(track.directoryID))
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
			// Add the track to the requisite directory, artist, and album and insert it into the track map
			dir.addTrack(track.id());
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
