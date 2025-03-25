// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]

use color_eyre::Result;
use window::MainWindow;

mod window;

fn main() -> Result<()>
{
	let terminal = ratatui::init();
	let mainWindow = MainWindow::new();
	let result = mainWindow.run(terminal);
	ratatui::restore();
	result
}
