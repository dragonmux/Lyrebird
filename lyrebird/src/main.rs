#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()>
{
	let terminal = ratatui::init();
	let result = run(terminal);
	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal) -> Result<()>
{
	loop
	{
		terminal.draw(draw)?;
		if matches!(event::read()?, Event::Key(_))
		{
			break Ok(());
		}
	}
}

fn draw(frame: &mut Frame)
{
	frame.render_widget("Lyrebird", frame.area());
}
