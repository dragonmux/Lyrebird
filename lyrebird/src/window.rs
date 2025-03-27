use std::time::Duration;

// SPDX-License-Identifier: BSD-3-Clause
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use directories::ProjectDirs;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

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

	currentlyPlaying: Option<String>,
	songDuration: Option<Duration>,
	playedDuration: Option<Duration>,
	errorState: Option<String>
}

#[derive(Clone, Copy)]
enum Tab
{
	LibraryTree,
}

impl Tab
{
	const fn value(self) -> usize
	{
		self as usize
	}
}

impl MainWindow
{
	/// Set up a new main window, building the style pallet needed
	pub fn new(paths: &ProjectDirs, config: &mut Config) -> Result<Self>
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
				activeEntry, &paths.cache_dir().join("library.json"), &config.libraryPath
			)?,

			currentlyPlaying: None,
			songDuration: None,
			playedDuration: None,
			errorState: None,
		})
	}

	/// Run the program window until an exit-causing event occurs
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()>
	{
		while !self.exit
		{
			terminal.draw(|frame| self.draw(frame))?;
			self.handleEvents()?;
		}
		Ok(())
	}

	fn handleEvents(&mut self) -> Result<()>
	{
		// See if we got any events
		match event::read()?
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
						_ => {}
					}
				}
				// It's some other kind of event, so figure out which is the active
				// tab and ask it what it thinks of this
				match self.activeTab
				{
					Tab::LibraryTree => self.libraryTree.handleKeyEvent(key),
				}
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
}

fn durationAsString(duration: Duration) -> String
{
	let seconds = duration.as_secs();
	let minutes = seconds / 60;
	let seconds = seconds % 60;
	format!("{minutes:2}:{seconds:02}")
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
		let currentlyPlaying = self.currentlyPlaying.as_ref().map_or_else
		(
			|| String::from("Nothing playing"), Clone::clone
		);
		let songDuration = self.songDuration.map_or_else
		(
			|| String::from("--:--"), durationAsString
		);
		let playedDuration = self.playedDuration.map_or_else
		(
			|| String::from("--:--"), durationAsString
		);
		let errorState = self.errorState.as_ref().map_or_else
		(
			|| String::from("No errors"), Clone::clone
		);

		// Display the program footer - which song is currently playing, song runtime, and whether errors have occured
		Line::from_iter([String::from(" "), currentlyPlaying])
			.style(self.footer)
			.render(footerLayout[0], buf);
		Line::styled(format!("{songDuration}/{playedDuration}"), self.footer)
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
