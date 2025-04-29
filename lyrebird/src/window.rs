// SPDX-License-Identifier: BSD-3-Clause
use std::path::{Path, PathBuf};
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use directories::ProjectDirs;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect, Size};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{channel, Receiver};
use tokio_stream::StreamExt;

use crate::options::OptionsPanel;
use crate::playback::{PlaybackState, Song};
use crate::playlists::Playlists;
use crate::widgets::tabBar::TabBar;
use crate::{config::Config, libraryTree::LibraryTree};

/// Represents the main window of Lyrebird
pub struct MainWindow
{
	header: Style,
	headerEntry: Style,
	headerNumber: Style,
	activeEntry: Style,
	footer: Style,

	exit: bool,
	activeTab: Tab,

	libraryTree: LibraryTree,
	optionsPanel: OptionsPanel,
	playlists: Playlists,

	currentlyPlaying: Option<(Song, Receiver<PlaybackState>)>,
	errorState: Option<String>
}

#[derive(Clone, Copy)]
enum Tab
{
	LibraryTree = 0,
	Options = 3,
	Playlists = 4,
}

impl Tab
{
	const fn value(self) -> usize
	{
		self as usize
	}
}

pub enum Operation
{
	/// Processing event determined there's nothing needs to be done
	None,
	/// Play a file, replacing the Now Playing playlist
	Play(PathBuf),
	/// Play a file already in the Now Playing playlist as if the current reached `PlaybackState::Complete`
	PlayNext(PathBuf),
	/// Add a file to the Now Playing playlist
	Playlist(PathBuf),
}

impl Operation
{
	pub fn playlist(song: Option<PathBuf>) -> Self
	{
		match song
		{
			Some(song) => Operation::Playlist(song),
			None => Operation::None,
		}
	}
}

impl MainWindow
{
	/// Set up a new main window, building the style pallet needed
	pub fn new(paths: &ProjectDirs, config: &mut Config, initialSize: Size) -> Result<Self>
	{
		let activeEntry = Style::new().light_blue();

		Ok(Self
		{
			header: Style::new().blue().on_black(),
			headerEntry: Style::new().blue().on_black(),
			headerNumber: Style::new().light_blue().on_black(),
			activeEntry,
			footer: Style::new().blue().on_black(),

			exit: false,
			activeTab: Tab::LibraryTree,

			libraryTree: LibraryTree::new
			(
				activeEntry,
				&paths.cache_dir().join("library.json"),
				&config.libraryPath,
				Size::new(initialSize.width, initialSize.height.saturating_sub(2)),
			)?,
			optionsPanel: OptionsPanel::new(),
			playlists: Playlists::new(activeEntry),

			currentlyPlaying: None,
			errorState: None,
		})
	}

	/// Run the program window until an exit-causing event occurs
	pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()>
	{
		// Set up an events stream for console events happening
		let mut events = EventStream::new();
		// Set up a redraw timer
		let mut frameTimer = tokio::time::interval(Duration::from_secs(1).div_f32(50.0));

		// Until the user's asked us to exit
		while !self.exit
		{
			// If we're not discovering the library tree any more, check if we don't need to join the background
			// thread for discovery
			if !self.libraryTree.isDiscovering()
			{
				self.libraryTree.maybeJoinDiscovery().await?;
				// Redraw the terminal before trying to process an event
				terminal.draw(|frame| self.draw(frame))?;
			}
			// See if there's something to do from one of our event sources
			tokio::select!
			{
				// Redraw the terminal every 50th of a second while discovery runs
				_ = frameTimer.tick(), if self.libraryTree.isDiscovering() =>
					{ terminal.draw(|frame| self.draw(frame))?; },
				// Ask if there are more events to handle
				Some(Ok(event)) = events.next() => { self.handleEvent(&event)?; },
				// If there is a file playing, check to see if it's giving us any notifications
				Some(notification) = self.playbackNotification(), if self.currentlyPlaying.is_some() =>
					{ self.handlePlaybackNotification(&notification)? },
			}
		}
		Ok(())
	}

