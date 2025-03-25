// SPDX-License-Identifier: BSD-3-Clause
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{style::{Style, Stylize}, DefaultTerminal, Frame};

pub struct MainWindow
{
	header: Style,
	headerEntry: Style,
	headerNumber: Style,
	activeEntry: Style,
	footer: Style,
}

impl MainWindow
{
	pub fn new() -> Self
	{
		MainWindow
		{
			header: Style::new().blue().on_black(),
			headerEntry: Style::new().blue().on_black(),
			headerNumber: Style::new().light_blue().on_black(),
			activeEntry: Style::new().light_blue(),
			footer: Style::new().blue().on_black(),
		}
	}

	pub fn run(&self, terminal: &mut DefaultTerminal) -> Result<()>
	{
		loop
		{
			terminal.draw(|frame| self.draw(frame))?;
			if matches!(event::read()?, Event::Key(_))
			{
				break Ok(());
			}
		}
	}

	fn draw(&self, frame: &mut Frame)
	{
		frame.render_widget("Lyrebird", frame.area());
	}
}
