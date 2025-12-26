use chrono::{DateTime, Local};
use std::path::PathBuf;

use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;




#[derive(Debug)]
struct FileMeta {
    path: PathBuf,
    extension: String,
    size_bytes: u64,
    modified: DateTime<Local>,
}



fn scan_directory(root: &PathBuf) -> Result<Vec<FileMeta>> {

    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path().to_path_buf();
        let metadata = fs::metadata(&path)
            .with_context(|| format!("Failed to read metadata: {:?}", path))?;

        let size_bytes = metadata.len();

        let modified: DateTime<Local> =
            metadata.modified()?.into();

        let extension = path
            .extension() 
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        files.push(FileMeta {
            path,
            extension,
            size_bytes,
            modified,
        });
    }
    println!("Scanned {} files", files.len());

    Ok(files)
}


fn main() -> Result<()> {
    let root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?);

    println!(
        "Starting file analysis in directory: {:?}",
        root
    );
    println!("Scanning directory: {:?}", root);

    let files = scan_directory(&root)?;

    println!("Found {} files\n", files.len());

    for file in files.iter().take(10) {
        println!("{:?}", file);
    }

    Ok(())
}

