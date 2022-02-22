use async_walkdir::{DirEntry, Filtering, WalkDir};
use futures_lite::StreamExt;
use std::path::PathBuf;

pub fn visit_all_gml_files<F: FnMut(String, PathBuf)>(
    project_path: PathBuf,
    errors: &mut Vec<std::io::Error>,
    mut visitor: F,
) {
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
    let mut gml_files = WalkDir::new(project_path.join("objects"))
        .filter(|entry| async move { filter(entry) })
        .chain(
            WalkDir::new(project_path.join("scripts")).filter(|entry| async move { filter(entry) }),
        );

    // Run on every gml file!
    futures_lite::future::block_on(async {
        loop {
            match gml_files.next().await {
                Some(Ok(entry)) => match std::fs::read_to_string(entry.path()) {
                    Ok(gml) => {
                        visitor(gml, entry.path());
                    }
                    Err(e) => errors.push(e),
                },
                Some(Err(e)) => errors.push(e),
                None => break,
            }
        }
    });
}
