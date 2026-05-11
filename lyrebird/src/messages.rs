// SPDX-License-Identifier: BSD-3-Clause

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
	ConcurrencyError,
	AddToPlaylist,
	RemoveFromPlaylist,
	PlayPlaylist,
	PlayCurrent,
	PauseCurrent,
	StopCurrent,
	NextTrack,
	PreviousTrack,
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
