use async_recursion::async_recursion;
use async_walkdir::{DirEntry, Filtering, WalkDir};
use futures::{executor::block_on, lock::Mutex, stream::Chain, Future, StreamExt};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct GmlWalker {
    walker: Chain<WalkDir, WalkDir>,
    errors: Vec<std::io::Error>,
}
impl GmlWalker {
    pub fn new(path: &Path) -> Self {
        /// Filters DirEntry's for gml files.
        fn filter(entry: DirEntry) -> Filtering {
            if let Some(true) = entry
                .path()
                .file_name()
                .map(|f| !f.to_string_lossy().ends_with(".gml"))
            {
                Filtering::Ignore
            } else {
                Filtering::Continue
            }
        }

        // Create a WalkDir for all the GML in the project folder...
        // (ignoring folders that don't contain GML gives us a huge speed bonus)

        Self {
            walker: WalkDir::new(path.join("objects"))
                .filter(|entry| async move { filter(entry) })
                .chain(
                    WalkDir::new(path.join("scripts")).filter(|entry| async move { filter(entry) }),
                ),
            errors: vec![],
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> Option<PathBuf> {
        if let Some(dir_entry) = self.walker.next().await {
            match dir_entry {
                Ok(dir_entry) => Some(dir_entry.path()),
                Err(e) => {
                    self.errors.push(e);
                    self.next().await
                }
            }
        } else {
            None
        }
    }
}
