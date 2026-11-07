// SPDX-License-Identifier: BSD-3-Clause

use std::fmt;
use std::fmt::Display;

use iced::Point;
use iced::keyboard;
use iced::window;

use crate::library::DirectoryID;
use crate::playback::PlaybackState;
use crate::playlist::PlaylistEntryID;
use crate::playlists::PlaylistID;
use crate::track::{AlbumID, ArtistID, TrackID};
use crate::widgets::tabBar::TabBarEnum;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab
{
	LibraryTree = 1,
	Artists = 2,
	Albums = 3,
	Options = 4,
	Playlists = 5,
}

#[derive(Clone)]
pub enum Message
{
	SwitchTo(Tab),
	LibraryLoading,
	LibraryLoaded,
	LibraryDiscover,
	LibraryDiscovering,
	LibraryDiscovered,
	LibraryCached,
	LibraryError,
	ConcurrencyError,
	LibraryTree(LibraryTree),
	ArtistList(ArtistList),
	AlbumList(AlbumList),
	Playlist(Playlist),
	KeyEvent(keyboard::Event),
	WindowOpened(window::Id),
	WindowClosed(window::Id),
	AddToPlaylist(PlaylistID, TrackID, PlayPosition),
	RemoveFromPlaylist(PlaylistID, PlaylistEntryID),
	PlayPlaylist(PlaylistID),
	PlayCurrent,
	PauseCurrent,
	TogglePlayback,
	NextTrack,
	PreviousTrack,
	PlaybackNotification(PlaybackState),
}

#[derive(Clone, Copy)]
pub enum LibraryTree
{
	SelectDirectory(DirectoryID),
	DirectoryMenuOpen(DirectoryID, Point, f32),
	DirectoryMenuClose,
	DirectoryAction(DirectoryID, DirectoryMenuItem),
	TrackMenuOpen(TrackID, Point, f32),
	TrackMenuClose,
	TrackAction(TrackID, TrackMenuItem),
}

#[derive(Clone, Copy)]
pub enum TrackMenuItem
{
	PlayNow,
	PlayNext,
	PlayLast,
}

#[derive(Clone, Copy)]
pub enum DirectoryMenuItem
{
	Play,
	AddNextToNowPlaying,
	AddLastToNowPlaying,
}

#[derive(Clone, Copy)]
pub enum ArtistList
{
	SelectArtist(ArtistID),
}

#[derive(Clone, Copy)]
pub enum AlbumList
{
	SelectAlbum(AlbumID),
}

#[derive(Clone, Copy)]
pub enum Playlist
{
	SelectPlaylist(PlaylistID),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayPosition
{
	Now,
	Next,
	Last,
}

impl Default for Tab
{
	fn default() -> Self
	{
		Self::LibraryTree
	}
}

impl TabBarEnum for Tab
{
	fn tabs<'a>() -> &'a[Self]
	{
		&[Self::LibraryTree, Self::Artists, Self::Albums, Self::Options, Self::Playlists]
	}

	fn name(&self) -> &'static str
	{
		match self
		{
			Self::LibraryTree => "Tree",
			Self::Artists => "Artists",
			Self::Albums => "Albums",
			Self::Options => "Options",
			Self::Playlists => "Playlist",
		}
	}

	fn value(&self) -> usize
	{
		*self as usize
	}

	fn message_for(&self) -> Message
	{
		Message::SwitchTo(self.clone())
	}
}

impl Display for TrackMenuItem
{
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			TrackMenuItem::PlayNow => write!(fmt, "Play Now"),
			TrackMenuItem::PlayNext =>  write!(fmt, "Play Next"),
			TrackMenuItem::PlayLast => write!(fmt, "Play Last"),
		}
	}
}

impl Display for DirectoryMenuItem
{
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			DirectoryMenuItem::Play => write!(fmt, "Play Tracks"),
			DirectoryMenuItem::AddNextToNowPlaying => write!(fmt, "Add tracks next to Now Playing"),
			DirectoryMenuItem::AddLastToNowPlaying => write!(fmt, "Add tracks to end of Now Playing"),
		}
	}
}
