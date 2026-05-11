// SPDX-License-Identifier: BSD-3-Clause

use serde::{Deserialize, Serialize};

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
	id: u64,
	/// ID of the parent directory (0 if this is the root, which also makes `id` 0)
	parentID: u64,
	/// Name of this specific directory to add to the end of the parent directory's path
	name: String,
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
