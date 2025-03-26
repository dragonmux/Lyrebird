use std::{fs::{create_dir_all, File}, path::PathBuf};

use color_eyre::eyre::Result;
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
pub struct Config
{
	version: ConfigVersion,
	pub libraryPath: PathBuf,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ConfigVersion
{
	Version1 = 1,
}

impl Config
{
	pub fn read(paths: &ProjectDirs) -> Result<Self>
	{
		let configPath = paths.config_dir().join("config.json");

		if configPath.exists()
		{
			let configFile = File::open(configPath)?;
			let config = serde_json::from_reader(configFile)?;

			return Ok(config);
		}
		
		Ok(Self::default())
	}

	pub fn write(&self, paths: &ProjectDirs) -> Result<()>
	{
		let configPath = paths.config_dir();
		create_dir_all(configPath)?;
		let configPath = configPath.join("config.json");
		let configFile = File::create(configPath)?;
		Ok(serde_json::to_writer(configFile, self)?)
	}
}

impl Default for Config
{
	fn default() -> Self
	{
		// Try to get the user directories
		let userDirs = UserDirs::new().expect("Failed to get user directories");
		// See if we can get the user's music directory; if we can't, default to their homedir.
		let musicDir = userDirs.audio_dir().map_or_else(|| userDirs.home_dir(), |dir| dir);

		// Generate a configuration with this data
		Self
		{
			version: ConfigVersion::Version1,
			libraryPath: musicDir.to_path_buf(),
		}
	}
}
