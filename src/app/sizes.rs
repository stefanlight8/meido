use std::{collections::HashMap, path::PathBuf};

use tokio::fs;

use crate::category::Category;

#[derive(Default)]
pub struct Sizes(pub HashMap<Category, u64>);

pub async fn dir_size(root: PathBuf) -> u64 {
    let mut stack = vec![root];
    let mut total = 0;

    while let Some(path) = stack.pop() {
        let mut dir = match tokio::fs::read_dir(&path).await {
            Ok(d) => d,
            Err(_) => continue,
        };

        while let Ok(Some(entry)) = dir.next_entry().await {
            let path = entry.path();

            match entry.file_type().await {
                Ok(ft) if ft.is_file() => {
                    if let Ok(meta) = entry.metadata().await {
                        total += meta.len();
                    }
                }

                Ok(ft) if ft.is_dir() => {
                    stack.push(path);
                }

                _ => {}
            }
        }
    }

    total
}

pub async fn get_sizes(paths: Vec<PathBuf>) -> u64 {
    let mut size = 0;

    for path in paths {
        match fs::metadata(&path).await {
            Ok(metadata) if metadata.is_file() => size += metadata.len(),
            Ok(metadata) if metadata.is_dir() => size += dir_size(path).await,
            _ => (),
        }
    }

    size
}
