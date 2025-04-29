// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]
#![warn(clippy::pedantic)]

use color_eyre::{eyre, Result};
use config::Config;
use directories::ProjectDirs;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use window::MainWindow;

mod config;
mod library;
mod libraryTree;
mod options;
mod playback;
mod playlist;
mod playlists;
mod widgets;
mod window;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()>
{
	tracing_subscriber::registry()
		.with
		(
			tracing_subscriber::fmt::layer()
				.with_filter(LevelFilter::INFO)
		)
		.init();

	// Try to get the application paths available
	let paths = ProjectDirs::from("com", "rachelmant", "Lyrebird").
		ok_or_else(|| eyre::eyre!("Failed to get program working paths"))?;
	// Now try to get a configuration object so we know where to find things and such
	let mut config = Config::read(&paths)?;

	// Aquire the terminal to use and set up the main window w/ the configuration
	let mut terminal = ratatui::init();
	let mut mainWindow = MainWindow::new(&paths, &mut config, terminal.size()?)?;
	// Now run the main window of Lyrebird till the user exits the program
	let result = mainWindow.run(&mut terminal).await;
	// Give the terminal back and return the result of running the main window
	ratatui::restore();
	// Re-serialise the user's config as our last step
	config.write(&paths)?;
	result
}
