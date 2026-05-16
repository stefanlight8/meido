use {
    crate::category::Category,
    anyhow::Result,
    async_stream::stream,
    futures_lite::Stream,
    std::{
        collections::{HashMap, HashSet},
        path::PathBuf,
    },
    tokio::fs,
};

#[derive(Debug, Clone)]
pub struct TrashEntry(pub Category, pub PathBuf);

#[derive(Debug, Clone)]
pub struct TrashBuffer {
    trash: HashMap<Category, Vec<PathBuf>>,
    pub can: HashSet<PathBuf>,
}

impl TrashBuffer {
    pub fn new() -> Self {
        Self {
            trash: HashMap::new(),
            can: HashSet::new(),
        }
    }

    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.trash.keys()
    }

    pub fn files(&self, category: Option<Category>) -> impl Iterator<Item = &PathBuf> + '_ {
        self.trash
            .iter()
            .filter(move |(k, _)| category.as_ref().map_or(true, |c| *k == c))
            .flat_map(|(_, v)| v.iter())
    }

    pub fn push(&mut self, entry: TrashEntry) {
        self.trash.entry(entry.0).or_default().push(entry.1);
    }

    pub fn contains(&self, path: &PathBuf) -> bool {
        self.can.contains(path)
    }

    pub fn trash_category(&mut self, category: &Category) {
        let buffer = self.trash.get(category).cloned().unwrap_or(vec![]);

        self.can.extend(buffer);
    }

    pub fn untrash_category(&mut self, category: &Category) {
        let buffer = self.trash.get(category).cloned().unwrap_or(vec![]);

        buffer.iter().for_each(|k| {
            self.can.remove(k);
        });
    }

    pub fn trash_file(&mut self, file: PathBuf) {
        self.can.insert(file);
    }

    pub fn untrash_file(&mut self, file: PathBuf) {
        self.can.remove(&file);
    }
}

// this is a temporary solution, in the future
// TODO: maybe own trash function
// because this doesn't work with async
// and probably just determining a trash folder
// and moving trash to it will be easier
// than this crate
pub async fn trash(paths: impl IntoIterator<Item = PathBuf>) -> Result<()> {
    trash::delete_all(paths)?;
    // will block tokio runtime, I hope that won't be a big problem for a quite time

    Ok(())
}

pub fn delete(paths: impl IntoIterator<Item = PathBuf>) -> impl Stream<Item = PathBuf> {
    stream! {
        for path in paths {
            let Ok(file_type) = fs::metadata(&path).await else {
                continue
            };

            if file_type.is_dir() {
                match fs::remove_dir_all(&path).await {
                    Ok(()) => yield path,
                    Err(err) => tracing::error!("failed to delete directory: {:?}", err),
                }
            } else if file_type.is_file() {
                match fs::remove_file(&path).await {
                    Ok(()) => yield path,
                    Err(err) => tracing::error!("failed to delete file: {:?}", err),
                }
            } else {
                tracing::warn!("unknown type: {}", path.display());
            }
        }
    }
}