	fn handleEvent(&mut self, event: &Event) -> Result<()>
	{
		// We did! find out what it was and handle it
		match event
		{
			// Key change event?
			Event::Key(key) =>
			{
				// Key press?
				if key.kind == KeyEventKind::Press
				{
					// Check to see if the event is for quitting
					match key.code
					{
						KeyCode::Char('q' | 'Q') => { return self.quit(); },
						KeyCode::Char(' ') => { self.togglePlayback(); },
						KeyCode::Char('1') => { self.activeTab = Tab::LibraryTree; }
						KeyCode::Char('4') => { self.activeTab = Tab::Options; }
						KeyCode::Char('5') => { self.activeTab = Tab::Playlists; }
						_ => {}
					}
				}
				// It's some other kind of event, so figure out which is the active
				// tab and ask it what it thinks of this
				let operation = match self.activeTab
				{
					Tab::LibraryTree => self.libraryTree.handleKeyEvent(*key),
					Tab::Options => self.optionsPanel.handleKeyEvent(*key),
					Tab::Playlists => self.playlists.handleKeyEvent(*key),
				};
				// If that key event resulted in a new file to play, process that
				match operation
				{
					Operation::Play(fileName) =>
					{
						let song = fileName.as_path();
						self.playlists.nowPlaying().replaceWith(song);
						self.playSong(song)?;
					},
					Operation::PlayNext(fileName) => self.playSong(fileName.as_path())?,
					Operation::Playlist(song) => self.playlistSong(song.as_path())?,
					Operation::None => {},
				}
			},
			Event::Resize(width, height) =>
			{
				self.libraryTree.handleResize(Size::new(*width, *height));
			},
			_ => {}
		}
		Ok(())
	}

	fn quit(&mut self) -> Result<()>
	{
		self.exit = true;
		self.libraryTree.writeCache()
	}

	// Draw the program window to the terminal
	fn draw(&mut self, frame: &mut Frame)
	{
		frame.render_widget(self, frame.area());
	}

	fn playSong(&mut self, fileName: &Path) -> Result<()>
	{
		// Make a new channel for the new playback thread to communicate back to us with
		let (sender, receiver) = channel(1);
		let mut song = Song::from(fileName, sender)?;
		let currentlyPlaying = self.currentlyPlaying.take();
		// If we already have a song playing, stop it
		if let Some((mut currentSong, _)) = currentlyPlaying
		{
			currentSong.stop()?;
		}
		// Now replace the current playing state with the new one having asked this new one to start
		song.play();
		self.currentlyPlaying = Some((song, receiver));
		Ok(())
	}

	fn playlistSong(&mut self, fileName: &Path) -> Result<()>
	{
		let nowPlaying = self.playlists.nowPlaying();
		nowPlaying.add(fileName);
		match &self.currentlyPlaying
		{
			Some(_) => Ok(()),
			None => self.playSong(fileName),
		}
	}

	fn togglePlayback(&mut self)
	{
		if let Some((song, _)) = &mut self.currentlyPlaying
		{
			match song.state()
			{
				PlaybackState::Playing =>
				{
					let result = song.pause();
					if let Err(error) = result
					{
						self.errorState = Some(error.to_string());
					}
				},
				PlaybackState::Paused |
				PlaybackState::Stopped |
				PlaybackState::NotStarted =>
					{ song.play(); }
				PlaybackState::Complete => {}
				PlaybackState::Unknown(error) =>
					{ self.errorState = Some(error); }
			}
		}
	}

	// Wait for a playback notification from the currently playing song - note, it is an
	// error to call this function if self.currentlyPlaying is None!
	async fn playbackNotification(&mut self) -> Option<PlaybackState>
	{
		#[expect(clippy::unwrap_used, reason = "impossible in context")]
		let (_, channel) = self.currentlyPlaying.as_mut().unwrap();
		channel.recv().await
	}

