//! File management operations

use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tera_common::error::{Error, Result};
use tokio::fs as async_fs;

/// List files in a directory
pub async fn list_files(path: &str, recursive: bool) -> Result<Vec<FileInfo>> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(Error::NotFound(format!("Path not found: {}", path.display())));
    }

    if !path.is_dir() {
        return Err(Error::InvalidInput(format!("Not a directory: {}", path.display())));
    }

    let mut files = Vec::new();
    let mut stack = vec![path.to_path_buf()];

    while let Some(current) = stack.pop() {
        match async_fs::read_dir(&current).await {
            Ok(mut entries) => {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    let metadata = entry.metadata().await?;

                    if metadata.is_dir() {
                        if recursive {
                            stack.push(path.clone());
                        }
                    }

                    files.push(FileInfo {
                        path: path.display().to_string(),
                        is_dir: metadata.is_dir(),
                        size: metadata.len(),
                        modified: metadata.modified()?
                            .duration_since(std::time::UNIX_EPOCH)?
                            .as_secs(),
                    });
                }
            }
            Err(e) => {
                return Err(Error::Io(e));
            }
        }
    }

    Ok(files)
}

/// Read file contents
pub async fn read_file(path: &str) -> Result<String> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(Error::NotFound(format!("File not found: {}", path.display())));
    }

    if path.is_dir() {
        return Err(Error::InvalidInput(format!("Is a directory: {}", path.display())));
    }

    let content = async_fs::read_to_string(path).await
        .map_err(Error::Io)?;

    Ok(content)
}

/// Write file contents
pub async fn write_file(path: &str, content: &str) -> Result<()> {
    let path = Path::new(path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        async_fs::create_dir_all(parent).await
            .map_err(Error::Io)?;
    }

    async_fs::write(path, content)
        .await
        .map_err(Error::Io)?;

    Ok(())
}

/// Delete a file or directory
pub async fn delete(path: &str, recursive: bool) -> Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(Error::NotFound(format!("Path not found: {}", path.display())));
    }

    if path.is_dir() {
        if recursive {
            async_fs::remove_dir_all(path)
                .await
                .map_err(Error::Io)?;
        } else {
            async_fs::remove_dir(path)
                .await
                .map_err(Error::Io)?;
        }
    } else {
        async_fs::remove_file(path)
            .await
            .map_err(Error::Io)?;
    }

    Ok(())
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u64,
}
