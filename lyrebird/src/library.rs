use std::{collections::BTreeMap, fs::{create_dir_all, File}, path::{Path, PathBuf}, sync::Arc};

use color_eyre::eyre::{self, OptionExt, Result};
use serde::{Deserialize, Serialize};
use tokio::spawn;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct MusicLibrary
{
	/// Root of this music library
	basePath: PathBuf,
	/// Path to where to cache the library
	#[serde(skip)]
	cacheFile: PathBuf,
	/// Paths to directories containing music relative to the root
	dirs: Vec<PathBuf>,
	/// Map of directories to a list of files in that directory which are music
	files: BTreeMap<PathBuf, Vec<PathBuf>>,
	#[serde(skip)]
	discoveryCancellation: CancellationToken,
}

impl MusicLibrary
{
	pub fn new(cacheFile: &Path, basePath: &PathBuf) -> Result<Arc<RwLock<Self>>>
	{
		if cacheFile.exists()
		{
			MusicLibrary::fromCache(cacheFile)
				.or_else
				(
					|report|
					{
						error!("Reading library cache failed: {}", report);
						MusicLibrary::fromPath(cacheFile, basePath)
					}
				)
		}
		else
		{
			MusicLibrary::fromPath(cacheFile, basePath)
		}
	}

	/// Construct a library from a cache JSON
	pub fn fromCache(cacheFile: &Path) -> Result<Arc<RwLock<Self>>>
	{
		let cache = File::open(cacheFile)?;
		let mut library: MusicLibrary = serde_json::from_reader(cache)?;
		library.cacheFile = cacheFile.to_path_buf();
		Ok(Arc::new(RwLock::new(library)))
	}

	/// Construct a library from a new base path
	pub fn fromPath(cacheFile: &Path, basePath: &PathBuf) -> Result<Arc<RwLock<Self>>>
	{
		if !basePath.is_dir()
		{
			return Err(eyre::eyre!("Library path must be a valid directory"));
		}

		let library = Arc::new
		(
			RwLock::new
			(
				MusicLibrary
				{
					basePath: basePath.clone(),
					cacheFile: cacheFile.to_path_buf(),
					dirs: Vec::new(),
					files: BTreeMap::new(),
					discoveryCancellation: CancellationToken::new(),
				}
			)
		);

		MusicLibrary::asyncDiscover(library.clone(), basePath.clone());

		Ok(library)
	}

	pub fn writeCache(&self) -> Result<()>
	{
		// Ask our discovery task to stop if it didn't already
		self.discoveryCancellation.cancel();
		// Make sure all the leading path elements exist
		create_dir_all
		(
			self.cacheFile.parent()
				.ok_or_eyre("Failed to extract the path to the music library cache file")?
		)?;
		// Open the cache file for writing
		let cache = File::create(&self.cacheFile)?;
		// Ask serde to serialise out the library cache
		Ok(serde_json::to_writer(cache, self)?)
	}

	fn asyncDiscover(library: Arc<RwLock<Self>>, currentDirectory: PathBuf)
	{
		let task = async move
		{
			MusicLibrary::discover(library.as_ref(), currentDirectory.as_path()).await
		};
		spawn(task);
	}

	async fn discover(library: &RwLock<Self>, currentDirectory: &Path) -> Result<()>
	{
		// Explore the current directory's contents
		let contents = currentDirectory.read_dir()?;
		// For each entry in it
		for entry in contents
		{
			// Get the path to that entry
			let path = entry?.path();
			// If it's a directory, add it to the set discovered and recurse
			if path.is_dir()
			{
				library.write().await.dirs.push(path.clone());
				Box::pin(MusicLibrary::discover(&library, &path)).await?;
			}
			// Else if it's a file, see if it's audio
			else
			{
				// Check if this file is an audio file, and if it is..

				// See if this file's directory is already in the map
				let filePath = path.parent()
					.ok_or_eyre("File does not have a valid path parent")?;
				if !library.read().await.files.contains_key(filePath)
				{
					library.write().await.files.insert(filePath.to_path_buf(), Vec::new());
				}
				// Now we definitely have a vec to use, look the path up and add the file
				library.write().await.files.get_mut(filePath)
					.ok_or_eyre("Failed to look file's path up in file map")?
					.push(path);
			}
			// If we're being asked to stop, stop
			if library.read().await.discoveryCancellation.is_cancelled()
			{
				break
			}
		}

		// We done? good!
		Ok(())
	}
}
