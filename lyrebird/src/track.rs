// SPDX-License-Identifier: BSD-3-Clause

use std::path::PathBuf;

/// Represents a track in a music library
pub struct Track
{
	/// Unique 64-bit identifier for the track
	id: u64,
	/// Path to the file holding this track
	file: PathBuf,
	/// Total length of the track in seconds
	totalLength: u64,
	/// Title of the track
	title: String,
	/// ID of the artist for the track (if there is one)
	artist: Option<ArtistID>,
	/// ID of the album for the track (if there is one)
	album: Option<AlbumID>
}

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

/// Unique strongly typed identifier for an album
pub struct AlbumID(u64);

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