	fn handlePlaybackNotification(&mut self, notification: &PlaybackState) -> Result<()>
	{
		match notification
		{
			// Playback completed, so.. go find out if there's something more
			// to play in the now playing playlist, and set it going if there is
			PlaybackState::Complete =>
			{
				let nowPlaying = self.playlists.nowPlaying();
				let nextEntry = nowPlaying.next();
				match nextEntry
				{
					Some(fileName) => self.playSong(fileName.as_path())?,
					None => self.currentlyPlaying = None,
				}
			},
			_ => {},
		}
		Ok(())
	}
}

fn durationAsString(duration: Duration) -> String
{
	if duration.is_zero()
	{
		"--:--".to_string()
	}
	else
	{
		let seconds = duration.as_secs();
		let minutes = seconds / 60;
		let seconds = seconds % 60;
		format!("{minutes:2}:{seconds:02}")
	}
}

// Turn the window into a widget for rendering to make the rendering phase simpler
impl Widget for &mut MainWindow
{
	fn render(self, area: Rect, buf: &mut Buffer)
		where Self: Sized
	{
		// Split the screen up into 3 major chunks - the header line, content, and footer line
		let areas = Layout::vertical
		(
			[Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)]
		).split(area);

		// Make the header tab titles
		let headerTabs = ["Tree", "Artists", "Albums", "Options", "Playlist"]
			.map(ToString::to_string)
			.into_iter()
			.enumerate()
			.map(|(num, tabTitle)|
				[
					Span::styled((num + 1).to_string(), self.headerNumber),
					Span::from(" "),
					Span::styled(tabTitle, self.headerEntry),
				]
			)
			.map(|spans| Line::from(spans.to_vec()).left_aligned());

		// Build a layout for the header line
		let headerLayout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)])
			.flex(Flex::SpaceBetween)
			.split(areas[0]);

		// Display the program header - starting with the program name, followed by the tabs
		Line::styled(" Lyrebird", self.header).render(headerLayout[0], buf);
		TabBar::new(headerTabs)
			.style(self.headerEntry)
			.highlightedStyle(self.activeEntry)
			.select(self.activeTab.value())
			.firstTabDivider(true)
			.render(headerLayout[1], buf);

		// Figure out which tab is currently active and draw that
		match self.activeTab
		{
			Tab::LibraryTree => self.libraryTree.render(areas[1], buf),
			Tab::Options => self.optionsPanel.render(areas[1], buf),
			Tab::Playlists => self.playlists.render(areas[1], buf),
		}

		// Build a layout for the footer line
		let (footerLayout, footerSpacers ) = Layout::horizontal
		(
			[Constraint::Percentage(50), Constraint::Fill(1), Constraint::Fill(3)]
		)
			.flex(Flex::SpaceBetween)
			.spacing(1)
			.split_with_spacers(areas[2]);

		// Figure out what strings are to be displayed in the footer
		let currentlyPlaying = self.currentlyPlaying.as_ref()
			.map_or_else(|| String::from("Nothing playing"), |(song, _)| song.description());
		let songDuration = self.currentlyPlaying.as_ref()
			.and_then(|(song, _)| song.songDuration())
			.map_or_else
			(
				|| String::from("--:--"), durationAsString
			);
		let playedDuration = self.currentlyPlaying.as_ref()
			.map_or_else
			(
				|| String::from("--:--"),
				|(song, _)| durationAsString(song.playedDuration())
			);
		let errorState = self.errorState.as_ref().map_or_else
		(
			|| String::from("No errors"), Clone::clone
		);

		// Display the program footer - which song is currently playing, song runtime, and whether errors have occured
		Line::from_iter([String::from(" "), currentlyPlaying])
			.style(self.footer)
			.render(footerLayout[0], buf);
		Line::styled(format!("{playedDuration}/{songDuration}"), self.footer)
			.centered()
			.render(footerLayout[1], buf);
		Line::styled(errorState, self.footer).render(footerLayout[2], buf);

		// Render the spacers for all the components of the footer
		for spacerRect in footerSpacers.iter()
		{
			Line::styled("│", self.footer).render(*spacerRect, buf);
		}
	}
}
