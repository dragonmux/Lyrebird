// SPDX-License-Identifier: BSD-3-Clause
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::{Tabs, Widget}, DefaultTerminal, Frame};

use crate::libraryTree::LibraryTree;

/// Represents the main window of Lyrebird
pub struct MainWindow
{
	header: Style,
	headerEntry: Style,
	headerNumber: Style,
	activeEntry: Style,
	footer: Style,

	exit: bool,
	activeTab: usize,

	libraryTree: LibraryTree
}

impl MainWindow
{
	/// Set up a new main window, building the style pallet needed
	pub fn new() -> Self
	{
		let activeEntry = Style::new().light_blue();

		MainWindow
		{
			header: Style::new().blue().on_black(),
			headerEntry: Style::new().blue().on_black(),
			headerNumber: Style::new().light_blue().on_black(),
			activeEntry: activeEntry,
			footer: Style::new().blue().on_black(),

			exit: false,
			activeTab: 0,

			libraryTree: LibraryTree::new(activeEntry)
		}
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
		match event::read()?
		{
			Event::Key(key) if key.kind == KeyEventKind::Press => match key.code
			{
				KeyCode::Char('q') | KeyCode::Char('Q') => self.quit(),
				_ => {}
			},
			_ => {}
		}
		Ok(())
	}

	fn quit(&mut self)
	{
		self.exit = true
	}

	// Draw the program window to the terminal
	fn draw(&self, frame: &mut Frame)
	{
		frame.render_widget(self, frame.area());
	}
}

// Turn the window into a widget for rendering to make the rendering phase simpler
impl Widget for &MainWindow
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
		let headerTabs: Vec<_> = ["Tree", "Artists", "Albums", "Options", "Playlist"]
			.map(|title| title.to_string())
			.into_iter()
			.enumerate()
			.map(|(num, tabTitle)|
				[
					Span::styled((num + 1).to_string(), self.headerNumber),
					Span::from(" "),
					Span::styled(tabTitle, self.headerEntry),
				]
			)
			.map(|spans| Line::from(spans.to_vec()).left_aligned())
			.collect();

		// Build a layout for the header line
		let headerLayout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)])
			.flex(Flex::SpaceBetween)
			.split(areas[0]);

		// Display the program header - starting with the program name, followed by the tabs
		Line::styled(" Lyrebird", self.header).render(headerLayout[0], buf);
		Tabs::new(headerTabs)
			.style(self.headerEntry)
			.highlight_style(self.activeEntry)
			.select(self.activeTab)
			.divider("│")
			.render(headerLayout[1], buf);

		match self.activeTab
		{
			0 => self.libraryTree.render(areas[1], buf),
			_ => {},
		}
	}
}
