// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]
#![warn(clippy::pedantic)]

use color_eyre::{eyre, Result};
use directories::ProjectDirs;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use window::MainWindow;

mod albumList;
mod artistList;
mod cache;
mod config;
mod library;
mod libraryTree;
mod messages;
mod options;
mod playback;
mod playlist;
mod playlists;
mod theme;
mod track;
mod widgets;
mod window;

fn main() -> Result<()>
{
	color_eyre::install()?;
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

	// Set up the main window w/ the configuration
	let mainWindow = MainWindow::new(&paths)?;
	// Now run the main window of Lyrebird till the user exits the program
    let result = iced_winit::run(mainWindow);
	Ok(result?)
}
